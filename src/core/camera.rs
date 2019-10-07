use crate::core::*;

pub enum Camera {
    Orthographic {
        projection: Orthographic3<f32>,
    },
    Perspective {
        projection: Perspective3<f32>,
    }
}

impl Camera {
    pub fn default() -> Self {
        Camera::Perspective {
            projection: Perspective3::new(16./9., 3.14/2., 1., 10000.),
        }
    }

    pub fn project_point(&self, point: &Point3<f32>) -> Point3<f32> {
        match self {
            Camera::Orthographic { ref projection } => projection.project_point(point),
            Camera::Perspective { ref projection } => projection.project_point(point)
        }
    }
}

