use crate::core::Vector3;

pub enum Widget {
    Text {
        translation: Vector3<f32>,
        text: String
    },
    FramedText {
        translation: Vector3<f32>,
        text: String,
        width: f32,
        height: f32,
    }
}

impl Widget {
    pub fn text_widget(text: &str, x: f32, y: f32) -> Self {
        Widget::Text {
            translation: Vector3::new(x, y, 0.),
            text: text.to_string()
        }
    }

    pub fn framed_text_widget(text: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Widget::FramedText {
            translation: Vector3::new(x, y, 0.),
            text: text.to_string(),
            width: w,
            height: h
        }
    }
}
