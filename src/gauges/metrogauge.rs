/*
MYHELLOGAUGES
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

use super::arc::{Arc, ArcContext};
use super::svgdraw;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetroGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    pub title: String,
    pub min: f64,
    pub max: f64,
    #[prop_or(2.0)]
    pub step: f64,
    #[prop_or(10.0)]
    pub step_label: f64,
    #[prop_or(135.0)]
    pub startangle: f64,
    #[prop_or(405.0)]
    pub endangle: f64,
    #[prop_or_default]
    pub children: ChildrenWithProps<Arc>,
}

#[function_component(MetroGauge)]
pub fn metro_gauge(props: &MetroGaugeProps) -> Html {
    let r1 = 55.0;
    let centerx = 100;
    let centery = 65;
    let arctotal = props.endangle - props.startangle;

    let (html_arrow, formatvalue) = match props.value {
        Some(v) => {
            let arcvalue =
                svgdraw::padvalue(props.min, props.max, arctotal, v) + props.startangle - 270.0;
            (
                html! {
                    <path
                    d="M 3 10 L -3 10 L 0 -50 Z"
                    class="metrogauge-arrow"
                    style={format!("transform: translate({}px, {}px) rotate({}deg);",
                        centerx, centery, arcvalue)}
                  />
                },
                v.to_string(),
            )
        }
        None => (html! {}, String::new()),
    };

    let m = ((props.max - props.min) / props.step) as usize;
    let lines: Vec<Html> = (0..=m)
        .into_iter()
        .map(|t| t as f64 * props.step + props.min)
        .map(|index| {
            let angle = svgdraw::radians(
                props.startangle - 360.0
                    + (arctotal * (index - props.min)) / (props.max - props.min),
            );
            let cos = angle.cos();
            let sin = angle.sin();
            html! {
                <line
                    x1={(centerx as f64 + r1 * cos).to_string()}
                    y1={(centery as f64 + r1 * sin).to_string()}
                    x2={(centerx as f64 + (r1 - 3.0) * cos).to_string()}
                    y2={(centery as f64 + (r1 - 3.0) * sin).to_string()}
                    class="metrogauge-mark"
              />
            }
        })
        .collect();

    let m = ((props.max - props.min) / props.step_label) as usize;
    let lines_label: Vec<Html> = (0..=m)
        .into_iter()
        .map(|t| t as f64 * props.step_label + props.min)
        .map(|index| {
            let angle = svgdraw::radians(
                props.startangle - 360.0
                    + (arctotal * (index - props.min)) / (props.max - props.min),
            );
            let cos = angle.cos();
            let sin = angle.sin();
            html! {
                <>
                    <line
                        x1={(centerx as f64 + r1 * cos).to_string()}
                        y1={(centery as f64 + r1 * sin).to_string()}
                        x2={(centerx as f64 + (r1 - 6.0) * cos).to_string()}
                        y2={(centery as f64 + (r1 - 6.0) * sin).to_string()}
                        class="metrogauge-markstep"
                    />
                    <text
                        x={(centerx as f64 + (r1 - 13.0) * cos).to_string()}
                        y={(centery as f64 + (r1 - 13.0) * sin).to_string()}
                        text-anchor="middle"
                        class="metrogauge-marklabel"
                    >
                        { index }
                    </text>
                </>
            }
        })
        .collect();

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            version="1.1"
            viewBox="0 0 200 130"
        >
        <ContextProvider<ArcContext> context={ArcContext{
            min: props.min,
            max: props.max,
            startangle: props.startangle,
            endangle: props.endangle,
            centerx,
            centery,
            r: 52.0,
            class: "circulargauge-arc" }}>
            { for props.children.iter() }
        </ContextProvider<ArcContext>>
        { lines }
        { lines_label }
        <path
            d={svgdraw::arcpath(
                centerx,
                centery,
                r1 + 2.0,
                props.startangle,
                props.endangle,
                arctotal > 180.0,
                1)}
            class="metrogauge-mark"
            style="fill: #00000000"
        />
        <text x=100 y=85 text-anchor="middle" class="metrogauge-value">
            { formatvalue }
        </text>
        <text x={centerx.to_string()} y=55 text-anchor="middle" class="metrogauge-title">
            { props.title.clone() }
        </text>
        { html_arrow }
        <circle
            cx={centerx.to_string()}
            cy={centery.to_string()}
            r=1.2
            class="metrogauge-arrowpin"
        />
        </svg>
    }
}
