use leptos::*;
use leptos_icons::*;
use leptos_router::A;

#[component]
pub fn HomeHero() -> impl IntoView {
    view! {

    <main class="flex flex-col items-center mt-10">
        <div class="max-w-5xl mx-auto mt-4 space-y-10">
            //<!-- Social Links Section -->
            <div class="flex justify-center flex-row mt-4 space-x-4">
                <A href="https://github.com/Chinxeleer" class="hover:text-rose-300">
                    <Icon icon=icondata::AiGithubFilled width="2rem" height="2rem" />
                </A>
                <A href="https://www.linkedin.com/in/blessing-kodze-a86302212/" class="hover:text-rose-300">
                    <Icon icon=icondata::AiLinkedinFilled width="2rem" height="2rem" />
                </A>
            </div>
            //<!-- About Section -->
            <div class="flex flex-col justify-center items-start text-justify text-sm lg:text-base mt-10 max-w-xl space-y-6">
                <h2 class="text-2xl font-bold">About</h2>
                <p class="px-1">
                    "I am a student at the University of the Witwatersrand studying Computer Science and Applied Mathematics.
                    I am a developer and a mathematician who is passionate about technology and programming."
                </p>
                <p class="px-1">
                    "I am interested in all things Rust 🦀 and Web Development. I build everything in Rust, even this website was built using Rust.
                    I've built terminal apps, games, APIs, and solved Advent of Code problems."
                </p>
                <p>"Linux Enthusiast 🐧. I use Arch by the way. I use Linux as my daily driver, growing in understaning the underlying "</p>
            </div>
        </div>
    </main>

        }
}
