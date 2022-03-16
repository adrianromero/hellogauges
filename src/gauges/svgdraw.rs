use std::f64::consts::PI;

pub fn arcpath(
    cx: i32,
    cy: i32,
    r: f64,
    start: f64,
    end: f64,
    orientation: i32,
    sweep: i32,
) -> String {
    format!(
        "M{} {} A {} {} 1 {} {} {} {}",
        cx as f64 + start.cos() * r,
        cy as f64 + start.sin() * r,
        r,
        r,
        orientation,
        sweep,
        cx as f64 + end.cos() * r,
        cy as f64 + end.sin() * r
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
