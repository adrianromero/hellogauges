use std::f64::consts::PI;

use yew::prelude::*;

fn padvalue(min: f64, max: f64, length: f64, value: f64) -> f64 {
    let lengthvalue = (length * (value - min)) / (max - min);
    if lengthvalue < 0.0 {
        return 0.0;
    }
    if lengthvalue > length {
        return length;
    }
    lengthvalue
}

fn create_padvalue(min: f64, max: f64, length: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |value: f64| {
        let lengthvalue = (length * (value - min)) / (max - min);
        if lengthvalue < 0.0 {
            return 0.0;
        }
        if lengthvalue > length {
            return length;
        }
        lengthvalue
    })
}

fn radians(angle: f64) -> f64 {
    (angle * PI) / 180.0
}

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

    let (arcvalue, formatvalue) = match props.value {
        Some(v) => (
            Some(padvalue(props.min, props.max, r1 * radians(360.0), v)),
            v.to_string(),
        ),
        None => (None, String::new()),
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
        {
            match arcvalue{
                Some(av)=> html!{
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
                        "##, av, centerx, centery, -centerx, -centery)}
                    />
                },
                None => html!{}
            }
        }
        <text x={100} y={65} textAnchor="middle" class="circulargauge-value">
            {formatvalue}
        </text>
        <text x={100} y={85} textAnchor="middle" class="circulargauge-title">
            {props.title.clone()}
        </text>
        </svg>
    }
}
