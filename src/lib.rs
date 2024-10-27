pub mod app;
pub mod server_functions;
pub mod blog;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod home;
pub mod projects;
pub mod resume;
pub mod routes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
