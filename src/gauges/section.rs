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

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SectionContext {
    pub min: f64,
    pub max: f64,
    pub offsety: f64,
    pub offsetx: f64,
    pub width: f64,
    pub class: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub start: f64,
    pub end: f64,
    #[prop_or(1.0)]
    pub len: f64,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let ctx = use_context::<SectionContext>().expect("no ctx found");

    html! {
    <line
        x1={(ctx.offsetx + (ctx.width * (props.start - ctx.min)) / (ctx.max - ctx.min)).to_string()}
        x2={(ctx.offsetx + (ctx.width * (props.end - ctx.min)) / (ctx.max - ctx.min)).to_string()}
        y1= {(ctx.offsety * props.len).to_string()}
        y2= {(ctx.offsety * props.len).to_string()}
        class={ctx.class}
        style={props.style.clone()}
    />}
}
