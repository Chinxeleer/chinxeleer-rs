use leptos::*;
use leptos_router::*;

struct Link<'a> {
    name: &'a str,
    href: &'a str,
}

#[component]
pub fn Nav() -> impl IntoView {
    let links = vec![
        Link {
            name: "Home",
            href: "/",
        },
        Link {
            name: "Blog",
            href: "blog",
        },
        Link {
            name: "Resume",
            href: "resume",
        },
        Link {
            name: "Projects",
            href: "projects",
        },
    ];
    view! {
        <nav class="flex text-slate-100 justify-center">
            <ul class="flex py-4 items-center">
                {links
                    .into_iter()
                    .map(|n| {
                        view! {
                            {move || {
                                if n.name == "Home" {
                                    view! {
                                        <A href=n.href>
                                            <h1 class="px-2 md:px-4 py-2 text-base antialiased md:text-xl font-bold hover:text-rose-300">
                                            "Blessing Kodze"
                                            </h1>
                                        </A>
                                    }
                                } else {
                                    view! {
                                        <A href=n.href>
                                            <p class="px-2 md:px-4 py-2 font-normal hover:underline hover:text-rose-300 underline-offset-4 decoration-2 ">
                                                {n.name}
                                            </p>
                                        </A>
                                    }
                                }
                            }}
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}
