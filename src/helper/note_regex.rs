use std::ops::Deref;

use lazy_static::lazy_static;
use leptos::{html::Span, *};
use regex::Regex;

const YOUTUBE_URL_TEXT_REGEX: &str = r#"(?x)
    (?:https?://)?               # Optional scheme
    (?:www\.)?                   # Optional www
    (?:youtube\.com|youtu\.be)   # Domain
    (?:                          # Group for different URL formats
        /watch\?v=               # watch?v=VIDEO_ID
        |/embed/                 # /embed/VIDEO_ID
        |/shorts/                # /shorts/VIDEO_ID
        |/live/                  # /live/VIDEO_ID
        |/                       # /VIDEO_ID (youtu.be/VIDEO_ID)
    )
    ([\w\-]{11})                 # Capture group for video ID (11 characters)
"#;
// REGEX FROM @snort/system
lazy_static! {
    static ref HASHTAG_REGEX: Regex =
        Regex::new(r#"(#[^\s!@#$%^&*()=+.\/,\[{\]};:'\"?><]+)"#).unwrap();
    // static ref TAG_REF_REGEX: Regex = Regex::new(r"(#\[\d+\])").unwrap();
    static ref FILE_EXTENSION_REGEX: Regex = Regex::new(r"\.([\w]{1,7})$").unwrap();
    static ref INVOICE_REGEX: Regex = Regex::new(r"(lnbc\w+)").unwrap();
    static ref CASHU_REGEX: Regex =
        Regex::new(r"(cashuA[A-Za-z0-9_-]{0,10000}={0,3})").unwrap();
    static ref MENTION_NOSTR_ENTITY_REGEX: Regex =
        Regex::new(r"@n(pub|profile|event|ote|addr)1[acdefghjklmnpqrstuvwxyz023456789]+").unwrap();
    static ref MARKDOWN_CODE_REGEX: Regex = Regex::new(r"(?ms)(```.+?```)").unwrap();
    static ref YOUTUBE_URL_REGEX: Regex =
        Regex::new(r"\bhttps?://(www\.)?(youtube\.com|youtu\.?be)/.+\b").unwrap();
    static ref YOUTUBE_EXTRACT_URL_REGEX: Regex =
        Regex::new(YOUTUBE_URL_TEXT_REGEX).unwrap();
    static ref IMAGE_URL_REGEX: Regex =
        Regex::new(r#"(?i)\bhttps?://\S+\.(jpg|jpeg|png|gif|bmp|webp)\b"#).unwrap();
    static ref VIDEO_URL_REGEX: Regex =
        Regex::new(r#"\bhttps?://\S+\.(mp4|wav|mov)\b"#).unwrap();
    static ref OTHER_URL_REGEX: Regex = Regex::new(r#"(?i)\bhttps?://[^\s]+\b"#).unwrap();
    static ref NOSTR_REGEX: Regex =
        Regex::new(r"(?x)(?:nostr:)?n(pub|profile|event|ote|addr)1[acdefghjklmnpqrstuvwxyz023456789]+").unwrap();
}

pub fn transform_text_to_html(text: &str) -> Vec<HtmlElement<Span>> {
    let patterns: Vec<(&Regex, Box<dyn Fn(&regex::Captures) -> HtmlElement<_>>)> = vec![
        (
            HASHTAG_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="hashtag">{&caps[0].to_owned()}</span> },
            ),
        ),
        (
            IMAGE_URL_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="img"><img src={&caps[0].to_owned()}></img></span> },
            ),
        ),
        (
            INVOICE_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="invoice">{&caps[0].to_owned()}</span> },
            ),
        ),
        (
            CASHU_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="cashu">{&caps[0].to_owned()}</span> },
            ),
        ),
        (
            MENTION_NOSTR_ENTITY_REGEX.deref(),
            Box::new(|caps: &regex::Captures| {
                let url = format!("/nostr/{}", &caps[0].to_owned());
                view! { <span class="nostr-entity"><iframe src={url} frameborder="0"></iframe></span> }
            }),
        ),
        (
            MARKDOWN_CODE_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="markdown"><code>{&caps[0].to_owned()}</code></span> },
            ),
        ),
        (
            YOUTUBE_URL_REGEX.deref(),
            Box::new(|caps: &regex::Captures| {
                let texts = &caps[0].to_owned();
                let regex = YOUTUBE_EXTRACT_URL_REGEX.deref();
                let url = regex
                    .captures(&texts)
                    .map(|mat| format!("https://youtube.com/embed/{}", &mat[1]));
                view! { <span class="video"><iframe src={url} title="YouTube video player" frameborder="0" allowfullscreen></iframe></span> }
            }),
        ),
        (
            VIDEO_URL_REGEX.deref(),
            Box::new(
                |caps: &regex::Captures| view! { <span class="video"><video controls><source src={&caps[0].to_owned()}/></video></span> },
            ),
        ),
        (
            OTHER_URL_REGEX.deref(),
            Box::new(|caps: &regex::Captures| {
                let url = &caps[0].to_owned();
                if IMAGE_URL_REGEX.is_match(url)
                    || YOUTUBE_URL_REGEX.is_match(url)
                    || VIDEO_URL_REGEX.is_match(url)
                {
                    return view! {<span/>};
                };
                view! { <span class="link"><a href={url} target="_blank">{url}</a></span> }
            }),
        ),
        (
            NOSTR_REGEX.deref(),
            Box::new(|caps: &regex::Captures| {
                let url = format!("/nostr/{}", &caps[0].to_owned());
                view! { <span class="nostr-entity"><iframe src={url} frameborder="0"></iframe></span> }
            }),
        ),
        // (
        //     FILE_EXTENSION_REGEX.deref(),
        //     Box::new(
        //         |caps: &regex::Captures| view! { <span class="file-extension"><img src={&caps[0].to_owned()}></img></span> },
        //     ),
        // ),
        // (
        //     TAG_REF_REGEX.deref(),
        //     Box::new(
        //         |caps: &regex::Captures| view! { <span class="tag-ref">{&caps[0].to_owned()}</span> },
        //     ),
        // ),
    ];
    let texts: Vec<Vec<&str>> = text.split("\n").map(|x| x.split(" ").collect()).collect();
    let mut result = Vec::new();
    let mut last_end: usize;

    for text_n in texts.into_iter() {
        for text in text_n.iter() {
            last_end = 0;
            for (regex, replacer) in &patterns {
                for caps in regex.captures_iter(&text) {
                    if let Some(mat) = caps.get(0) {
                        if mat.start() > last_end {
                            result.push(
                                view! { <span>{&text[last_end..mat.start()].to_owned()}</span> },
                            );
                        }
                        result.push(replacer(&caps));
                        last_end = mat.end();
                    }
                }
            }
            if last_end < text.len() {
                result.push(view! { <span>{&text[last_end..].to_owned()}</span> });
            }
            result.push(view! { <span>" "</span> })
        }
        result.push(view! {
            <span>
                <br/>
            </span>
        })
    }
    result
}
