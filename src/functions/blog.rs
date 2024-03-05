use leptos::*;

/// Add a post to storage
#[server]
pub async fn add_post(slug: String, content: String) -> Result<(), ServerFnError> {
    let store = spin_sdk::key_value::Store::open_default()?;
   store.set_json(slug, &content).map_err(|e| ServerFnError::new(e.to_string()))?; 
    Ok(())
}

/// Get a post from storage
#[server]
pub async fn get_post(slug: String) -> Result<Option<String>, ServerFnError> {
    let store = spin_sdk::key_value::Store::open_default()?;

    let post = store
        .get_json(slug)
        .map_err(ServerFnError::new)?
        .unwrap_or_default();
    Ok(post)
}
