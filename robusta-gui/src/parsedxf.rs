use std::f32::consts::PI;

use super::*;

//Arc
impl From<&dxf::entities::Arc> for REntity {
    fn from(value: &dxf::entities::Arc) -> Self {
        REntity::Arc(value.into())
    }
}

impl From<&dxf::entities::Arc> for arc::Arc {
    fn from(sp: &dxf::entities::Arc) -> Self {
        let x1 = sp.center.x + sp.start_angle.to_radians().cos() * sp.radius;
        let y1 = sp.center.y + sp.start_angle.to_radians().sin() * sp.radius;
        let point1 = point::Point::new(x1 as f32, y1 as f32, 0.);

        let x2 = sp.center.x + sp.end_angle.to_radians().cos() * sp.radius;
        let y2 = sp.center.y + sp.end_angle.to_radians().sin() * sp.radius;
        let point2 = point::Point::new(x2 as f32, y2 as f32, 0.);

        let mut p3_angle_rad = ((sp.start_angle + sp.end_angle) / 2.).to_radians() as f32;
        if sp.start_angle > sp.end_angle {
            p3_angle_rad -= PI;
        }

        let (p3_x, p3_y) = (
            sp.center.x as f32 + sp.radius as f32 * p3_angle_rad.cos(),
            sp.center.y as f32 + sp.radius as f32 * p3_angle_rad.sin(),
        );
        let lazy_point = point::Point::new(p3_x, p3_y, 0.);

        arc::Arc::new([point1, point2, lazy_point])
    }
}

//Line
impl From<&dxf::entities::Line> for REntity {
    fn from(value: &dxf::entities::Line) -> Self {
        REntity::Line(value.into())
    }
}

impl From<&dxf::entities::Line> for line::Line {
    fn from(sp: &dxf::entities::Line) -> Self {
        let point1 = point::Point::new(sp.p1.x as f32, sp.p1.y as f32, 0.);
        let point2 = point::Point::new(sp.p2.x as f32, sp.p2.y as f32, 0.);
        line::Line::new([point1, point2])
    }
}

//Circle
impl From<&dxf::entities::Circle> for REntity {
    fn from(value: &dxf::entities::Circle) -> Self {
        REntity::Circle(value.into())
    }
}

impl From<&dxf::entities::Circle> for circle::Circle {
    fn from(sp: &dxf::entities::Circle) -> Self {
        let point1 = point::Point::new((sp.center.x + sp.radius) as f32, sp.center.y as f32, 0.);
        let point2 = point::Point::new(sp.center.x as f32, sp.center.y as f32, 0.);

        circle::Circle::new([point1, point2])
    }
}

//Text
impl From<&dxf::entities::Text> for REntity {
    fn from(value: &dxf::entities::Text) -> Self {
        REntity::Text(value.into())
    }
}

impl From<&dxf::entities::Text> for text::Text {
    fn from(sp: &dxf::entities::Text) -> Self {
        let origin = point::Point::new(sp.location.x as f32, sp.location.y as f32, 0.);

        text::Text {
            bud_pos: [origin],
            body: sp.value.clone(),
            rotation: sp.rotation as f32,
            height: sp.text_height as f32,
            leader: None,
        }
    }
}

//Insert
impl From<&dxf::entities::Insert> for REntity {
    fn from(value: &dxf::entities::Insert) -> Self {
        REntity::Text(value.into())
    }
}

impl From<&dxf::entities::Insert> for text::Text {
    fn from(sp: &dxf::entities::Insert) -> Self {
        let origin = point::Point::new(sp.location.x as f32, sp.location.y as f32, 0.);

        text::Text {
            bud_pos: [origin],
            body: sp.name.clone(),
            // rotation: sp.rotation as f32,
            rotation: 0.0,
            height: 1.0,
            leader: None,
        }
    }
}

impl From<&dxf::entities::MText> for REntity {
    fn from(value: &dxf::entities::MText) -> Self {
        REntity::Text(value.into())
    }
}

impl From<&dxf::entities::MText> for text::Text {
    fn from(sp: &dxf::entities::MText) -> Self {
        let origin =
            point::Point::new(sp.insertion_point.x as f32, sp.insertion_point.y as f32, 0.);

        text::Text {
            bud_pos: [origin],
            body: sp.text.clone(),
            rotation: sp.rotation_angle as f32,
            height: 1.0,
            leader: None,
        }
    }
}

//Other
pub fn lwp_to_lines(sp: &dxf::entities::LwPolyline) -> Vec<REntity> {
    let mut lv = Vec::new();
    let mut spviter = sp.vertices.iter();
    let mut pre_v = spviter.next().unwrap();
    for current_v in spviter {
        lv.push(REntity::Line(line::Line {
            definition: [
                point::Point::new(pre_v.x as f32, pre_v.y as f32, 0.),
                point::Point::new(current_v.x as f32, current_v.y as f32, 0.),
            ],
        }));

        pre_v = current_v;
    }

    lv
}

pub fn pl_to_lines(sp: &dxf::entities::Polyline) -> Vec<REntity> {
    let mut lv = Vec::new();
    let mut spviter = sp.vertices();
    let mut pre_v = spviter.next().unwrap();
    for current_v in spviter {
        lv.push(REntity::Line(line::Line {
            definition: [
                point::Point::new(
                    pre_v.location.x as f32,
                    pre_v.location.y as f32,
                    pre_v.location.z as f32,
                ),
                point::Point::new(
                    current_v.location.x as f32,
                    current_v.location.y as f32,
                    current_v.location.z as f32,
                ),
            ],
        }));

        pre_v = current_v;
    }

    lv
}
