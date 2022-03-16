use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod gauges;
mod utils;

use gauges::CircularGauge;
use gauges::ControlGauge;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app_component() -> Html {
    let counter = use_state(|| 0.0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1.0;
            counter.set(value);
        }
    };

    let onclick2 = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1.0;
            counter.set(value);
        }
    };

    html! {
        <div>
        <button {onclick}>{ "+1" }</button>
        <button onclick={onclick2}>{ "-1" }</button>
        <p>{ *counter }</p>
        <p>{ (4.5f64).sin() }</p>
        <CircularGauge value = { Some(*counter) } title = "Temperature" min = 0.0 max= 20.0 />
        <ControlGauge value = { Some(*counter) } title = "Temperature2" min = 0.0 max= 20.0 startangle=180.0 endangle=360.0/>
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
