use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(title: String, date: String, link: String) -> impl IntoView {
    view! {
        <A href=link class="hover:text-purple-800">
            <div class="w-[400px] md:w-[600px] flex flex-col bg-inherit rounded-lg shadow-sm p-4 md:p-5 ">
                <h3 class="text-lg font-bold text-slate-800">{title}</h3>
                <p class="mt-1 text-xs uppercase justify-end">
                    {date}
                </p>
            </div>
        </A>
    }
}
