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

use std::f64::consts::PI;

pub fn piepath(
    cx: i32,
    cy: i32,
    r: f64,
    start: f64,
    end: f64,
    orientation: bool,
    sweep: i32,
) -> String {
    format!(
        "M{} {} A {} {} 1 {} {} {} {} L {} {} Z",
        cx as f64 + start.cos() * r,
        cy as f64 + start.sin() * r,
        r,
        r,
        if orientation { 1 } else { 0 },
        sweep,
        cx as f64 + end.cos() * r,
        cy as f64 + end.sin() * r,
        cx,
        cy
    )
}

pub fn arcpath(
    cx: i32,
    cy: i32,
    r: f64,
    start: f64,
    end: f64,
    orientation: bool,
    sweep: i32,
) -> String {
    let startradians = radians(start);
    let endradians = radians(end);
    format!(
        "M{} {} A {} {} 1 {} {} {} {}",
        cx as f64 + startradians.cos() * r,
        cy as f64 + startradians.sin() * r,
        r,
        r,
        if orientation { 1 } else { 0 },
        sweep,
        cx as f64 + endradians.cos() * r,
        cy as f64 + endradians.sin() * r
    )
}

pub fn padvalue(min: f64, max: f64, length: f64, value: f64) -> f64 {
    let lengthvalue = (length * (value - min)) / (max - min);
    if lengthvalue < 0.0 {
        return 0.0;
    }
    if lengthvalue > length {
        return length;
    }
    lengthvalue
}

// pub fn create_padvalue(min: f64, max: f64, length: f64) -> Box<dyn Fn(f64) -> f64> {
//     Box::new(move |value: f64| {
//         let lengthvalue = (length * (value - min)) / (max - min);
//         if lengthvalue < 0.0 {
//             return 0.0;
//         }
//         if lengthvalue > length {
//             return length;
//         }
//         lengthvalue
//     })
// }

pub fn radians(angle: f64) -> f64 {
    (angle * PI) / 180.0
}
