use wayland_client;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::{wl_registry, wl_seat};

pub mod __interfaces {
    use wayland_client::protocol::__interfaces::*;
    wayland_scanner::generate_interfaces!("./protocol/gamescope-input-method.xml");
}
use self::__interfaces::*;

wayland_scanner::generate_client_code!("./protocol/gamescope-input-method.xml");

#[derive(Clone)]
struct AppData {
    text: String,
}

struct AppState {
    running: bool,
    seat: Option<wl_seat::WlSeat>,
    imm: Option<gamescope_input_method_manager::GamescopeInputMethodManager>,
    im: Option<gamescope_input_method::GamescopeInputMethod>,
}

impl AppState {
    fn create_im(&mut self, qh: &QueueHandle<AppState>, data: &AppData) {
        if let (Some(seat), Some(imm)) = (&self.seat, &self.imm) {
            if self.im.is_none() {
                self.im = Some(imm.create_input_method(&seat, qh, data.clone()))
            }
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, AppData> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        data: &AppData,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            match &interface[..] {
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, version, qh, data.clone());
                }
                "gamescope_input_method_manager" => {
                    let imm = registry.bind::<gamescope_input_method_manager::GamescopeInputMethodManager, _, _>(name, version, qh, ());
                    state.imm = Some(imm);
                    state.create_im(qh, data);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_seat::WlSeat, AppData> for AppState {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        data: &AppData,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Name { name: _ } = event {
            state.seat = Some(seat.clone());
            state.create_im(qh, data);
        }
    }
}

impl Dispatch<gamescope_input_method_manager::GamescopeInputMethodManager, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &gamescope_input_method_manager::GamescopeInputMethodManager,
        _: gamescope_input_method_manager::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<gamescope_input_method::GamescopeInputMethod, AppData> for AppState {
    fn event(
        state: &mut Self,
        _: &gamescope_input_method::GamescopeInputMethod,
        event: gamescope_input_method::Event,
        data: &AppData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let (gamescope_input_method::Event::Done { serial }, Some(im)) = (event, &state.im) {
            if state.running {
                im.set_string(data.text.clone());
                im.commit(serial);
                // im.destroy();
                state.running = false;
            }
        }
    }
}

fn main() {
    let text = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    let conn = Connection::connect_to_env().expect("Failed to connect to Gamescope");

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qh, AppData { text });

    let mut state = AppState { running: true, seat: None, imm: None, im: None };
    while state.running {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
    event_queue.roundtrip(&mut state).unwrap();

    // I have to wait a bit or Steam will crash.
    std::thread::sleep(std::time::Duration::from_millis(100));
}
