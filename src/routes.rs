use crate::{about::about_page::AboutPage, blog::blog_page::BlogPage, home::home_page::HomePage};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn MainRoutes() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/chinxeleer-rust.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <main class="h-screen bg-rose-300">
            <Routes>
                <Route path="" view=HomePage />
                <Route path="about" view=AboutPage />
                <Route path="blog" view=BlogPage />
            </Routes>
        </main>
    }
}
