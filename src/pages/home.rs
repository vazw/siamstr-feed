 use leptos::*;
 use crate::pages::feed::Feed;

#[component]
pub fn Home() -> impl IntoView {
	view! {
		<div>
			<Feed/>
		</div>
	}    
}
