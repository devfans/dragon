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
            projection: Perspective3::new(16./9., 3.14/8., 1000., 10000.),
        }
    }

    pub fn project_point(&self, point: &Point3<f32>) -> Point3<f32> {
        match self {
            Camera::Orthographic { ref projection } => projection.project_point(point),
            Camera::Perspective { ref projection } => projection.project_point(point)
        }
    }

    pub fn as_matrix(&self) -> &Matrix4<f32> {
        match self {
            Camera::Orthographic { ref projection } => projection.as_matrix(),
            Camera::Perspective { ref projection } => projection.as_matrix()
        }

    }

    pub fn transform_size(&self, size: f32, distance: f32) -> f32 {
        match self {
            Camera::Orthographic { .. } => size,
            Camera::Perspective { ref projection } => {
                if distance == 0. {
                    0.
                } else {
                    size * projection.znear() / distance
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_size() {
        let camera = Camera::default();
        let distance = 10f32;
        let size = 5f32;
        assert_eq!(size / distance, camera.transform_size(size, distance));
        println!("{}", camera.transform_size(size, distance));
    }
}
 
