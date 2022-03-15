// mod utils;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[function_component(App)]
fn app_component() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
 
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    yew::start_app::<App>();
    Ok(())
//     let root = document()
//     .query_selector("#root")
//     .expect("can't get #root node for rendering")
//     .expect("can't unwrap #root node");
}