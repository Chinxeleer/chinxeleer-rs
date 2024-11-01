use leptos::*;
use leptos_image::Image;

#[component]
pub fn HomeHero() -> impl IntoView {
    view! {

    <main class="flex flex-col text-pink-100 items-center mt-4">
        <div class="max-w-5xl mx-auto space-y-10">
            //<!-- Social Links Section -->
            <div class="flex rounded-full justify-center">

            // TODO:Add an image of me
            <Image
            src="pictures/profile.jpg"
            blur=true
            width=150
            height=150
            quality=100
            priority=true
            class="rounded-full anti-aliasing"
                />

            </div>
            //<!-- About Section -->
            <div class="flex flex-col items-start text-justify text-sm lg:text-base mt-10 max-w-xl space-y-6">
                <h2 class="text-2xl font-bold">About</h2>
                <p class="px-1">
                    "I am a student at the University of the Witwatersrand studying Computer Science and Applied Mathematics.
                    I am a developer and a mathematician who loves programming."
                </p>
                <p class="px-1">
                    "I am interested in all things Rust 🦀 and Web Development. I build everything in Rust, even this website was built using Rust.
                    I've built terminal apps, games, APIs, and solved Advent of Code problems."
                </p>
                <p>"Linux Enthusiast 🐧. I use Arch by the way. I use Linux as my daily driver so as to grow in understaning the underlying infrastructure of the Linux Kernel."</p>
                <p>"As one of the leaders at the Every Nation Campus Wits society, I invest my time in discipling young students. Challenging their world view and ultimately raising them to be leaders too."</p>
            </div>
        </div>
    </main>

        }
}
