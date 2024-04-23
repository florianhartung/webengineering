pub mod app;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod server;
#[cfg(feature = "ssr")]
pub mod app_state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
