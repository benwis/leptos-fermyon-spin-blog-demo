use crate::functions::blog::get_post;
use leptos::*;
use leptos_router::{use_params, Params};

#[derive(Params, PartialEq, Clone)]
pub struct PathParams {
    pub slug: Option<String>,
}

#[component]
pub fn post() -> impl IntoView {
    let params = use_params::<PathParams>();

    let post = create_resource(
        move || params.get().map(|p| p.slug).unwrap().unwrap_or_default(),
        get_post,
    );
    view! {
      <h1>My Best Blog</h1>
      <Suspense fallback=|| {
          view! { <p>Loading...</p> }
      }>
        {move || {
            post.get()
                .map(|p| match p {
                    Ok(Some(p)) => view! { <section>{p}</section> }.into_view(),
                    Ok(None) => view! { <p>"Post Not Found"</p> }.into_view(),
                    Err(_) => view! { <p>"An Error Occured"</p> }.into_view(),
                })
        }}

      </Suspense>
    }
}
