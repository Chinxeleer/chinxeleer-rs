use std::collections::HashMap;

use leptos::*;

use crate::{
    blog::blog_card::BlogCard,
    server_functions::posts::{Post, PostType},
};

#[component]
pub fn BlogList() -> impl IntoView {
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");

    view! {
    <div class="mx-auto">
        <h1 class="font-bold text-xl text-center">"Blog"</h1>
            <div class="min-h-full mx-auto space-y-4 overflow-scroll">
                <BlogCard title="Card 1" link="1" date="1 January 2003" />
                <BlogCard title="Card 2" link="card 2" date="4 December 2015"/>
                <BlogCard title="Card 3" link="card 3" date="6 September 2018"/>
                <BlogCard title="Card 4" link="card 4" date="19 July 2023"/>
            </div>
    </div>}
}
