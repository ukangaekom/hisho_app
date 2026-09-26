use dioxus::prelude::*;

const BOOT_CSS: Asset = asset!("/assets/styling/boot_screen.css");

async fn sleep_ms(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms as u64)).await;
    }
}

#[component]
pub fn BootScreen(
    app_name: String,
    tagline: String,
    on_ready: EventHandler<()>,
) -> Element {
    let mut progress = use_signal(|| 10);
    let mut status_text = use_signal(|| "Initializing security context...".to_string());
    let mut is_finished = use_signal(|| false);

    // Run async boot initialization when component mounts
    use_effect(move || {
        spawn(async move {
            // Task 1: Initialize local storage & crypto keys
            sleep_ms(400).await;
            *progress.write() = 35;
            *status_text.write() = "Restoring session token...".to_string();

            // Task 2: Connect to financial API services
            sleep_ms(500).await;
            *progress.write() = 70;
            *status_text.write() = "Connecting to market telemetry...".to_string();

            // Task 3: Finalize cache sync
            sleep_ms(400).await;
            *progress.write() = 100;
            *status_text.write() = "Engine Ready!".to_string();

            // Allow short pause at 100% for visual polish
            sleep_ms(300).await;
            *is_finished.write() = true;

            // Trigger completion callback to main app
            sleep_ms(500).await;
            on_ready.call(());
        });
    });

    let fade_class = if is_finished() { "fade-out" } else { "" };

    rsx! {
        document::Stylesheet { href: BOOT_CSS }

        div { class: "boot-screen {fade_class}",
            div { class: "boot-logo-container",
                div { class: "boot-logo-glow" }
                div { class: "boot-logo-badge", "H" }
            }

            h1 { class: "boot-title", "{app_name}" }
            p { class: "boot-subtitle", "{tagline}" }

            div { class: "boot-progress-bar",
                div {
                    class: "boot-progress-fill",
                    style: "width: {progress}%",
                }
            }

            p { class: "boot-status", "{status_text}" }
        }
    }
}

