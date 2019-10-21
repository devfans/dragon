use crate::core::Point3;
use crate::core::Transform3;

pub enum Shape {
    Line {
        begin: Point3<f32>,
        end: Point3<f32>
    },
    Circle {
        center: Transform3,
        radius: f32,
    },
    Sphere {
        center: Point3<f32>,
        radius: f32,
    },
    Triangle {
        a: Point3<f32>,
        b: Point3<f32>,
        c: Point3<f32>,
    },
    Rectange {
        center: Transform3,
        width: f32,
        height: f32,
    },
    Cube {
        center: Transform3,
        size: f32,
    },
    Cuboid {
        center: Transform3,
        width: f32,
        height: f32,
    }
}
