use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use nostr_sdk::prelude::*;
use std::str::FromStr;

// Modules
mod components;
mod helper;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::nostr_endpoint::NostrEP;
use crate::pages::not_found::NotFound;
const NSEC_ANON: &str = "nsec1nuq7e2w37t89apaupmxj5ylg027mnfmtzv4d2rcmjmt7fwjhhlmsxdmlq7";

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let app_keys = Keys::from_str(NSEC_ANON).expect("ปลาอานนท์");
    let app_keys_signal = create_rw_signal(app_keys);
    let client = Client::new(app_keys_signal.get_untracked());
    let client_signal = create_rw_signal(client);
    let (event_text, event_text_list) = create_signal(Vec::<Event>::new());
    provide_context(app_keys_signal);
    provide_context(client_signal);
    provide_context(event_text);
    provide_context(event_text_list);

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="dark"/>

        // sets the document title
        <Title text="Siamstr Feed"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/*" view=NotFound/>
                <Route path="/nostr/:id" view=NostrEP/>
            </Routes>
        </Router>
    }
}
