use leptos::*;

use crate::blog::blog_card::BlogCard;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <main class="flex flex-col justify-center align-center space-y-2 md:px-20">
            <h1 class="font-bold text-xl text-center">"Blog"</h1>
            <div class="min-h-full mx-auto space-y-4 overflow-scroll">
                <BlogCard title="Card 1" date="1 January 2003" />
                <BlogCard title="Card 2" date="4 December 2015"/>
                <BlogCard title="Card 3" date="6 September 2018"/>
                <BlogCard title="Card 4" date="19 July 2023"/>
            </div>

        </main>
    }
}
