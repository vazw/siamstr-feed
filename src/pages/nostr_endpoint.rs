use std::collections::HashSet;
use std::str::FromStr;

use leptos::html::{Div, Span};
use leptos::logging::log;
use leptos::*;
use leptos_router::*;
use nostr_sdk::prelude::*;

use crate::components::chats::Profiles;
use crate::helper::note_regex::transform_text_to_html;

#[derive(Params, PartialEq)]
struct NostrAddress {
    id: String,
}
#[component]
pub fn NostrEP() -> impl IntoView {
    let address = use_params::<NostrAddress>();
    let event_text = use_context::<ReadSignal<Vec<Event>>>().expect("event read init");
    let event_text_list = use_context::<WriteSignal<Vec<Event>>>().expect("event write init");
    let event_id = address.with_untracked(|param| {
        param
            .as_ref()
            .to_owned()
            .map(|x| x.id.to_string())
            .unwrap_or_default()
    });
    let event_id = if event_id.starts_with("nostr:") {
        event_id.replace("nostr:", "")
    } else {
        event_id
    };
    let event_info = Nip19::from_bech32(&event_id).expect("valid nostr address");
    let added_events = create_signal(HashSet::<String>::new());
    let container_ref = create_node_ref::<Div>();
    let client = use_context::<RwSignal<Client>>()
        .expect("app key init")
        .get_untracked();
    let mut candidate: Option<Event> = None;
    match &event_info {
        Nip19::Event(eve) => {
            let idx = eve.event_id.to_hex();
            for event in event_text.get_untracked().into_iter() {
                if event.id.to_hex() == idx {
                    candidate = Some(event.clone());
                    break;
                }
            }
        }
        Nip19::EventId(evid) => {
            for event in event_text.get_untracked().into_iter() {
                if event.id.to_hex() == evid.to_hex() {
                    candidate = Some(event.clone());
                    break;
                }
            }
        }
        _ => (),
    };
    spawn_local(async move {
        let _ = client.add_relay("ws://localhost:4869").await;
        let _ = client.add_relay("wss://relay.siamstr.com").await;
        let _ = client.add_relay("wss://relay.notoshi.win").await;
        // let _ = client.add_relay("wss://bostr.lecturify.net").await;
        client.connect().await;
        let filter_1 = match event_info.clone() {
            Nip19::Pubkey(npub) => Filter::new().pubkey(npub).kind(Kind::Metadata).limit(10),
            Nip19::Profile(ref profiles) => Filter::new()
                .pubkey(profiles.clone().public_key)
                .kind(Kind::Metadata)
                .limit(10),
            Nip19::Event(ref eve) => Filter::new()
                .event(eve.clone().event_id)
                .kinds([Kind::TextNote, Kind::LongFormTextNote])
                .limit(1000),
            Nip19::EventId(evid) => Filter::new()
                .event(evid)
                .kinds([Kind::TextNote, Kind::LongFormTextNote])
                .limit(1000),
            Nip19::Coordinate(ref s) => Filter::new()
                .pubkey(s.public_key)
                .kind(Kind::Metadata)
                .limit(10),
            _ => Filter::new().limit(1),
        };
        // let filters_3 = Filter::new().kinds(vec![Kind::Reaction]);

        let sub_id_2 = client.subscribe(vec![filter_1], None).await;
        // let sub_id_3 = client.subscribe(vec![filters_3], None).await;
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                // if subscription_id == sub_id_1 && event.kind == Kind::Metadata {
                //     // handle the event
                //     let json_str = format!(r#"{}"#, event.content());
                //     let data: Metadata = serde_json::from_str(&json_str).unwrap_or(Metadata::new());
                //     evm.insert(event.pubkey.to_hex(), data);
                //     event_metadata_list.set(evm.clone());
                // } else
                if subscription_id == sub_id_2
                // && (event.kind == Kind::TextNote || event.kind == Kind::LongFormTextNote)
                {
                    let mut evt = event_text.get_untracked().to_vec();
                    evt.push(*event);
                    event_text_list.set(evt);
                }
            }
        }
    });
    create_effect(move |_| {
        let events_ = event_text.get();
        for x in events_.iter() {
            if !added_events.0.with(|set| set.contains(&x.id.to_hex())) {
                let (npub, ext_contents) = if let Some(event_c) = &candidate {
                    let npub = event_c.pubkey.to_hex();
                    let ext_contents = transform_text_to_html(event_c.content());
                    (npub, ext_contents)
                } else {
                    let npub = x.pubkey.to_hex();
                    let ext_contents = transform_text_to_html(x.content());
                    (npub, ext_contents)
                };
                let message = view! {
                    <div class="flex items-end">
                        <div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-1 items-start">
                            <div>
                                <span class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white">
                                    {ext_contents}
                                </span>
                            </div>
                        </div>
                        <Profiles profile=npub order=2/>
                    </div>
                };
                let c_ref = container_ref.get().expect("messages");
                // .append_child(&message)
                c_ref.append_child(&message).unwrap();
                added_events.1.update(|set| {
                    set.insert(x.id.to_hex());
                });
            }
        }
    });
    view! {
        <div node_ref=container_ref class="flex flex-col space-y-4 p-3 overflow-y-auto scrollbar-thumb-blue scrollbar-thumb-rounded scrollbar-track-blue-lighter scrollbar-w-2 scrolling-touch h-screen">
        </div>
    }
}
