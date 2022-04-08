/*
HELLOGAUGES
Copyright (C) 2022 Adrián Romero
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod utils;

use hellogauges::CircularGauge;
use hellogauges::ControlGauge;
use hellogauges::DialGauge;
use hellogauges::LiquidGauge;
use hellogauges::MetroGauge;
use hellogauges::SimpleGauge;
use hellogauges::{Arc, Section};

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
                    <Arc start = 0.0 end = 10.0  r = 1.15 />
                    <Arc start = 10.0 end = 20.0 r = 0.9 style = "stroke: green;" />
                </CircularGauge>
            </div>
            <div>
                <SimpleGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature2" min = {-10.0} max= {20.0} >
                    <Arc start = 0.0 end = 10.0 />
                    <Arc start = 10.0 end = 20.0 r = 0.8 style = "stroke: red;" />
                </SimpleGauge>
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
            <div>
                <LiquidGauge value = { Some(*counter) } pattern="°C,1" title = "Temperature2" min = {0.0} max = {120.0} />
            </div>
        </div>
    </div>
    }
}
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    yew::start_app::<App>();
    // let root = document()
    //     .query_selector("#root")
    //     .expect("can't get #root node for rendering")
    //     .expect("can't unwrap #root node");
    // yew::start_app_in_element::<App>(root);
    Ok(())
}
