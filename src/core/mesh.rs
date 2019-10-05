use crate::core::*;

pub struct Mesh {
    vertices: Vec<Point3<f32>>,
    breaks: Vec<f32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Point3<f32>>, breaks: Vec<f32>) -> Self {
        Self {
            vertices,
            breaks,
        }
    }
}
