use std::cell::RefCell;
use std::rc::Rc;

pub type Physics2D = Rc<RefCell<Physics2DProto>>;

pub struct Physics2DProto {
    w: i32,
    h: i32,
}

impl Physics2DProto {
    pub fn new(w: i32, h: i32) -> Physics2D {
        let px = Self { w, h };
        Rc::new(RefCell::new(px))
    }
}

