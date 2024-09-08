use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
    <div class="flex flex-col justify-center align-center my-auto text-center rounded-lg p-[20px] w-[400px] md:w-[600px]">
        <h1 class="text-4xl font-extrabold">Blessing Kodze</h1>
        <p className="text-sm font-bold mb-4">
              "Much like a hero in \"Solo Leveling,\" I rise to new
              challenges, leveling up my skills with each line of code written,
              each problem solved. When I’m not forging new paths in the world
              of programming, you can find me immersed in the rich narratives of
              anime, drawing strength and inspiration from the Holy Bible
              sharpening my strategic mind through gaming."
            </p>
        <img class="rounded-full bg-blend-screen" src="/gif/gog.gif" alt="Blessing"/>


    </div>
    }
}
