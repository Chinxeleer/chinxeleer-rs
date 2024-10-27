use std::collections::HashMap;

use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_params_map;

use crate::{
    error_template::{AppError, ErrorTemplate},
    server_functions::posts::{Post, PostType},
};

#[component]
pub fn BlogView() -> impl IntoView {
    let params = use_params_map();
    let post_query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());
    let post_title = post_query();
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");
    view! {
        <div class="bg-slate-900 text-purple-200">
            <div class="max-w-5xl mx-auto">
                <h1 class="text-4xl font-bold">{post_title}</h1>
            </div>
            <div>
            <Transition
            fallback = move || view! { <div>Loading</div> }
            >
                <div>
                {move ||{
                            posts().map(|posts| {
                                match posts{
                                    Ok(posts) => {
                                        let post = posts
                                .get(&PostType::Blog)
                                .expect("Unable to read the right post_type")
                                .iter()
                                .find(|&p| p.post_metadata.create_href() == post_query());
                            if let Some(post) = post {
                                view! {
                                    <Title text=post.post_metadata.title.clone()/>
                                    <Meta
                                        name="description"
                                        content=post.post_metadata.description.clone()
                                    />
                                    <div inner_html=post.post_content.clone()></div>
                                }
                                    .into_view()
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);

                                view! {  <ErrorTemplate outside_errors/> }
                                    .into_view()
                            }},
                                    Err(_) => todo!(),


                            }})
                    }

                }
                </div>
            </Transition>
            </div>
        </div>
    }
}
