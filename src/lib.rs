use askama::Template;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use axum_js_fetch::App;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

// --- Entry Point for Deno ---

#[wasm_bindgen]
pub struct MyApp(App);

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen]
    pub fn new() -> Self {
        let app = Router::new()
            .route("/", get(start_handler))
            .route("/{lang}/index.html", get(index_handler))
            .route("/{lang}/greet-me.html", get(greeting_handler));
        
        Self(App::new(app))
    }

    #[wasm_bindgen]
    pub async fn serve(&self, req: web_sys::Request) -> web_sys::Response {
        self.0.oneshot(req).await
    }
}

// --- Logic & Types ---

#[derive(Default, Debug, Clone, Copy, PartialEq, Deserialize, strum::Display)]
#[allow(non_camel_case_types)]
enum Lang {
    #[default]
    en,
    de,
    fr,
}

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Render Error")]
    Render(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Template)]
        #[template(path = "error.html")]
        struct ErrorTmpl {
            lang: Lang,
            message: String,
        }

        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let tmpl = ErrorTmpl {
            lang: Lang::default(),
            message: self.to_string(),
        };

        match tmpl.render() {
            Ok(html) => (status, Html(html)).into_response(),
            Err(_) => (status, "Critical Rendering Failure").into_response(),
        }
    }
}

// --- Handlers ---

async fn start_handler() -> Redirect {
    Redirect::temporary("/en/index.html")
}

#[derive(Debug, Deserialize)]
struct IndexHandlerQuery {
    #[serde(default)]
    name: String,
}

async fn index_handler(
    Path((lang,)): Path<(Lang,)>,
    Query(query): Query<IndexHandlerQuery>,
) -> Result<impl IntoResponse, AppError> {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexTmpl {
        lang: Lang,
        name: String,
    }

    let template = IndexTmpl {
        lang,
        name: query.name,
    };

    let html = template.render()?;
    Ok(Html(html))
}

#[derive(Debug, Deserialize)]
struct GreetingHandlerQuery {
    name: String,
}

async fn greeting_handler(
    Path((lang,)): Path<(Lang,)>,
    Query(query): Query<GreetingHandlerQuery>,
) -> Result<impl IntoResponse, AppError> {
    #[derive(Template)]
    #[template(path = "greet.html")]
    struct GreetTmpl {
        lang: Lang,
        name: String,
    }

    let template = GreetTmpl {
        lang,
        name: query.name,
    };

    let html = template.render()?;
    Ok(Html(html))
}
