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

mod circulargauge;
mod controlgauge;
mod dialgauge;
mod liquidgauge;
mod metrogauge;

mod arc;
mod section;
mod svgdraw;

pub use circulargauge::{CircularGauge, CircularGaugeProps};
pub use controlgauge::{ControlGauge, ControlGaugeProps};
pub use dialgauge::{DialGauge, DialGaugeProps};
pub use liquidgauge::{LiquidGauge, LiquidGaugeProps};
pub use metrogauge::{MetroGauge, MetroGaugeProps};

pub use arc::{Arc, ArcProps};
pub use section::{Section, SectionProps};
