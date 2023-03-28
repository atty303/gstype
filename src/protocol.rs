use wayland_client;
use wayland_client::protocol::*;

pub mod __interfaces {
    use wayland_client::protocol::__interfaces::*;
    wayland_scanner::generate_interfaces!("./protocol/gamescope-input-method.xml");
}
use self::__interfaces::*;

wayland_scanner::generate_client_code!("./protocol/gamescope-input-method.xml");
