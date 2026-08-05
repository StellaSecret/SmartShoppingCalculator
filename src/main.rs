#![allow(non_snake_case)]

mod app;
mod css;
mod i18n;
mod storage;

fn main() {
    dioxus::launch(app::App);
}
