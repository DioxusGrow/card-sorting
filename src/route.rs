use crate::ui::pages::error_page::Err404;
use crate::ui::pages::home::Home;
use dioxus::core::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
