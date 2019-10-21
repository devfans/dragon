use crate::core::Vector3;

pub enum Widget {
    Text {
        translation: Vector3<f32>,
        text: String
    },
}
