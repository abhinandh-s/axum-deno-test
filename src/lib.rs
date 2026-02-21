use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

use askama::Template;
use axum::response::Html;
use axum_js_fetch::App;
use wasm_bindgen::prelude::*;

mod post;

#[wasm_bindgen]
pub struct Website(App);

impl Default for Website {
    fn default() -> Self {
        let app = Router::new()
            .route("/", get(show_post));
        Self(App::new(app))
    }
}

#[wasm_bindgen]
impl Website {
    #[wasm_bindgen]
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        Self::default()
    }

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
        )
            .into_response(),
    }
}


