use crate::functions::blog::AddPost;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn AddPost() -> impl IntoView {
    let add_post = create_server_action::<AddPost>();
    view! {
      <h1>"Add Post"</h1>
      <ActionForm action=add_post>
        <input name="slug"/>
        <input name="content"/>
        <button type="submit">Submit</button>
      </ActionForm>
    }
}
