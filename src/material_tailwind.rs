use leptos::*;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] class: Option<MaybeSignal<String>>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
      <button
        class=move || {
            format!(
                "middle none center rounded-lg py-3 px-6 font-sans text-xs font-bold uppercase text-white shadow-md transition-all hover:shadow-lg focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none {}",
                class.as_ref().map(| c | c()).unwrap_or_default()
            )
        }

        data-ripple-light="true"
        disabled=move || disabled.map(|d| d()).unwrap_or_default()
      >
        {children()}
      </button>
    }
}

#[component]
pub fn Textarea(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<html::Textarea>>,
    #[prop(optional, into)] readonly: Option<MaybeSignal<bool>>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
    #[prop(optional, into)] label: Option<MaybeSignal<String>>,
    #[prop(optional, into)] placeholder: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let reference = create_node_ref::<html::Textarea>();

    let view = view! {
      <div class="w-96">
        <div class="relative w-full min-w-[200px]">
          <textarea
            node_ref=reference
            class="peer h-full min-h-[100px] w-full resize-none rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-pink-500 focus:border-t-transparent focus:outline-0 disabled:resize-none disabled:border-0 disabled:bg-blue-gray-50"
            placeholder=move || placeholder.as_ref().map(|p| p()).unwrap_or_default()
            readonly=move || readonly.map(|r| r()).unwrap_or_default()
            disabled=move || disabled.map(|d| d()).unwrap_or_default()
          >
            {children()}
          </textarea>
          <label class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-pink-500 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-pink-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-pink-500 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
            {label}
          </label>
        </div>
      </div>
    };

    if let Some(r) = node_ref {
        reference.get_untracked().expect("to exist").node_ref(r);
    }

    view
}
