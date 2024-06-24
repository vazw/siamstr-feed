// use crate::components::loading::LoadingIndi;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use std::str::FromStr;

use crate::components::chats::{ChatOther, ChatSelf};
use crate::helper::note_regex::transform_text_to_html;
use leptos::{
    html::{AnyElement, Div},
    *,
};
use nostr_sdk::prelude::*;

#[component]
pub fn Feed() -> impl IntoView {
    let app_keys = use_context::<RwSignal<Keys>>().expect("app key init");
    let event_text = use_context::<ReadSignal<Vec<Event>>>().expect("event read init");
    let event_text_list = use_context::<WriteSignal<Vec<Event>>>().expect("event write init");
    let pk = create_rw_signal(app_keys.get_untracked().public_key().to_hex());
    let (event_metadata, event_metadata_list) = create_signal(HashMap::<String, Metadata>::new());
    let container_ref = create_node_ref::<Div>();
    let added_events = create_signal(HashSet::<String>::new());
    let client = use_context::<RwSignal<Client>>()
        .expect("app key init")
        .get_untracked();

    spawn_local(async move {
        let _ = client.add_relay("ws://localhost:4869").await;
        let _ = client.add_relay("wss://relay.siamstr.com").await;
        let _ = client.add_relay("wss://relay.notoshi.win").await;
        let _ = client.add_relay("wss://bostr.lecturify.net").await;
        client.connect().await;
        // let filters_1 = Filter::new().kinds(vec![Kind::Metadata]);
        // let sub_id_1 = client.subscribe(vec![filters_1], None).await;
        let filters_2 = Filter::new()
            .kinds(vec![Kind::TextNote])
            .hashtag("siamstr")
            // .limit(1000)
            .since(Timestamp::now().sub(100_000));
        // let filters_3 = Filter::new().kinds(vec![Kind::Reaction]);
        let sub_id_2 = client.subscribe(vec![filters_2], None).await;
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
        let pk_ = pk.get_untracked();
        let events_ = event_text.get();
        let mut p_event: u64 = Timestamp::now().as_u64();
        for x in events_.iter() {
            if (x.kind == Kind::TextNote || x.kind == Kind::LongFormTextNote)
                && !added_events.0.with(|set| set.contains(&x.id.to_hex()))
            {
                let pk_i = x.pubkey.clone().to_hex();
                let ext_contents = transform_text_to_html(x.content());

                let message = if pk_i == pk_ {
                    view! {
                        <div class="chat-message">
                            <ChatSelf ext_contents=ext_contents profile=pk_i/>
                        </div>
                    }
                } else {
                    view! {
                        <div class="chat-message">
                            <ChatOther ext_contents=ext_contents profile=pk_i/>
                        </div>
                    }
                };
                let c_ref = container_ref.get().expect("messages");
                // .append_child(&message)
                if p_event > x.created_at().as_u64() {
                    c_ref.prepend_with_node_1(&message).unwrap();
                } else {
                    c_ref.append_child(&message).unwrap();
                };
                p_event = x.created_at.as_u64();
                added_events.1.update(|set| {
                    set.insert(x.id.to_hex());
                });
            }
        }
    });
    // create_effect(move |_| {
    //     added_events.0.get();
    //     if let Some(div) = container_ref.get() {
    //         div.set_scroll_top(div.scroll_height())
    //     }
    // });

    // create_effect(move |_| {
    //     added_events.0.get();
    //     for (npub, metadata_) in event_metadata.get().iter() {
    //         let profile_ = metadata_.picture.clone().unwrap_or("".to_string());
    //         let profile_node = create_node_ref::<Img>();
    //         let profile_view = view! { <div><Profiles profile=profile_node/></div> };
    //         profile_node
    //             .get_untracked()
    //             .expect("profile image")
    //             .set_src(&profile_);
    //         if let Some(el) = window()
    //             .document()
    //             .expect("window")
    //             .get_element_by_id(npub.as_str())
    //         {
    //             if !el.id().is_empty() {
    //                 if !el.replace_with_with_node_1(&profile_view).is_ok() {
    //                     log!("replace Failed");
    //                 } else {
    //                     log!("replaced profile");
    //                 }
    //             } else {
    //                 log!("replaced skip");
    //             };
    //         }
    //     }
    // });

    view! {
        <div class="block w-full max-w-full bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 justify-items-center">
            <div class="flex-1 p:2 sm:p-6 justify-between flex flex-col h-screen">

                <div class="flex items-center space-x-4 h-8 border-b-2 border-gray-200">
                    <div class="flex flex-col leading-tight">
                        <div class="text-2xl mt-1 flex items-center">
                            <span class="text-gray-700 dark:text-purple-600 mr-3">
                                "Nostr Feed"
                            </span>
                            <span class="text-lg text-gray-600 dark:text-purple-200">
                                "#siamstr"
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    id="messages"
                    class="flex flex-col space-y-4 p-3 overflow-y-auto scrollbar-thumb-blue scrollbar-thumb-rounded scrollbar-track-blue-lighter scrollbar-w-2 scrolling-touch h-screen"
                    node_ref=container_ref
                ></div>
                <div class="border-t-2 border-gray-200 px-4 pt-4 mb-2 sm:mb-0 z-1">
                    <div class="relative flex">
                        <input
                            type="text"
                            placeholder="เกิดอะไรขึ้น??   ส่งข้อความด้วยปลาอานนท์"
                            class="w-full focus:outline-none focus:placeholder-gray-400 text-gray-600 placeholder-gray-600 pl-12 bg-gray-200 rounded-md py-3"
                        />
                        <div class="absolute right-0 items-center inset-y-0">
                            <button
                                type="button"
                                class="inline-flex items-center justify-center rounded-full h-10 w-10 transition duration-500 ease-in-out text-gray-500 hover:bg-gray-300 focus:outline-none"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    class="h-6 w-6 text-gray-600"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"
                                    ></path>
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"
                                    ></path>
                                </svg>
                            </button>
                            <button
                                type="button"
                                class="inline-flex items-center justify-center rounded-full h-10 w-10 transition duration-500 ease-in-out text-gray-500 hover:bg-gray-300 focus:outline-none"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    class="h-6 w-6 text-gray-600"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                    ></path>
                                </svg>
                            </button>
                            <button
                                type="button"
                                class="inline-flex items-center justify-center rounded-lg px-4 py-3 transition duration-500 ease-in-out text-white bg-blue-500 hover:bg-blue-400 focus:outline-none"
                            >
                                <span class="font-bold">"Send"</span>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                    class="h-6 w-6 ml-2 transform rotate-90"
                                >
                                    <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// <Suspense fallback={move || {
//     view! {
//                                 <LoadingIndi/>
//     }
// }}>
//     // handles the error from the resource
//     <ErrorBoundary fallback={|_| {
//         view! {
//             <p>"เกิดข้อผิดพลาด"</p>
//         }
//     }}>
//         // add chat here!!!
//         {move || {
//             let pk_ = pk.get_untracked();
//             let events_ = event_text.get();
//             let metadata_ = event_metadata.get().clone();
//             events_
//                 .iter()
//                 .map(move |x| {
//                     let pk_i = x.pubkey.clone().to_hex();
//                     let profile = if let Some(prof_pic) = metadata_
//                         .get(&pk_i.clone())
//                     {
//                         if let Some(pics) = prof_pic.picture.clone() {
//                             pics
//                         } else {
//                             "".to_string()
//                         }
//                     } else {
//                         "".to_string()
//                     };
//                     let ext_contents = transform_text_to_html(x.content()).into_view();
//                     if pk_i == pk_ {
//                         view! {
//                             <div class="chat-message">
//                                 <div class="flex items-end justify-end">
//                                     <div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-1 items-end">
//                                         <div>
//                                             <span class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white ">
//                                                 {ext_contents}
//                                             </span>
//                                         </div>
//                                     </div>
//                                     <img
//                                         src={profile}
//                                         alt="My profile"
//                                         class="w-6 h-6 rounded-full order-2"
//                                     />
//                                 </div>
//                             </div>
//                         }
//                     } else {
//                         view! {
//                             <div class="chat-message">
//                                 <div class="flex items-end">
//                                     <div class="flex flex-col space-y-2 text-xs max-w-xs mx-2 order-2 items-start">
//                                         <div>
//                                             <span class="px-4 py-2 rounded-lg inline-block rounded-bl-none bg-gray-300 text-gray-600">
//                                                 {ext_contents}
//                                             </span>
//                                         </div>
//                                     </div>
//                                     <img
//                                         src={profile}
//                                         alt="My profile"
//                                         class="w-6 h-6 rounded-full order-1"
//                                     />
//                                 </div>
//                             </div>
//                         }
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         }}

//     </ErrorBoundary>
// </Suspense>
