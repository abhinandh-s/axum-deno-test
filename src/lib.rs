use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header},
    response::{IntoResponse, Response, Redirect},
    routing::get,
    Router, Form,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use askama::Template;
use axum::response::Html;
use axum_js_fetch::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Website(App);

impl Default for Website {
    fn default() -> Self {
        let app = Router::new()
            .route("/", get(show_post))
            .route("/public", get(public))
            .route("/private", get(private))
            .route("/login", get(login_form).post(login))
            .route("/logout", get(logout));
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

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "JWT_SECRET";
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

// Extract Claims from Cookie instead of Authorization header
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract token from Cookie header
        let cookies = parts
            .headers
            .get(header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::InvalidToken)?;

        // Parse cookies and find "token"
        let token = cookies
            .split(';')
            .find_map(|cookie| {
                let mut parts = cookie.trim().splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some("token"), Some(value)) => Some(value.to_string()),
                    _ => None,
                }
            })
            .ok_or(AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = decode::<Claims>(&token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidToken => {
                // Redirect to login if token is invalid
                Redirect::to("/login").into_response()
            }
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, Html(WRONG_CREDENTIALS_HTML)).into_response()
            }
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials").into_response()
            }
            AuthError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error").into_response()
            }
        }
    }
}

const WRONG_CREDENTIALS_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Login Failed</title>
    <meta http-equiv="refresh" content="3;url=/login">
    <style>
        body {
            font-family: sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 0;
        }
        .container {
            background: white;
            padding: 40px;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
            text-align: center;
        }
        h1 { color: #dc2626; }
        p { color: #666; }
    </style>
</head>
<body>
    <div class="container">
        <h1>❌ Login Failed</h1>
        <p>Wrong username or password!</p>
        <p>Redirecting back to login page...</p>
    </div>
</body>
</html>
"#;

async fn public() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Public Page</title>
            <style>
                body {
                    font-family: sans-serif;
                    max-width: 600px;
                    margin: 50px auto;
                    padding: 20px;
                }
                .container {
                    background: #f0f0f0;
                    padding: 30px;
                    border-radius: 8px;
                }
                a {
                    color: #2563eb;
                    text-decoration: none;
                    display: inline-block;
                    margin: 5px 0;
                }
                a:hover {
                    text-decoration: underline;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Welcome to the Public Area 🌍</h1>
                <p>This page is accessible to everyone, no authentication required!</p>
                <hr>
                <p>
                    <a href="/login">→ Go to Login</a><br>
                    <a href="/private">→ Try Private Area (requires login)</a><br>
                    <a href="/">→ Go to Home</a>
                </p>
            </div>
        </body>
        </html>
    "#)
}

async fn private(claims: Claims) -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Private Page</title>
            <style>
                body {{
                    font-family: sans-serif;
                    max-width: 600px;
                    margin: 50px auto;
                    padding: 20px;
                }}
                .container {{
                    background: #dcfce7;
                    padding: 30px;
                    border-radius: 8px;
                    border: 2px solid #16a34a;
                }}
                .user-info {{
                    background: white;
                    padding: 15px;
                    border-radius: 5px;
                    margin: 15px 0;
                }}
                a {{
                    color: #2563eb;
                    text-decoration: none;
                    display: inline-block;
                    margin: 5px 0;
                }}
                a:hover {{
                    text-decoration: underline;
                }}
                .logout-btn {{
                    background: #dc2626;
                    color: white;
                    padding: 10px 20px;
                    border-radius: 5px;
                    display: inline-block;
                    margin-top: 10px;
                }}
                .logout-btn:hover {{
                    background: #b91c1c;
                    text-decoration: none;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🔒 Welcome to the Protected Area!</h1>
                <p>Congratulations! You are authenticated.</p>
                <div class="user-info">
                    <h3>Your Data:</h3>
                    <p><strong>Email:</strong> {}</p>
                    <p><strong>Company:</strong> {}</p>
                </div>
                <hr>
                <p>
                    <a href="/public">→ Go to Public Area</a><br>
                    <a href="/">→ Go to Home</a>
                </p>
                <a href="/logout" class="logout-btn">Logout</a>
            </div>
        </body>
        </html>
        "#,
        claims.sub, claims.company
    ))
}

async fn login(Form(form): Form<LoginForm>) -> Result<Response, AuthError> {
    if form.username != "foo" || form.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let now = (js_sys::Date::now() / 1000.0) as usize;
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: now + 300, // 5 minutes
    };

    let header = Header::default();
    let token = encode(&header, &claims, &KEYS.encoding).map_err(|e| {
        web_sys::console::log_1(&format!("JWT Encode Error: {:?}", e).into());
        AuthError::TokenCreation
    })?;

    // Set cookie and redirect to /private
    let cookie = format!("token={}; Path=/; Max-Age=300; SameSite=Lax", token);
    
    Ok((
        [(header::SET_COOKIE, cookie)],
        Redirect::to("/private"),
    )
        .into_response())
}

async fn logout() -> Response {
    // Clear the cookie and redirect to login
    let cookie = "token=; Path=/; Max-Age=0";
    
    (
        [(header::SET_COOKIE, cookie)],
        Redirect::to("/login"),
    )
        .into_response()
}

async fn login_form() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Login</title>
            <style>
                body {
                    font-family: sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin: 0;
                }
                .login-container {
                    background: white;
                    max-width: 400px;
                    width: 100%;
                    padding: 40px;
                    border-radius: 12px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.2);
                }
                h2 {
                    margin-top: 0;
                    color: #333;
                    text-align: center;
                }
                .form-group {
                    margin-bottom: 20px;
                }
                label {
                    display: block;
                    margin-bottom: 5px;
                    color: #555;
                    font-weight: 500;
                }
                input {
                    width: 100%;
                    padding: 12px;
                    border: 1px solid #ddd;
                    border-radius: 6px;
                    font-size: 14px;
                    box-sizing: border-box;
                }
                input:focus {
                    outline: none;
                    border-color: #667eea;
                }
                button {
                    width: 100%;
                    padding: 12px;
                    background: #667eea;
                    color: white;
                    border: none;
                    border-radius: 6px;
                    font-size: 16px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: background 0.3s;
                }
                button:hover {
                    background: #5568d3;
                }
                .hint {
                    margin-top: 20px;
                    padding: 15px;
                    background: #f8f9fa;
                    border-radius: 6px;
                    font-size: 13px;
                    color: #666;
                }
                .hint strong {
                    color: #333;
                }
                code {
                    background: #e9ecef;
                    padding: 2px 6px;
                    border-radius: 3px;
                    font-family: monospace;
                }
                .links {
                    margin-top: 20px;
                    text-align: center;
                }
                .links a {
                    color: #667eea;
                    text-decoration: none;
                    font-size: 14px;
                }
                .links a:hover {
                    text-decoration: underline;
                }
            </style>
        </head>
        <body>
            <div class="login-container">
                <h2>🔐 Login</h2>
                <form method="POST" action="/login">
                    <div class="form-group">
                        <label for="username">Username</label>
                        <input type="text" id="username" name="username" required autofocus />
                    </div>
                    <div class="form-group">
                        <label for="password">Password</label>
                        <input type="password" id="password" name="password" required />
                    </div>
                    <button type="submit">Login</button>
                </form>
                <div class="hint">
                    <strong>💡 Test Credentials:</strong><br>
                    Username: <code>foo</code><br>
                    Password: <code>bar</code>
                </div>
                <div class="links">
                    <a href="/public">→ Go to Public Area</a>
                </div>
            </div>
        </body>
        </html>
    "#)
}

// ------------------------------------------ //
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


