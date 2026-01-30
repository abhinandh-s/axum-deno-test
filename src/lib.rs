use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use axum_js_fetch::App;
use wasm_bindgen::prelude::*;

mod post;

#[wasm_bindgen]
pub struct MyApp(App);

impl Default for MyApp {
    fn default() -> Self {
        let app = Router::new().route("/", get(show_post));
        Self(App::new(app))
    }
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen]
    pub fn new() -> Self {
        Self::default()
    }

    /// This function handles the bridge between Deno/JS and Axum.
    /// [`wasm_bindgen_futures`] makes this 'async' possible.
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
    let mut blog = PostTemplate {
        title: "Fast Rust Blog".into(),
        date: "2026-01-16".into(),
        content: "This is rendered via Askama 0.15 in Wasm!".into(),
    };
    let articles = post::get_all_articles_sorted();
    if let Some(f) = articles.first() {
        blog = PostTemplate {
            title: f.matter.title.clone(),
            date: f.matter.published_at.clone(),
            content: f.content.clone(),
        }
    }

    match blog.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template Render Error: {}", e),
        )
            .into_response(),
    }
}
