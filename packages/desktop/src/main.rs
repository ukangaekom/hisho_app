use dioxus::prelude::*;
use ui::BootScreen;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_booted = use_signal(|| false);

    rsx! {
        if !is_booted() {
            BootScreen {
                app_name: "HISHO".to_string(),
                tagline: "Desktop Financial OS".to_string(),
                on_ready: move |_| {
                    *is_booted.write() = true;
                }
            }
        } else {
            div {
                style: "position: fixed; inset: 0; width: 100vw; height: 100vh; background-color: #ffffff; margin: 0; padding: 0;"
            }
        }
    }
}
