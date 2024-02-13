use std::f32::consts::PI;

pub mod arc;
pub mod circle;
pub mod line;
pub mod point;
pub mod text;

pub fn angle_full_circle(delta_x: f32, delta_y: f32) -> f32 {
    if delta_x == 0. && delta_y == 0. {
        panic!("At least two points are identical.")
    } else if delta_x == 0. {
        match delta_y.is_sign_positive() {
            true => return PI * 0.5,
            false => return PI * 1.5,
        }
    }

    let mut angle_rad = (delta_y / delta_x).atan();
    if angle_rad.is_sign_negative() {
        angle_rad += 2.0 * PI;
    }
    if delta_x.is_sign_negative() {
        angle_rad += PI;
    }
    return angle_rad % (2. * PI);
}
