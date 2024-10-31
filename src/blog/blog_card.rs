use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(title: String, date: String, link: String, summary: String) -> impl IntoView {
    view! {
        <A href=link class="hover:text-purple-500">
            <div class="w-[400px] md:w-[600px] hover:border-b-2 border-purple-300 flex flex-col bg-inherit shadow-sm  p-4 md:p-5 ">
                <h3 class="text-lg font-bold dark:text-white">{title}</h3>
                <p class="mt-1 text-xs font-medium uppercase text-gray-500 ">
                    {date}
                </p>
            </div>
        </A>
    }
}
