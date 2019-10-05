use crate::core::*;

pub struct Mesh {
    pub vertices: Vec<Point3<f32>>,
    pub breaks: Vec<usize>,
}

impl Mesh {
    pub fn new(vertices: Vec<Point3<f32>>, breaks: Vec<usize>) -> Self {
        Self {
            vertices,
            breaks,
        }
    }
}
