use dioxus::prelude::*;
use ui::FinancialDashboard;

#[component]
pub fn Home() -> Element {
    rsx! {
        FinancialDashboard {}
    }
}
