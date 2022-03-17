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

use super::svgdraw;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CircularGaugeProps {
    pub value: Option<f64>,
    pub title: String,
    pub min: f64,
    pub max: f64,
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
                    cx={Some(centerx.to_string())}
                    cy={Some(centery.to_string())}
                    r={Some(r1.to_string())}
                    class="circulargauge-bar"
                    style={format!(r##"
                        fill: #00000000;
                        stroke-miterlimit: 0;
                        stroke-dasharray: {} 400;
                        transform: translate({}px, {}px) rotate(-90deg) translate({}px, {}px);
                    "##, svgdraw::padvalue(props.min, props.max, r1 * svgdraw::radians(360.0), v), centerx, centery, -centerx, -centery)}
                />
            },
            v.to_string(),
        ),
        None => (html! {}, String::new()),
    };

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            version="1.1"
            viewBox="0 0 200 130"
        >
        <circle
            cx={Some(centerx.to_string())}
            cy={Some(centery.to_string())}
            r={Some(r1.to_string())}
            class="circulargauge-background"
            style="fill: #00000000; stroke-miterlimit: 0"
        />
        { html_arc }
        <text x={100} y={65} text-anchor="middle" class="circulargauge-value">
            { formatvalue }
        </text>
        <text x={100} y={85} text-anchor="middle" class="circulargauge-title">
            { props.title.clone() }
        </text>
        </svg>
    }
}
