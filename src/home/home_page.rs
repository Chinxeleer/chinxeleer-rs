use leptos::*;

use crate::home::{home_hero::Hero, home_nav::Nav};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-full flex flex-col">
            <div>
                <Nav />
            </div>
            <div class="flex flex-1">
                <div class="flex mx-auto">
                    <Hero/>
                </div>
            </div>
            <div class="py-4 flex justify-center"><h1>"Footer"</h1></div>
        </div>
    }
}
