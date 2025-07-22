use leptos::prelude::*;

mod app;
use app::*;
pub mod bevy_app;

pub fn main() {
    console_error_panic_hook::set_once();

    let path = web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default();

    if path.contains("bevy_window") {
        mount_to_body(move || {
            view! {
                <BevyApp/>
            }
        })
    } else {
        mount_to_body(move || {
            view! {
                <App/>
            }
        })
    };
}
