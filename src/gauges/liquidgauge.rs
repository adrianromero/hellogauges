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
pub struct LiquidGaugeProps {
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub pattern: String,
    pub title: String,
    pub min: f64,
    pub max: f64,
}

#[function_component(LiquidGauge)]
pub fn liquid_gauge(props: &LiquidGaugeProps) -> Html {
    let r1 = 55.0;
    let r2 = 52.0;
    let centerx = 100i32;
    let centery = 65i32;

    let (html_indicator, formatvalue) = match props.value {
        Some(v) => (
            {
                let yvalue = svgdraw::padvalue(props.min, props.max, r2 * 2.0, v);
                let yvalue = centery as f64 + r2 - yvalue;
                html! {
                    <rect
                        class="liquidgauge-bar"
                        x=0
                        y={yvalue.to_string()}
                        width=200
                        height=130
                  />
                }
            },
            svgdraw::format_number(&props.pattern, v),
        ),
        None => (html! {}, String::new()),
    };

    // let clip_path = html! {
    //   <clip-path id="cut-off-bottom">
    //         { html_indicator }
    //   </clip-path>
    // };

    let clip_path = {
        #[allow(clippy::useless_conversion)]
        <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(
            #[allow(clippy::redundant_clone, unused_braces)]
            ::std::convert::Into::<::yew::virtual_dom::VNode>::into(
                ::yew::virtual_dom::VTag::__new_other(
                    ::std::borrow::Cow::<'static, ::std::primitive::str>::Borrowed("clipPath"),
                    ::std::default::Default::default(),
                    ::std::option::Option::None,
                    ::yew::virtual_dom::Attributes::Static(&[["id", "cut-off-bottom"]]),
                    ::yew::virtual_dom::listeners::Listeners::None,
                    ::yew::virtual_dom::VList::with_children(
                        {
                            let mut __yew_v = ::std::vec::Vec::new();
                            ::std::iter::Extend::extend(
                                &mut __yew_v,
                                ::std::convert::Into::<::yew::utils::NodeSeq<_, _>>::into(
                                    html_indicator,
                                ),
                            );
                            __yew_v
                        },
                        ::std::option::Option::None,
                    ),
                ),
            ),
        )
    };

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            version="1.1"
            viewBox="0 0 200 130"
        >
            <defs>
            { clip_path }
            </defs>
            <circle
                cx={centerx.to_string()}
                cy={centery.to_string()}
                r={r1.to_string()}
                class="liquidgauge-border"
                style="fill: #00000000;"
            />
            <text
                id="value1"
                x={100}
                y={65}
                text-anchor="middle"
                class="liquidgauge-value liquidgauge-value_1"
            >
                { formatvalue.clone() }
            </text>
            <text
                id="title1"
                x={100}
                y={85}
                text-anchor="middle"
                class="liquidgauge-title liquidgauge-title_1"
            >
                { props.title.clone() }
            </text>
            <circle
                cx={centerx.to_string()}
                cy={centery.to_string()}
                r={r2.to_string()}
                class="liquidgauge-background"
                clip-path="url(#cut-off-bottom)"
            />
            <text
                id="value2"
                x=100
                y=65
                text-anchor="middle"
                class="liquidgauge-value liquidgauge-value_2"
                clip-path="url(#cut-off-bottom)"
            >
                { formatvalue }
            </text>
            <text
                id="title2"
                x={100}
                y={85}
                text-anchor="middle"
                class="liquidgauge-title liquidgauge-title_2"
                clip-path="url(#cut-off-bottom)"
            >
                { props.title.clone() }
            </text>
        </svg>
    }
}