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

use super::arc::{Arc, ArcContext};
use super::svgdraw;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetroGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub pattern: String,
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
                svgdraw::format_number(&props.pattern, v),
            )
        }
        None => (html! {}, String::new()),
    };

    let m = ((props.max - props.min) / props.step) as usize;
    let lines_mark: Vec<Html> = (0..=m)
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

    let mut lines_marklabel: Vec<Html> = Vec::new();
    let mut lines_markstep: Vec<Html> = Vec::new();

    for index in (0..=m)
        .into_iter()
        .map(|t| t as f64 * props.step_label + props.min)
    {
        let angle = svgdraw::radians(
            props.startangle - 360.0 + (arctotal * (index - props.min)) / (props.max - props.min),
        );
        let cos = angle.cos();
        let sin = angle.sin();

        lines_marklabel.push(html! {
            <text
                x={(centerx as f64 + (r1 - 13.0) * cos).to_string()}
                y={(centery as f64 + (r1 - 13.0) * sin).to_string()}
                text-anchor="middle"
                dominant-baseline="middle"
                class="metrogauge-marklabel"
            >
                { index }
            </text>
        });
        lines_markstep.push(html! {
            <line
                x1={(centerx as f64 + r1 * cos).to_string()}
                y1={(centery as f64 + r1 * sin).to_string()}
                x2={(centerx as f64 + (r1 - 6.0) * cos).to_string()}
                y2={(centery as f64 + (r1 - 6.0) * sin).to_string()}
                class="metrogauge-markstep"
            />
        });
    }

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            version="1.1"
            viewBox="0 0 200 130"
        >
        <g style="fill: #00000000; stroke: #808080; stroke-width: 2px; stroke-linecap: butt; stroke-miterlimit: 0;">
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
        </g>
        <g style="stroke: #606060; stroke-width: 0.8px; stroke-linecap: square;">
            { lines_mark }
        </g>
        <g style="stroke: #606060; stroke-width: 1px; stroke-linecap: square;">
            { lines_markstep }
        </g>
        <g style="fill: #0000008C; font: 6px sans-serif;">
            { lines_marklabel }
        </g>
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
        <g style="fill: #000000D9; font: bold 10px sans-serif;">
            <text x=100 y=85 text-anchor="middle" class="metrogauge-value">
                { formatvalue }
            </text>
        </g>
        <g style="fill: #0000008C; font: 8px sans-serif;">
            <text x={centerx.to_string()} y=55 text-anchor="middle" class="metrogauge-title">
                { props.title.clone() }
            </text>
        </g>
        <g style="fill: #ff0000D9; stroke: #606060af; stroke-width: 0.5px;">
            { html_arrow }
        </g>
        <g style="fill: #d1d1d1; stroke: #606060; stroke-width: 0.5px;">
            <circle
                cx={centerx.to_string()}
                cy={centery.to_string()}
                r=1.2
                class="metrogauge-arrowpin"
            />
        </g>
        </svg>
    }
}
