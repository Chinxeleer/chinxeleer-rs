use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(title: String, date: String, link: String) -> impl IntoView {
    view! {
        <A href=link class="hover:text-purple-500">
            <div class="w-[400px] md:w-[600px] hover:border-b-2 border-purple-300 rounded-lg flex flex-col bg-inherit shadow-sm  p-4 md:p-5 ">
                <h3 class="text-xl font-bold text-gray-800 dark:text-white">{title}</h3>
                <p class="mt-1 text-xs font-medium uppercase text-gray-500 dark:text-neutral-500">
                    {date}
                </p>
                <p class="mt-2 text-gray-500 dark:text-neutral-400">
                    "Some quick example text to build on the card title and make up the bulk of the card's content."
                </p>
            </div>
        </A>
    }
}
