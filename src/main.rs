use wayland_client::{Connection, Dispatch, protocol::{wl_registry, wl_seat}, QueueHandle, WEnum};
use protocol::gamescope_input_method_manager::GamescopeInputMethodManager;
use protocol::gamescope_input_method;
use crate::protocol::gamescope_input_method_manager;

mod protocol;

struct AppData {
    seat: Option<wl_seat::WlSeat>,
    imm: Option<GamescopeInputMethodManager>,
    im: Option<gamescope_input_method::GamescopeInputMethod>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _conn: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { name, interface, version } = event {
            println!("[{}] {} (v{})", name, interface, version);
            match &interface[..] {
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, version, qh, ());
                }
                "gamescope_input_method_manager" => {
                    let imm = registry.bind::<GamescopeInputMethodManager, _, _>(name, version, qh, ());
                    state.imm = Some(imm);
                    //protocol::gamescope_input_method_manager::GamescopeInputMethodManager::
                }
                _ => {}
            }
            //if protocol::gamescope_input_method_manager::GamescopeInputMethodManager::
        } else {
            println!("{:?}", event);
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for AppData {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Name { name } = event {
            println!("wl_seat::name {:?}", name);
            state.seat = Some(seat.clone());

            if let Some(imm) = state.imm.clone() {
                let r = imm.create_input_method(&seat, qh, ());
                println!("im: {:?}", r);
                state.im = Some(r);
            }
        }
    }
}

impl Dispatch<GamescopeInputMethodManager, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &GamescopeInputMethodManager,
        _: gamescope_input_method_manager::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<gamescope_input_method::GamescopeInputMethod, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &gamescope_input_method::GamescopeInputMethod,
        event: gamescope_input_method::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("GamescopeInputMethod: {:?}", event);
        if let gamescope_input_method::Event::Done { serial } = event {
            if serial == 1 {
                if let Some(im) = &state.im {
                    im.set_string("Hello, world! こんにちは、世界！".to_string());
                    im.commit(serial);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let conn = Connection::connect_to_env().unwrap();
    let display = conn.display();

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    //event_queue.roundtrip(&mut AppData).unwrap();

    let mut state = AppData { seat: None, imm: None, im: None };
    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}
