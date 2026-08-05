#![allow(non_snake_case)]

mod app;
mod calc;
mod css;
mod i18n;
mod models;
mod storage;

fn main() {
    dioxus::launch(app::App);
}
