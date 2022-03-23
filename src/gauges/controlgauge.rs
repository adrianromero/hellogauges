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
pub struct ControlGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub pattern: String,
    pub title: String,
    pub min: f64,
    pub max: f64,
    #[prop_or(180.0)]
    pub startangle: f64,
    #[prop_or(360.0)]
    pub endangle: f64,
    #[prop_or_default]
    pub children: ChildrenWithProps<Arc>,
}

#[function_component(ControlGauge)]
pub fn control_gauge(props: &ControlGaugeProps) -> Html {
    let r1 = 45.0;
    let centerx = 100;
    let centery = 80;

    let arctotal = props.endangle - props.startangle;
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
                "##, centerx, centery, svgdraw::padvalue(props.min, props.max, arctotal, v) + props.startangle - 270.0)}
              />
            },
            html! {
                <path
                    d={svgdraw::arcpath(
                        centerx,
                        centery,
                        r1,
                        props.startangle,
                        props.endangle,
                        arctotal > 180.0,
                        1)}
                    class="controlgauge-bar"
                    style={format!(r##"
                        fill: #00000000;
                        stroke-miterlimit: 0;
                        stroke-dasharray: {} 400;
                    "##, svgdraw::padvalue(props.min, props.max, arctotalrad, v))}
                />
            },
            svgdraw::format_number(&props.pattern, v),
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
            d={svgdraw::arcpath(
                centerx,
                centery,
                r1,
                props.startangle,
                props.endangle,
                arctotal > 180.0,
                1)}
            class="controlgauge-background"
            style=r##"
                fill: #00000000;
                stroke-miterlimit: 0;
                stroke-dasharray: none;
            "##
        />
        {html_arcrad}
        <ContextProvider<ArcContext> context={ArcContext{
            min: props.min,
            max: props.max,
            startangle: props.startangle,
            endangle: props.endangle,
            centerx,
            centery,
            r: 61.0,
            class: "controlgauge-arc" }}>
            { for props.children.iter() }
        </ContextProvider<ArcContext>>
        <text x=100 y=105 text-anchor="middle" class="controlgauge-value">
          { formatvalue }
        </text>
        <text
          x={centerx.to_string()}
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
