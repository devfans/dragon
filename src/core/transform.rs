use crate::core::*;

pub struct Transform2d {
}

pub struct Transform3 {
    isometry: Isometry3<f32>, 
    scale: Vector3<f32>,
}

impl Transform3 {
    pub fn new(
        position: Translation3<f32>,
        rotation: UnitQuaternion<f32>,
        scale: Vector3<f32>,
    ) -> Self {
        Self {
            isometry: Isometry3::from_parts(na::convert(position), na::convert(rotation)),
            scale: na::convert(scale),
        }
    }

    pub fn default() -> Self {
        Self {
            isometry: Isometry3::identity(),
            scale: Vector3::from_element(1.),
        }
    }

    #[inline]
    pub fn translation(&self) -> &Vector3<f32> {
        &self.isometry.translation.vector
    }

    #[inline]
    pub fn translation_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.isometry.translation.vector
    }

    #[inline]
    pub fn rotation(&self) -> &UnitQuaternion<f32> {
        &self.isometry.rotation
    }

    #[inline]
    pub fn rotation_mut(&mut self) -> &mut UnitQuaternion<f32> {
        &mut self.isometry.rotation
    }

    #[inline]
    pub fn prepend_translation(&mut self, translation: Vector3<f32>) -> &mut Self {
        self.isometry.translation.vector += translation;
        self
    }
    
    #[inline]
    pub fn append_translation(&mut self, translation: Vector3<f32>) -> &mut Self {
        self.isometry.translation.vector += self.isometry.rotation * translation;
        self
    }

    pub fn set_translation(
        &mut self,
        position: Vector3<f32>,
    ) -> &mut Self {
        self.isometry.translation.vector = na::convert(position);
        self
    }
    
    pub fn set_translation_xyz(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.set_translation(Vector3::new(x, y, z))
    }

    pub fn set_rotation(
        &mut self,
        rotation: UnitQuaternion<f32>,
    ) -> &mut Self {
        self.isometry.rotation = na::convert(rotation);
        self
    }
}


