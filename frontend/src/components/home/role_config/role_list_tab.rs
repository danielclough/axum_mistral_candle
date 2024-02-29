use leptonic::{
    collapsible::{Collapsibles, OnOpen},
    stack::Stack,
    Size,
};
use common::llm::{inference::InferenceArgsForInput, role_list::RoleListEntry};
use leptos::*;

use crate::components::home::role_config::role_list_item::RoleListItem;

#[component]
pub fn RoleListTab(
    ipv4: Signal<String>,
    role_list: Vec<RoleListEntry>,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
    set_refresh_token: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Stack spacing=Size::Em(0.6)>
                <h2>"Current: " {inference_args.get().role}</h2>
                <For each=move || role_list.clone() key=|list| list.clone() let:item>
                    <RoleListItem
                        ipv4=ipv4
                        item=item
                        inference_args=inference_args
                        set_inference_args=set_inference_args
                        set_refresh_token=set_refresh_token
                    />
                </For>
            </Stack>
        </Collapsibles>
    }
}