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
pub struct ControlGaugeProps {
    pub value: Option<f64>,
    pub title: String,
    pub min: f64,
    pub max: f64,
    pub startangle: Option<f64>,
    pub endangle: Option<f64>,
}

#[function_component(ControlGauge)]
pub fn control_gauge(props: &ControlGaugeProps) -> Html {
    let r1 = 45.0;
    let centerx = 100;
    let centery = 80;
    let startangle = props.startangle.unwrap_or(180.0);
    let endangle = props.endangle.unwrap_or(360.0);

    let arctotal = endangle - startangle;
    let arctotalrad = r1 * svgdraw::radians(arctotal);

    let (html_arc, html_arcrad, formatvalue) = match props.value {
        Some(v) => (
            html! {
                <path
                // d="M 1 10 L -1 10  L -1 -55 L 0 -60 L 1 -55 Z"
                d="M 5 5 L 0 10 L -5 5 L 0 -65 Z"
                class="controlgauge-arrow"
                style={format!(r##"
                    transform: translate({}px, {}px) rotate({}deg);
                "##, centerx, centery, svgdraw::padvalue(props.min, props.max, arctotal, v) + startangle - 270.0)}
              />
            },
            html! {
                <path
                d={svgdraw::arcpath(centerx, centery, r1,
                    svgdraw::radians(startangle),
                    svgdraw::radians(endangle),
                    if arctotal > 180.0 { 1 } else { 0 },
                    1)}
                class="controlgauge-bar"
                style={format!(r##"
                    fill: #00000000;
                    stroke-miterlimit: 0;
                    stroke-dasharray: {} 400;
                "##, svgdraw::padvalue(props.min, props.max, arctotalrad,v))}
              />
            },
            v.to_string(),
        ),
        None => (html! {}, html! {}, String::new()),
    };

    html! {
        <svg
        xmlns="http://www.w3.org/2000/svg"
        version="1.1"
        viewBox="0 0 200 130"
      >
        <path
          d={svgdraw::arcpath(centerx,
            centery,
            r1,
            svgdraw::radians(startangle),
            svgdraw::radians(endangle),
            if arctotal > 180.0 { 1 } else { 0 },
            1)}
          class="controlgauge-background"
          style=r##"
            fill: #00000000;
            stroke-miterlimit: 0;
            stroke-dasharray: none;
          "##
        />
        // <Arcs
        //   arcs={arcs}
        //   min={min}
        //   max={max}
        //   centerx={centerx}
        //   centery={centery}
        //   startangle={startangle}
        //   endangle={endangle}
        // />
        {html_arcrad}
        <text x=100 y=105 text-anchor="middle" class="controlgauge-value">
          { formatvalue }
        </text>
        <text
          x={ Some(centerx.to_string()) }
          y=15
          text-anchor="middle"
          class="controlgauge-title"
        >
          { props.title.clone() }
        </text>
        { html_arc }
      </svg>

    }
}
