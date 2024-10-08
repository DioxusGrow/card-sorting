#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn ItemCardUi(card: ItemCard, id: usize, color: &'static str) -> Element {
    let mut data = use_context::<ApplicationData>();
    let mut dragStartState = data.currentCard;
    let mut dragOverBg = use_signal(|| false);
    let opacity = use_signal(|| false);

    rsx! {
        div {
            class: "w-1/6 h-1/3 rounded-lg border-4 border-solid m-5 flex justify-center items-center cursor-grab border-black text-white",
            class: if dragOverBg() { "bg-slate-500 bg-opacity-100" } else { "{color}" },
            class: if opacity() { "bg-opacity-100" } else { "bg-opacity-100" },
            draggable: card.draggable,
            ondragstart: move |_| {
                dragStartState.set(id);
                let dragStartStateID = dragStartState();
                let dataList = data.list.read();
                info!("DragStartID! {id:?} dragStartState {dragStartStateID:?}");
                info!("DataList! {dataList:?}");
            },
            ondragleave: move |_| {
                dragOverBg.set(false);
            },
            ondragover: move |ev| {
                ev.prevent_default();
                dragOverBg.set(true);
            },
            ondrop: move |ev| {
                ev.prevent_default();
                dragOverBg.set(false);
                if id != dragStartState() {
                    data.list.write().swap(id, dragStartState());
                }
                let dragStartStateID = dragStartState();
                info!("Swap DropCardID! {id:?} dragStartStateID {dragStartStateID:?}");
            },
            "{card.text} {card.id}"
        }
    }
}
