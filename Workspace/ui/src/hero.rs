use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { target: "_blank", href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { target: "_blank", href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
