use leptos::{html::Span, *};

#[component]
pub fn ChatOther(ext_contents: Vec<HtmlElement<Span>>, profile: String) -> impl IntoView {
    view! {
        <div class="flex items-end">
            <div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-2 items-start">
                <div>
                    <span class="px-4 py-2 rounded-lg inline-block rounded-bl-none bg-gray-300 text-gray-600">
                        {ext_contents}
                    </span>
                </div>
            </div>
            <Profiles profile=profile order=1/>
        </div>
    }
}

#[component]
pub fn ChatSelf(ext_contents: Vec<HtmlElement<Span>>, profile: String) -> impl IntoView {
    view! {
        <div class="flex items-end justify-end">
            <div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-1 items-end">
                <div>
                    <span class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white">
                        {ext_contents}
                    </span>
                </div>
            </div>
            <Profiles profile=profile order=2/>
        </div>
    }
}

#[component]
pub fn Profiles(profile: String, order: u8) -> impl IntoView {
    let url = format!(
        "https://media.nostr.band/thumbs/{}/{}-picture-64",
        &profile[60..],
        profile
    );
    let class_list = format!("w-6 h-6 rounded-full order-{}", order);
    view! {
        <div>
            <img
                src=url
                alt="My profile"
                class=class_list
            />
        </div>
    }
}
