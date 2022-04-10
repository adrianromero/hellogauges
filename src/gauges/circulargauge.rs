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
pub struct CircularGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub pattern: String,
    pub title: String,
    pub min: f64,
    pub max: f64,
    #[prop_or_default]
    pub children: ChildrenWithProps<Arc>,
}

#[function_component(CircularGauge)]
pub fn circular_gauge(props: &CircularGaugeProps) -> Html {
    let r1 = 55.0;
    let centerx = 100;
    let centery = 65;

    let (html_arc, formatvalue) = match props.value {
        Some(v) => (
            html! {
                <circle
                    cx={centerx.to_string()}
                    cy={centery.to_string()}
                    r={r1.to_string()}
                    class="circulargauge-bar"
                    style={format!(r##"
                        stroke-dasharray: {} 400;
                        transform: translate({}px, {}px) rotate(-90deg) translate({}px, {}px);"##, 
                        svgdraw::padvalue(props.min, props.max, r1 * svgdraw::radians(360.0), v), centerx, centery, -centerx, -centery)}
                />
            },
            svgdraw::format_number(&props.pattern, v),
        ),
        None => (html! {}, String::new()),
    };

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
                startangle: -90.0,
                endangle: 270.0,
                centerx,
                centery,
                r: r1,
                class: "circulargauge-arc" }}>
                    { for props.children.iter() }
            </ContextProvider<ArcContext>>
        </g>
        <g style="fill: #00000000; stroke: #D0D0D0; stroke-width: 5px; stroke-linecap: butt; stroke-miterlimit: 0;">
            <circle
                cx={centerx.to_string()}
                cy={centery.to_string()}
                r={r1.to_string()}
                class="circulargauge-background"
            />
        </g>
        <g style="fill: #00000000; stroke: #0000FF; stroke-width: 5px; stroke-linecap: butt; stroke-miterlimit: 0;">
            { html_arc }
        </g>
        <g style="fill: #000000D9; font: bold 22px sans-serif;">
            <text x={100} y={65} text-anchor="middle" class="circulargauge-value">
                { formatvalue }
            </text>
        </g>
        <g style="fill: #0000008C; font: 10px sans-serif;">
            <text x={100} y={75} text-anchor="middle" dominant-baseline="hanging" class="circulargauge-title">
                { props.title.clone() }
            </text>
        </g>
        </svg>
    }
}
