use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod gauges;
mod utils;

use gauges::Arc;
use gauges::CircularGauge;
use gauges::ControlGauge;
use gauges::DialGauge;
use gauges::MetroGauge;
use gauges::Section;

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
        <div class="gaugecontainer">
            <div>
                <CircularGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature" min = {-10.0} max= {20.0} >
                    <Arc start = 0.0 end = 10.0 />
                    <Arc start = 10.0 end = 20.0 r = 0.9 style = "stroke: green;" />
                </CircularGauge>
            </div>
            <div>
                <ControlGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature2" min = {-10.0} max= {20.0} >
                    <Arc start = 0.0 end = 10.0 />
                    <Arc start = 10.0 end = 20.0 r = 0.8 style = "stroke: red;" />
                </ControlGauge>
            </div>
            <div>
                <DialGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature2" min = {-10.0} max = {20.0} step = 0.5 step_label = 5.0>
                    <Section start = 0.0 end = 10.0 />
                    <Section start = 10.0 end = 20.0 style = "stroke: red;" />
                </DialGauge>
            </div>
            <div>
                <MetroGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature2" min = {0.0} max = {120.0} >
                    <Arc start = 0.0 end = 10.0 />
                    <Arc start = 10.0 end = 20.0 style = "stroke: red;" />
                </MetroGauge>
            </div>
        </div>
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
