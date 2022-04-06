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

use super::svgdraw;

#[derive(Clone, PartialEq)]
pub struct ArcContext {
    pub min: f64,
    pub max: f64,
    pub startangle: f64,
    pub endangle: f64,
    pub centerx: i32,
    pub centery: i32,
    pub r: f64,
    pub class: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct ArcProps {
    pub start: f64,
    pub end: f64,
    #[prop_or(1.0)]
    pub r: f64,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Arc)]
pub fn arc(props: &ArcProps) -> Html {
    let ctx = use_context::<ArcContext>().expect("no ctx found");

    let arctotal = ctx.endangle - ctx.startangle;
    let arcstart = svgdraw::padvalue(ctx.min, ctx.max, arctotal, props.start) + ctx.startangle;
    let arcend = svgdraw::padvalue(ctx.min, ctx.max, arctotal, props.end) + ctx.startangle;

    html! {
    <path
        d={svgdraw::arcpath(
            ctx.centerx,
            ctx.centery,
            ctx.r * props.r,
            arcstart,
            arcend,
            arcend - arcstart > 180.0,
            1)}
        class={ctx.class}
        style={props.style.clone()}
    />}
}
