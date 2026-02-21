#[derive(Clone)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub tag: &'static str,
    pub excerpt: &'static str,
    pub content: &'static str,
}

pub fn all_posts() -> Vec<Post> {
    vec![
        Post {
            slug: "hello-catppuccin",
            title: "Hello, Catppuccin!",
            date: "2025-01-15",
            tag: "Design",
            excerpt: "Discovering the pastel perfection of the Catppuccin colour palette and why it's become my favourite theme.",
            content: r#"
<p>
  Catppuccin is a community-driven pastel theme that comes in four delicious flavours:
  <strong>Latte</strong> (light), <strong>Frappé</strong>, <strong>Macchiato</strong>, and
  <strong>Mocha</strong> (darkest). Each flavour shares the same hue relationships but varies in
  lightness and saturation, creating a cohesive family of themes.
</p>
<h2>Why Catppuccin?</h2>
<p>
  Unlike many dark themes that feel harsh or neon, Catppuccin's muted pastels are easy on the
  eyes for long coding sessions while still being visually distinctive. The palette was designed
  with accessibility and aesthetics in mind.
</p>
<h2>The Four Flavours</h2>
<ul>
  <li><strong>Latte</strong> — The light theme. Warm creamy backgrounds with deep text.</li>
  <li><strong>Frappé</strong> — A medium-dark theme with cool blue-grey tones.</li>
  <li><strong>Macchiato</strong> — Slightly darker than Frappé, more saturated accents.</li>
  <li><strong>Mocha</strong> — The darkest flavour. Rich, deep, and cosy.</li>
</ul>
<p>
  Use the theme switcher in the header to try them all right now!
</p>
"#,
        },
        Post {
            slug: "rust-wasm-without-tokio",
            title: "Rust + WASM Without Tokio",
            date: "2025-02-01",
            tag: "Rust",
            excerpt: "How to use Axum's routing logic in a WebAssembly context where Tokio's async runtime isn't available.",
            content: r#"
<p>
  When compiling Rust to WebAssembly, you cannot use Tokio — the browser already provides its
  own event loop. This blog itself demonstrates the pattern: Axum's handler concepts are used
  for organisation, but the actual dispatching is a synchronous match statement in
  <code>router.rs</code>.
</p>
<h2>The Pattern</h2>
<p>
  Instead of <code>axum::serve(listener, app)</code>, we:
</p>
<ol>
  <li>Register a <code>click</code> listener on <code>document</code>.</li>
  <li>Intercept navigational clicks, build a path string.</li>
  <li>Call a sync <code>Router::handle(&amp;path)</code> → <code>String</code>.</li>
  <li>Inject the returned HTML into <code>#app</code>.</li>
</ol>
<p>
  Handlers are plain Rust functions returning <code>String</code>. No futures, no executors.
</p>
<h2>wasm-bindgen Async</h2>
<p>
  If you need actual async work (e.g. <code>fetch</code>), use
  <code>wasm_bindgen_futures::spawn_local</code> which delegates to the browser's microtask
  queue — no Tokio required.
</p>
"#,
        },
        Post {
            slug: "tailwind-v4-css-first",
            title: "Tailwind CSS v4: The CSS-First Revolution",
            date: "2025-02-20",
            tag: "CSS",
            excerpt: "Tailwind CSS v4 drops the JavaScript config file in favour of native CSS custom properties and @theme blocks.",
            content: r#"
<p>
  Tailwind CSS v4 is a complete rewrite. Gone is <code>tailwind.config.js</code>. Instead, you
  configure your design tokens directly in CSS using the new <code>@theme</code> at-rule.
</p>
<h2>@theme Blocks</h2>
<pre><code>@theme {
  --color-primary: oklch(65% 0.15 260);
  --font-sans: "Inter", sans-serif;
}</code></pre>
<p>
  Any CSS custom property declared inside <code>@theme</code> automatically becomes a Tailwind
  utility class — <code>text-primary</code>, <code>bg-primary</code>, etc.
</p>
<h2>Catppuccin + Tailwind v4</h2>
<p>
  For this blog, each Catppuccin flavour is expressed as a set of custom properties on
  <code>[data-theme="latte"]</code>, <code>[data-theme="frappe"]</code>, etc. Tailwind v4's
  CSS-native approach means switching themes is just toggling a single attribute — no JavaScript
  class manipulation needed.
</p>
"#,
        },
        Post {
            slug: "building-a-spa-in-rust",
            title: "Building a SPA in Pure Rust",
            date: "2025-03-10",
            tag: "WebAssembly",
            excerpt: "A deep dive into architecting a single-page application using Rust compiled to WebAssembly, with client-side routing and DOM manipulation.",
            content: r#"
<p>
  Single-page applications in Rust/WASM are still a frontier. Frameworks like Leptos and Yew
  exist, but sometimes you want to understand the primitives. This blog is intentionally minimal:
  no framework, just <code>web-sys</code> and <code>wasm-bindgen</code>.
</p>
<h2>Client-Side Routing</h2>
<p>
  We use the URL hash (<code>#/post/slug</code>) as our routing mechanism. The
  <code>hashchange</code> / <code>popstate</code> events trigger a re-render. This avoids any
  server-side routing configuration.
</p>
<h2>Rendering Strategy</h2>
<p>
  All pages are rendered to HTML strings on the Rust side and injected via
  <code>set_inner_html</code>. This is the simplest approach and works great for a content-heavy
  blog. For highly interactive UIs, a virtual DOM approach (Yew/Leptos) would be preferable.
</p>
<h2>State Management</h2>
<p>
  Theme state is stored in <code>localStorage</code> via <code>web-sys</code> and restored on
  startup. Global mutable state in WASM is managed with <code>thread_local!</code> and
  <code>RefCell</code> (WASM is single-threaded).
</p>
"#,
        },
    ]
}

pub fn find_post(slug: &str) -> Option<Post> {
    all_posts().into_iter().find(|p| p.slug == slug)
}
