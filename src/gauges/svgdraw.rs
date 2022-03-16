use std::f64::consts::PI;

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

pub fn create_padvalue(min: f64, max: f64, length: f64) -> Box<dyn Fn(f64) -> f64> {
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

pub fn radians(angle: f64) -> f64 {
    (angle * PI) / 180.0
}
