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

use super::section::{Section, SectionContext};
use super::svgdraw;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DialGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub pattern: String,
    pub title: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub step_label: f64,
    #[prop_or_default]
    pub children: ChildrenWithProps<Section>,
}

#[function_component(DialGauge)]
pub fn fc_DialgGauge(props: &DialGaugeProps) -> Html {
    let (html_bar, formatvalue) = match props.value {
        Some(v) => (
            html! {
               <line
               x1=20
               y1=45
               x2=180
               y2=45
               class="dialgauge-bar"
               style={ format!(r##"
                    fill: #00000000;
                    stroke-miterlimit: 0;
                    stroke-dasharray: {} 400;"##,
                   svgdraw::padvalue(props.min, props.max,160.0, v)) }
             />
            },
            svgdraw::format_number(&props.pattern, v),
        ),
        None => (html! {}, String::new()),
    };

    let m = ((props.max - props.min) / props.step) as usize;
    let lines_mark: Vec<Html> = (0..=m)
        .into_iter()
        .map(|t| t as f64 * props.step + props.min)
        .map(|index| {
            let mark = 20.0 + (160.0 * (index - props.min)) / (props.max - props.min);
            html! {
                <line
                    x1={mark.to_string()}
                    y1=36
                    x2={mark.to_string()}
                    y2=54
                    class="dialgauge-mark"
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
        let mark = 20.0 + (160.0 * (index - props.min)) / (props.max - props.min);
        lines_markstep.push(html! {
            <line
                x1={mark.to_string()}
                y1=30
                x2={mark.to_string()}
                y2=60
                class="dialgauge-markstep"
            />
        });
        lines_marklabel.push(html! {
            <text
                x={mark.to_string()}
                y=70
                text-anchor="middle"
                class="dialgauge-marklabel"
            >
                { index }
            </text>
        });
    }

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            version="1.1"
            viewBox="0 0 200 130"
        >
        <g style="fill: #00000000; stroke: #D0D0D0; stroke-width: 10px; stroke-linecap: butt; stroke-miterlimit: 0;">
            <line x1 = 20 y1 = 45 x2 = 180 y2 = 45 class="dialgauge-background" />
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
        <g style="fill: #00000000; stroke:#ff0000D9; stroke-width: 10px; stroke-linecap: butt; stroke-miterlimit: 0;">
            { html_bar }
        </g>
        <g style="fill: #00000000; stroke: #808080; stroke-width: 2px; stroke-linecap: butt; stroke-miterlimit: 0;">
            <ContextProvider<SectionContext> context={SectionContext{
                min: props.min,
                max: props.max,
                offsetx: 20.0,
                offsety: 45.0,
                width: 160.0,
                class: "dialgauge-section" }}>
                { for props.children.iter() }
            </ContextProvider<SectionContext>>
        </g>
        <g style="fill: #000000D9; font: bold 14px sans-serif;">
            <text x=180 y=20 text-anchor="end" class="dialgauge-value">
                { formatvalue }
            </text>
        </g>
        <g style="fill: #0000008C; font: 10px sans-serif;">
            <text x=20 y=20 text-anchor="start" class="dialgauge-title">
                { props.title.clone() }
            </text>
        </g>
        </svg>
    }
}
