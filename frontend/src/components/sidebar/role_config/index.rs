use super::role_list_tab::RoleListTab;
use common::llm::{inference::InferenceArgsForInput, role_list::RoleList};
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn RoleListContainer(
    ipv4: Signal<String>,
    role_list: RoleList,
    inference_args: Signal<InferenceArgsForInput>,
    set_inference_args: WriteSignal<InferenceArgsForInput>,
) -> impl IntoView {
    view! {
        <Tabs mount=Mount::Once>
            <Tab name="human" label="Human".into_view()>
                <RoleListTab
                    ipv4=ipv4
                    role_list=role_list.human.clone()
                    inference_args=inference_args
                    set_inference_args=set_inference_args
                />
            </Tab>
            <Tab name="computer" label="Computer".into_view()>
                <RoleListTab
                    ipv4=ipv4
                    role_list=role_list.computer.clone()
                    inference_args=inference_args
                    set_inference_args=set_inference_args
                />
            </Tab>
        </Tabs>
    }
}
