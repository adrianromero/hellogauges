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

fn use_counter(
    init: f64,
    min: f64,
    max: f64,
) -> (f64, Box<dyn Fn(f64) -> Box<dyn Fn(MouseEvent) -> ()>>) {
    let counter_hook = use_state(|| init);
    let counter: f64 = *counter_hook;
    let inc_counter: Box<dyn Fn(f64) -> Box<dyn Fn(MouseEvent) -> ()>> =
        Box::new(move |inc: f64| {
            let counter_hook = counter_hook.clone();
            Box::new(move |_| {
                let value = *counter_hook + inc;
                let value = if value > max { max } else { value };
                let value = if value < min { min } else { value };
                counter_hook.set(value);
            })
        });
    (counter, inc_counter)
}

#[function_component(App)]
fn app_component() -> Html {
    let (counter, inc_counter) = use_counter(-2.0, -10.0, 20.0);
    let (power, inc_power) = use_counter(60.0, 0.0, 100.0);
    let (usage, inc_usage) = use_counter(50.0, 0.0, 100.0);
    let (weight, inc_weight) = use_counter(72.0, 40.0, 120.0);
    let (km, inc_km) = use_counter(50.0, 0.0, 120.0);
    let (humidity, inc_humidity) = use_counter(60.0, 0.0, 100.0);

    html! {
    <div>
    <h1>{"HELLOGAUGES"}</h1>
    <div>{"Gauge Components for the Yew framework"}</div>

    <div class="gaugegallery">
    <div class="gaugecontainer">
        <div></div>
        <div class="gaugetitle">
            <button onclick={inc_counter(-1.0)}>{ "<" }</button>
            {"\u{00a0}CircularGauge\u{00a0}"}
            <button onclick={inc_counter(1.0)}>{ ">" }</button>
        </div>
        <div class="gaugetitle">
            <button onclick={inc_power(-2.0)}>{ "<<" }</button>
            {"\u{00a0}SimpleGauge\u{00a0}"}
            <button onclick={inc_power(2.0)}>{ ">>" }</button>
        </div>
        <div class="gaugetitle">
            <button onclick={inc_usage(-5.0)}>{ "<<" }</button>
            {"\u{00a0}ControlGauge\u{00a0}"}
            <button onclick={inc_usage(5.0)}>{ ">>" }</button>
        </div>
    </div>
    <div class="gaugecontainer">
        <div class="gaugestyle">{"Default gauges style"}</div>
        <div>
            <CircularGauge value = { Some(counter) } pattern="°C,1" title = "Temperature" min = {-10.0} max= {20.0} />
        </div>
        <div>
            <SimpleGauge value = { Some(power) } pattern="kW,0" title = "Power" min = {0.0} max= {100.0} />
        </div>
        <div>
            <ControlGauge value = { Some(usage) } pattern="Gb,0" title = "Usage" min = {0.0} max= {100.0} />
        </div>
    </div>
    <div class="gaugecontainer gaugestyled">
        <div class="gaugestyle">{"Default gauges style"}</div>
        <div>
            <CircularGauge value = { Some(counter) } pattern="°C,1" title = "Temperature" min = {-10.0} max= {20.0} >
                <Arc start = {-10.0} end = 5.0 style = "stroke: #0000FF30;" />
                <Arc start = 5.0 end = 20.0 style = "stroke:  #FF000030;" />
            </CircularGauge>
        </div>
        <div>
            <SimpleGauge value = { Some(power) } pattern="kW,0" title = "Power" min = {0.0} max= {100.0} >
                <Arc start = 0.0 end = 20.0 style = "stroke: green;" />
                <Arc start = 20.0 end = 80.0 style = "stroke: lightgray;" />
                <Arc start = 80.0 end = 100.0 style = "stroke: red;" />
            </SimpleGauge>
        </div>
        <div>
            <ControlGauge value = { Some(usage) } pattern="Gb,0" title = "Usage" min = {0.0} max= {100.0}  >
                <Arc start = 80.0 end = 100.0 r = 0.8 />
            </ControlGauge>
        </div>
    </div>

    <div class="gaugecontainer">
        <div></div>
        <div class="gaugetitle">
            <button onclick={inc_weight(-2.0)}>{ "<<" }</button>
            {"\u{00a0}DialGauge\u{00a0}"}
            <button onclick={inc_weight(2.0)}>{ ">>" }</button>
        </div>
        <div class="gaugetitle">
            <button onclick={inc_km(-5.0)}>{ "<<" }</button>
            {"\u{00a0}MetroGauge\u{00a0}"}
            <button onclick={inc_km(5.0)}>{ ">>" }</button>
        </div>
        <div class="gaugetitle">
            <button onclick={inc_humidity(-5.0)}>{ "<<" }</button>
            {"\u{00a0}LiquidGauge\u{00a0}"}
            <button onclick={inc_humidity(5.0)}>{ ">>" }</button>
        </div>
    </div>
    <div class="gaugecontainer">
        <div class="gaugestyle">{"Default gauges style"}</div>
        <div>
            <DialGauge value = { Some(weight) } pattern="Kg,3" title = "Weight" min = {40.0} max = {120.0} step = 2.0 step_label = 10.0/>
        </div>
        <div>
            <MetroGauge value = { Some(km) } pattern="km/h,0" title = "Speedometer" min = {0.0} max = {120.0} />
        </div>
        <div>
            <LiquidGauge value = { Some(humidity) } pattern="%,0" title = "Humidity" min = {0.0} max = {100.0} />
        </div>
    </div>
    <div class="gaugecontainer gaugestyled">
        <div class="gaugestyle">{"Styled gauges"}</div>
        <div>
            <DialGauge value = { Some(weight) } pattern="Kg,3" title = "Weight" min = {40.0} max = {120.0} step = 2.0 step_label = 10.0>
                <Section start = 0.0 end = 10.0 />
                <Section start = 10.0 end = 20.0 style = "stroke: red;" />
            </DialGauge>
        </div>
        <div>
            <MetroGauge value = { Some(km) } pattern="km/h,0" title = "Speedometer" min = {0.0} max = {120.0} />
        </div>
        <div>
            <LiquidGauge value = { Some(humidity) } pattern="%,0" title = "Humidity" min = {0.0} max = {100.0} />
        </div>
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
