use leptos::*;
use leptos_router::*;

#[component]
pub fn HomeFooter() -> impl IntoView {
    view! {
        <div class="py-4 flex justify-center">
            <A href="https://github.com/Chinxeleer" class="font-light hover:text-purple-400 text-purple-300">"github"</A>
        </div>
    }
}
