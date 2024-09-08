pub mod about;
pub mod app;
pub mod blog;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod home;
pub mod routes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
