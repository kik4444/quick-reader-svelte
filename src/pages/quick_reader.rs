use leptos::{
    html::{Textarea, P},
    *,
};

use crate::splitter::TextChunk;

#[derive(Debug)]
pub struct AppData {
    pub text: String,
    pub current_chunk: usize,
    pub chunks: Vec<TextChunk>,
    pub wpm: usize,
    pub chunk_size: usize,
}

#[component]
pub fn QuickReader() -> impl IntoView {
    let area = create_node_ref::<Textarea>();
    let display = create_node_ref::<P>();

    set_timeout(
        move || {
            // let chunks = splitter::split(&area().unwrap().value(), 0).unwrap();
            display().unwrap().set_inner_text("Changed");
        },
        std::time::Duration::from_secs(1),
    );

    view! {
      <textarea cols="29" rows="10" node_ref=area></textarea>
      <button>"click me"</button>
      <p node_ref=display>"To show"</p>
    }
}
