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
        <nav class="flex text-white justify-center">
            <ul class="flex space-x-4 py-2">
                {links
                    .into_iter()
                    .map(|n| {
                        view! {
                            {move || {
                                if n.name == "Home" {
                                    view! {
                                        <A href=n.href>
                                            <p class="px-2 md:px-4 py-2 text-md md:text-xl font-bold hover:text-purple-400 text-purple-300">
                                                "Blessing Kodze"
                                            </p>
                                        </A>
                                    }
                                } else {
                                    view! {
                                        <A href=n.href>
                                            <p class="px-2 md:px-4 py-2 font-normal hover:underline hover:text-purple-400 underline-offset-4 decoration-2 text-purple-300">
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
