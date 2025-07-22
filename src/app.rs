use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::bevy_app::core::init::init_bevy_app;

#[component]
pub fn BevyApp() -> impl IntoView {
    match window().document().unwrap().body() {
        Some(body) => {
            let _ = body.style().set_property("background", "none transparent");
        }
        None => (),
    };

    view! {
        <BevyCanvas
            init=move || {
                init_bevy_app()
            }
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-screen h-screen flex items-center justify-center overflow-hidden">
            <div class="w-full h-full">
                <iframe class="m-0 p-0 w-full h-full" src="/bevy_window"/>
            </div>
        </main>
    }
}
