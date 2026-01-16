use axum::{
    response::Html,
    routing::get,
    Router,
};
use askama::Template; // This provides the .render() method
use axum_js_fetch::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp(App);

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen]
    pub fn new() -> Self {
        // We define the router inside new()
        let app = Router::new()
            .route("/", get(show_post));
        Self(App::new(app))
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

async fn show_post() -> Html<String> {
    let blog = PostTemplate {
        title: "Fast Rust Blog".into(),
        date: "2024-05-20".into(),
        content: "This is rendered via Askama in Wasm!".into(),
    };

    // Askama's .render() returns a Result<String, askama::Error>
    match blog.render() {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<h1>Render Error</h1><p>{}</p>", e)),
    }
}
