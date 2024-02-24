use crate::components::sidebar::model_config::model_list_chip::ModelListChip;
use crate::components::sidebar::model_config::model_list_item::ModelListItem;
use common::llm::model_list::ModelDLList;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn ModelListGrid(
    model_list_signal: ReadSignal<ModelDLList>,
    tags_all: ReadSignal<Vec<String>>,
    tags_enabled: ReadSignal<Vec<String>>,
    set_tags_enabled: WriteSignal<Vec<String>>,
    all_enabled: ReadSignal<bool>,
    ipv4: Signal<String>,
    template_current: ReadSignal<String>,
    repo_id: ReadSignal<String>,
    quantized_current: ReadSignal<bool>,
    quantized: bool,
    gpu_type: Signal<String>,
) -> impl IntoView {
    view! {
        <Grid style="padding:1rem;" spacing=Size::Em(0.6)>
            <H1 style="width:100%;padding:0.25rem;text-align:center;box-shadow:2px 2px 8px #000;">
                "Model List"
            </H1>

            <Row>
                <For
                    each=move || tags_all.get().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:item
                >
                    <ModelListChip
                        name=item.clone().1
                        set_tags_enabled=set_tags_enabled
                        all_enabled=all_enabled
                        tags_enabled=tags_enabled
                        tags_all=tags_all
                    />
                </For>
            </Row>

            <Row>
                <For
                    each=move || model_list_signal.get().list.clone()
                    key=|list| list.clone()
                    let:item
                >
                    <ModelListItem
                        ipv4=ipv4
                        item=item
                        template_current=template_current.get()
                        repo_id=repo_id.get()
                        quantized_current=quantized_current.get()
                        quantized=quantized
                        tags_enabled=tags_enabled
                        gpu_type=gpu_type
                    />
                </For>
            </Row>
        </Grid>
    }
}
