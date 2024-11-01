use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(title: String, date: String, link: String) -> impl IntoView {
    view! {
        <A href=link class="hover:text-purple-500">
            <div class="w-[400px] md:w-[600px] hover:bg-purple-200 flex flex-col bg-inherit shadow-sm p-4 md:p-5 ">
                <h3 class="text-lg font-bold text-gray-800 dark:text-white">{title}</h3>
                <p class="mt-1 text-xs uppercase justify-end">
                    {date}
                </p>
            </div>
        </A>
    }
}
