use axum::{
    body::Body,
    extract::{Json, Query},
    routing::get,
    Router,
};
use axum_js_fetch::App;
use futures_lite::{stream, Stream};
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
        Self(ax_js::new(app))
    }

    #[wasm_bindgen]
    pub async fn serve(&self, req: web_sys::Request) -> web_sys::Response {
        self.0.oneshot(req).await
    }
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> String {
    format!("received query string: {:?}", params)
}

#[derive(Debug, Deserialize)]
struct TestStruct {
    hello: String,
}

async fn handler2(
    Json(payload): Json<TestStruct>,
) -> Body {
    let stream: futures_lite::stream::Repeat<Result<String, Infallible>> = stream::repeat(Ok(payload.hello));
    Body::from_stream(stream)
}

async fn handler3() -> Body {
    let chunks : Vec<Result<&'static str, Infallible>> = vec![Ok("Hello,"), Ok(" "), Ok("world!")];
    let stream = stream::iter(chunks);
    Body::from_stream(stream)
}
// -- new 
//
#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    title: String,
    content: String,
}

async fn show_post() -> Html<String> {
    let blog = PostTemplate {
        title: "Fast Rust Blog".into(),
        content: "This is rendered via Askama in Wasm!".into(),
    };

    // .render() generates the string, Html() sets the Content-Type
    match blog.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<h1>Error rendering template</h1>".to_string()),
    }
}
