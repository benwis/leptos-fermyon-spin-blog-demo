use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::add::AddPost;
use crate::pages::home::Home;
use crate::pages::notfound::NotFound;
use crate::pages::post::Post;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      <Stylesheet id="leptos" href="/pkg/leptos_blog_demo_finished.css"/>

      // sets the document title
      <Title text="Welcome to My Blog"/>

      // content for this welcome page
      <Router>
        <main>
          <Routes>
            <Route path="" view=Home/>
            <Route path="/blog/:slug" view=Post/>
            <Route path="/blog/add" view=AddPost/>
            <Route path="/*any" view=NotFound/>
          </Routes>
        </main>
      </Router>
    }
}
