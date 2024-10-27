use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::MainRoutes,
    server_functions::posts::get_posts,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let posts = create_resource(|| (), |_| async move { get_posts().await });
    provide_context(posts);
    view! {
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
           <MainRoutes/>
        </Router>
    }
}
