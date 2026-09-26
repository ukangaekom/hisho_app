use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut connection = use_signal(|| "terminal");
    let mut docs_step = use_signal(|| "connect");
    let (docs_label, docs_heading, docs_description, docs_command) = match docs_step() {
        "inspect" => (
            "02 / INSPECT",
            "Resolve the contract.",
            "Provide an address and network. Hisho reads contract state, token metadata, balances, and protocol context.",
            "hisho inspect 0xa0b8…eB48 --chain ethereum",
        ),
        "verify" => (
            "03 / VERIFY",
            "Check the evidence.",
            "Review source, metadata, ownership, and proxy signals together before relying on a contract.",
            "hisho verify token 0xa0b8…eB48 --chain ethereum",
        ),
        "execute" => (
            "04 / EXECUTE",
            "Act with authorization.",
            "Review the proposed intent and authorize execution explicitly. Reads stay separate from writes.",
            "hisho execute --review <intent>",
        ),
        _ => (
            "01 / CONNECT",
            "Start with intent.",
            "Bring Hisho into your agent workflow through a persistent terminal session or a compatible MCP client.",
            "hisho",
        ),
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Title { "Hisho | On-chain intelligence runtime" }

        div { class: "site-shell",
            header { class: "topbar",
                a {
                    class: "wordmark",
                    href: "#top",
                    aria_label: "Hisho home",
                    span { class: "mark", "h" }
                    span { "hisho" }
                    span { class: "wordmark-period", "." }
                }

                nav { class: "main-nav", aria_label: "Main navigation",
                    a { href: "#runtime", "Runtime" }
                    a { href: "#compatibility", "Compatibility" }
                    a { href: "#docs", "Docs" }
                }

                a { class: "nav-cta", href: "#docs",
                    "Get started"
                    span { "↗" }
                }
            }

            main { id: "top",
                section { class: "hero section-wrap",
                    div { class: "hero-copy",
                        div { class: "eyebrow",
                            span { class: "pulse-dot" }
                            "ON-CHAIN INTELLIGENCE RUNTIME"
                        }
                        h1 {
                            "Read the chain."
                            br {}
                            span { class: "muted-title", "Know what’s real." }
                            br {}
                            "Then act."
                        }
                        p { class: "hero-lede",
                            "Contract intelligence, verification, and execution in one trusted runtime. From a plain-English question to a decision you can stand behind."
                        }
                        div { class: "hero-actions",
                            a {
                                class: "button button-primary",
                                href: "#runtime",
                                "Explore the runtime"
                                span { "↘" }
                            }
                            a { class: "text-link", href: "#compatibility",
                                "See agent compatibility"
                            }
                        }
                        div { class: "hero-note",
                            span { class: "note-mark", "↳" }
                            span { "Built for people who need the answer, not another RPC tutorial." }
                        }
                    }

                    div { class: "console-frame",
                        div { class: "console-topline",
                            div { class: "window-dots",
                                span {}
                                span {}
                                span {}
                            }
                            span { class: "console-path", "hisho / inspect" }
                            span { class: "console-live",
                                span { class: "pulse-dot" }
                                "SAMPLE OUTPUT"
                            }
                        }
                        div { class: "console-content",
                            div { class: "console-meta",
                                span { "ETHEREUM MAINNET" }
                                span { class: "meta-block", "BLOCK 20,184,392" }
                            }
                            div { class: "prompt-line",
                                span { class: "prompt-symbol", "›" }
                                span { "Is this token contract verified and safe to interact with?" }
                            }
                            div { class: "contract-card",
                                div { class: "contract-heading",
                                    div { class: "token-emblem", "U" }
                                    div { class: "token-title",
                                        strong { "USD Coin" }
                                        span { "USDC · ERC-20" }
                                    }
                                    span { class: "verified-badge", "✓ VERIFIED" }
                                }
                                div { class: "address-line",
                                    span { "CONTRACT" }
                                    code { "0xa0b8…eB48" }
                                    button {
                                        class: "copy-mark",
                                        title: "Copy contract address",
                                        "⧉"
                                    }
                                }
                                div { class: "console-divider" }
                                div { class: "result-row",
                                    span { class: "result-check", "✓" }
                                    div { class: "result-copy",
                                        strong { "Source code" }
                                        span { "Verified on Etherscan" }
                                    }
                                    span { class: "result-state", "MATCH" }
                                }
                                div { class: "result-row",
                                    span { class: "result-check", "✓" }
                                    div { class: "result-copy",
                                        strong { "Token metadata" }
                                        span { "Name, symbol & decimals agree" }
                                    }
                                    span { class: "result-state", "MATCH" }
                                }
                                div { class: "result-row",
                                    span { class: "result-check", "✓" }
                                    div { class: "result-copy",
                                        strong { "Proxy analysis" }
                                        span { "Implementation resolved" }
                                    }
                                    span { class: "result-state", "CLEAR" }
                                }
                            }
                            div { class: "console-footer",
                                span { class: "footer-spark", "✳" }
                                span { "3 checks shown" }
                                span { class: "footer-time", "illustrative" }
                            }
                        }
                        div { class: "console-rail",
                            span { "HISHO RUNTIME" }
                            span { "001 / 003" }
                        }
                    }
                }

                section { class: "signal-strip",
                    div { class: "signal-inner",
                        div { class: "signal-label", "THE SHIFT IS ALREADY HERE" }
                        div { class: "signal-stat",
                            strong { "90%" }
                            span { "use AI coding agents weekly" }
                        }
                        div { class: "signal-stat",
                            strong { "68%" }
                            span { "use them every day" }
                        }
                        div { class: "signal-source",
                            "JetBrains developer survey · 15,000+ respondents · 2026"
                        }
                    }
                }

                section { id: "runtime", class: "runtime-section section-wrap",
                    div { class: "section-kicker", "01 / THE RUNTIME" }
                    div { class: "section-heading-row",
                        h2 { "From scattered calls\nto trusted action." }
                        p {
                            "The work is not just fetching blockchain data. It’s knowing what to trust, what changed, and what should happen next."
                        }
                    }
                    div { class: "flow-grid",
                        article { class: "flow-step",
                            span { class: "flow-index", "01" }
                            div { class: "flow-icon read-icon", "⌕" }
                            h3 { "Read" }
                            p {
                                "Inspect contracts, tokens, balances, and activity directly from the network."
                            }
                            span { class: "flow-code", "state · metadata · events" }
                        }
                        article { class: "flow-step flow-featured",
                            span { class: "flow-index", "02" }
                            div { class: "flow-icon verify-icon", "✓" }
                            h3 { "Verify" }
                            p {
                                "Cross-check source, metadata, ownership, and protocol signals before they become assumptions."
                            }
                            span { class: "flow-code", "evidence · confidence · context" }
                        }
                        article { class: "flow-step",
                            span { class: "flow-index", "03" }
                            div { class: "flow-icon act-icon", "↗" }
                            h3 { "Act" }
                            p {
                                "Prepare and execute on-chain actions only when authorized, with clear intent and safety checks."
                            }
                            span { class: "flow-code", "intent · authorization · execution" }
                        }
                    }
                    div { class: "runtime-footnote",
                        span { "✳" }
                        "Models reason. Hisho handles the chain."
                    }
                }

                section { class: "bridge-section",
                    div { class: "bridge-inner section-wrap",
                        div { class: "bridge-copy",
                            div { class: "section-kicker", "02 / ONE RUNTIME, ANY AGENT" }
                            h2 { "Your agent’s way in.\nHisho’s way of knowing." }
                            p {
                                "Keep the model layer flexible. Switch providers for cost, latency, or trust requirements without changing the blockchain runtime underneath."
                            }
                            div {
                                class: "mode-switch",
                                role: "tablist",
                                aria_label: "Connection mode",
                                button {
                                    class: if connection() == "terminal" { "mode-tab selected" } else { "mode-tab" },
                                    role: "tab",
                                    aria_selected: connection() == "terminal",
                                    onclick: move |_| connection.set("terminal"),
                                    span { "⌘" }
                                    "Terminal"
                                }
                                button {
                                    class: if connection() == "mcp" { "mode-tab selected" } else { "mode-tab" },
                                    role: "tab",
                                    aria_selected: connection() == "mcp",
                                    onclick: move |_| connection.set("mcp"),
                                    span { "◈" }
                                    "MCP"
                                }
                            }
                            div { class: "mode-description",
                                if connection() == "terminal" {
                                    "Persistent local sessions give terminal-capable agents a direct conversation with Hisho."
                                } else {
                                    "A standardized MCP interface connects compatible agents to Hisho tools and verification workflows."
                                }
                            }
                        }
                        div { class: "code-window",
                            div { class: "code-header",
                                span { class: "code-indicator" }
                                span {
                                    if connection() == "terminal" {
                                        "TERMINAL SESSION"
                                    } else {
                                        "MCP CONFIGURATION"
                                    }
                                }
                                span { class: "code-header-right", "EVM · READY" }
                            }
                            if connection() == "terminal" {
                                div { class: "code-body",
                                    p { class: "code-comment",
                                        "# Ask Hisho to verify before you interact"
                                    }
                                    p { class: "code-command",
                                        span { "$ " }
                                        "hisho"
                                    }
                                    p { class: "code-command",
                                        span { "  › " }
                                        "inspect 0xa0b8…eB48"
                                    }
                                    p { class: "code-output", "✓ Source verified" }
                                    p { class: "code-output", "✓ Token metadata matched" }
                                    p { class: "code-output", "→ Ready for your next instruction" }
                                    div { class: "code-cursor" }
                                }
                                div { class: "code-caption",
                                    span { "LOCAL SESSION" }
                                    span { "No provider lock-in" }
                                }
                            } else {
                                div { class: "code-body",
                                    p { class: "code-comment", "// Connect any MCP-compatible agent" }
                                    p { class: "code-key",
                                        "transport"
                                        span { ": " }
                                        "stdio"
                                    }
                                    p { class: "code-key",
                                        "command"
                                        span { ": " }
                                        "hisho"
                                    }
                                    p { class: "code-key",
                                        "tools"
                                        span { ": [" }
                                    }
                                    p { class: "code-key code-indent", "inspect_contract," }
                                    p { class: "code-key code-indent", "verify_token," }
                                    p { class: "code-key code-indent", "prepare_transaction" }
                                    p { class: "code-key", "]" }
                                }
                                div { class: "code-caption",
                                    span { "MODEL AGNOSTIC" }
                                    span { "Standardized MCP interface" }
                                }
                            }
                        }
                    }
                }

                section { id: "compatibility", class: "compat-section section-wrap",
                    div { class: "compat-heading",
                        div { class: "section-kicker", "03 / AGENT-AGNOSTIC ACCESS" }
                        h2 { "Fits into the tools\nyou already use." }
                        p {
                            "No dedicated integration for every agent. If it can hold a terminal session or speak MCP, it can work with Hisho."
                        }
                    }
                    div { class: "compat-content",
                        div { class: "compat-topline",
                            span { "BUILT FOR YOUR WORKFLOW" }
                            span { "28+ ENVIRONMENTS" }
                        }
                        div { class: "tool-grid",
                            span { "Claude Code" }
                            span { "Codex CLI" }
                            span { "Cursor" }
                            span { "Gemini CLI" }
                            span { "Windsurf" }
                            span { "Cline" }
                            span { "Roo Code" }
                            span { "Aider" }
                            span { "OpenHands" }
                            span { "Goose" }
                            span { "Warp" }
                            span { "Kiro" }
                            span { "VS Code / Copilot" }
                            span { "Copilot CLI" }
                            span { "JetBrains Junie" }
                            span { "Zed" }
                            span { "Continue" }
                            span { "Augment Code" }
                            span { "Qodo" }
                            span { "Trae" }
                            span { "OpenCode" }
                            span { "Antigravity" }
                            span { "Amazon Q" }
                            span { "Tabnine" }
                            span { "PearAI" }
                            span { "Void" }
                            span { "Junie CLI" }
                            span { "+ MCP clients" }
                        }
                    }
                }

                section { class: "docs-section", id: "docs",
                    div { class: "docs-inner section-wrap",
                        div { class: "docs-intro",
                            div { class: "section-kicker", "04 / START HERE" }
                            h2 { "A clearer path\nto the chain." }
                            p {
                                "Bring Hisho into the environment where you already build. Start with a question, then let the runtime assemble the evidence."
                            }
                        }
                        div { class: "docs-panel",
                            div {
                                class: "docs-panel-nav",
                                role: "tablist",
                                aria_label: "Quickstart steps",
                                span { class: "docs-nav-title", "QUICKSTART" }
                                button {
                                    class: if docs_step() == "connect" { "docs-nav-item active" } else { "docs-nav-item" },
                                    role: "tab",
                                    aria_selected: docs_step() == "connect",
                                    onclick: move |_| docs_step.set("connect"),
                                    "01  Connect"
                                }
                                button {
                                    class: if docs_step() == "inspect" { "docs-nav-item active" } else { "docs-nav-item" },
                                    role: "tab",
                                    aria_selected: docs_step() == "inspect",
                                    onclick: move |_| docs_step.set("inspect"),
                                    "02  Inspect"
                                }
                                button {
                                    class: if docs_step() == "verify" { "docs-nav-item active" } else { "docs-nav-item" },
                                    role: "tab",
                                    aria_selected: docs_step() == "verify",
                                    onclick: move |_| docs_step.set("verify"),
                                    "03  Verify"
                                }
                                button {
                                    class: if docs_step() == "execute" { "docs-nav-item active" } else { "docs-nav-item" },
                                    role: "tab",
                                    aria_selected: docs_step() == "execute",
                                    onclick: move |_| docs_step.set("execute"),
                                    "04  Execute"
                                }
                            }
                            div { class: "docs-panel-main",
                                div { class: "docs-panel-top",
                                    span { "{docs_label}" }
                                    span { class: "docs-version", "EVM RUNTIME" }
                                }
                                h3 { "{docs_heading}" }
                                p { "{docs_description}" }
                                div { class: "quick-command",
                                    span { class: "command-prompt", "$" }
                                    code { "{docs_command}" }
                                    span { class: "command-live", "READY" }
                                }
                                div { class: "docs-tags",
                                    span { "Read-only by default" }
                                    span { "Explicit authorization" }
                                    span { "Auditable intent" }
                                }
                                a { class: "docs-link", href: "#runtime",
                                    "Explore runtime capabilities"
                                    span { "↗" }
                                }
                            }
                        }
                    }
                }

                section { class: "closing-band",
                    div { class: "closing-inner section-wrap",
                        div { class: "closing-mark", "h" }
                        div { class: "closing-copy",
                            span { class: "section-kicker", "THE NEXT INTERFACE TO THE CHAIN" }
                            h2 { "Make verification\npart of the action." }
                        }
                        a { class: "button button-light", href: "#docs",
                            "Build with Hisho"
                            span { "↗" }
                        }
                    }
                }
            }

            footer { class: "site-footer section-wrap",
                a { class: "footer-brand", href: "#top", "hisho." }
                span { "Intelligence. Verification. Execution." }
                span { class: "footer-proof", "TOP 100 · MOONSHOTS BUILD WITH GEMINI X PRIZE" }
                a { class: "back-top", href: "#top", "BACK TO TOP ↑" }
            }
        }
    }
}
