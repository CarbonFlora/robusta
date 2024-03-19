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

        let mut a = text::Text::new(origin);
        a.body = sp.value.clone();
        a.horizontal_tj = match sp.horizontal_text_justification {
            dxf::enums::HorizontalTextJustification::Left => text::HorizontalTJ::Left,
            dxf::enums::HorizontalTextJustification::Center => text::HorizontalTJ::Center,
            dxf::enums::HorizontalTextJustification::Right => text::HorizontalTJ::Right,
            dxf::enums::HorizontalTextJustification::Aligned => text::HorizontalTJ::Aligned,
            dxf::enums::HorizontalTextJustification::Middle => text::HorizontalTJ::Middle,
            dxf::enums::HorizontalTextJustification::Fit => text::HorizontalTJ::Fit,
        };
        a.vertical_tj = match sp.vertical_text_justification {
            dxf::enums::VerticalTextJustification::Baseline => text::VerticalTJ::Baseline,
            dxf::enums::VerticalTextJustification::Bottom => text::VerticalTJ::Bottom,
            dxf::enums::VerticalTextJustification::Middle => text::VerticalTJ::Middle,
            dxf::enums::VerticalTextJustification::Top => text::VerticalTJ::Top,
        };
        a
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

        text::Text::new(origin)
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

        text::Text::new(origin)
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
