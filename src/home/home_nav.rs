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
            name: "About",
            href: "about",
        },
    ];
    view! {
        <nav class="flex text-white justify-center">
            <ul class="flex space-x-4 py-2">
                {links
                    .into_iter()
                    .map(|n| view! { <A href=n.href><p class="px-4 py-2 font-bold text-purple-600">{n.name}</p></A> })
                    .collect_view()}
            </ul>
        </nav>
    }
}
