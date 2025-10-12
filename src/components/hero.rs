use crate::pronunication::{pronounce, get_consonant_classes};
use dioxus::prelude::*;

#[component]
pub fn ThaiTranslator() -> Element {
    let mut thai = use_signal(|| "".to_string());
    let transliteration = pronounce(&thai.to_string());
    let consonant_classes = get_consonant_classes(&thai.to_string());
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen w-full px-4 py-8",
            div {
                class: "w-full max-w-2xl space-y-8",
                input {
                    r#type: "text",
                    placeholder: "Enter Thai text...",
                    class: "w-full text-3xl md:text-4xl lg:text-5xl text-center p-6 bg-gray-800 text-white border-2 border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 transition-colors",
                    oninput: move |e| thai.set(e.value()),
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl text-center text-gray-200 min-h-[3rem]",
                    "{transliteration}"
                }
                p {
                    class: "text-xl md:text-2xl lg:text-3xl text-center text-gray-400 min-h-[2rem] font-mono",
                    "{consonant_classes}"
                }
            }
        }
    }
}
