use axum::{
    body::Body,
    extract::{Json, Query},
    routing::get,
    response::Html, // Added this
    Router,
};
// You must import the trait for .render() to work!
use askama::Template; 
use axum_js_fetch::App;
use futures_lite::{stream};
use serde::Deserialize;
use std::{collections::HashMap, convert::Infallible};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp(App);

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen]
    pub fn new() -> Self {
        let app = Router::new()
            .route("/", get(show_post));
        Self(App::new(app))
    }

    #[wasm_bindgen]
    pub async fn serve(&self, req: web_sys::Request) -> web_sys::Response {
        self.0.oneshot(req).await
    }
}

// Your template struct must match the variables in your HTML
#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    title: String,
    date: String,    // Added this because your HTML uses {{ date }}
    content: String,
}

async fn show_post() -> Html<String> {
    let blog = PostTemplate {
        title: "Fast Rust Blog".into(),
        date: "2024-05-20".into(), // Added data here
        content: "This is rendered via Askama in Wasm!".into(),
    };

    match blog.render() {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<h1>Render Error</h1><p>{}</p>", e)),
    }
}
