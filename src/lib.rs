use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use askama::Template; 
use axum_js_fetch::App;
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

    /// This function handles the bridge between Deno/JS and Axum.
    /// wasm_bindgen_futures (added to Cargo.toml) makes this 'async' possible.
    #[wasm_bindgen]
    pub async fn serve(&self, req: web_sys::Request) -> web_sys::Response {
        self.0.oneshot(req).await
    }
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    title: String,
    date: String,
    content: String,
}

async fn show_post() -> impl IntoResponse {
    let blog = PostTemplate {
        title: "Fast Rust Blog".into(),
        date: "2026-01-16".into(),
        content: "This is rendered via Askama 0.15 in Wasm!".into(),
    };

    match blog.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template Render Error: {}", e),
        ).into_response(),
    }
}
