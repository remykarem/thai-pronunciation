use crate::pronunication::pronounce;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let mut thai = use_signal(|| "".to_string());
    let transliteration = pronounce(&thai.to_string());
    rsx! {
        input {
            type: "text",
            placeholder: "First Name…",

            // Update the first_name signal on text input
            oninput: move |e| thai.set(e.value()),
        }
        p { "{transliteration}" }
    }
}
