
#[macro_export]
macro_rules! impl_component {
    ($type: ty) => {
        impl dragon::ecs::Component for $type {}
    }
}

#[macro_export]
macro_rules! impl_dense_component {
    ($type: ty) => {
        impl dragon::ecs::Component for $type {
            fn dense() -> bool { true }
        }
    }
}


#[macro_export]
macro_rules! entity_filter {
    ($w: expr $(,$T: ty)+) => {
        0 as u128 $(+ $w.get_component_id::<$T>().unwrap())+
    }
}

#[macro_export]
macro_rules! entity_iter_with_filter {
    ($entities: expr, $mask: expr) => {
        $entities.iter().filter(|entity| entity.components & $mask == $mask)
    }
}

#[macro_export]
macro_rules! entity_iter_mut_with_filter {
    ($entities: expr, $mask: expr) => {
        $entities.iter_mut().filter(|entity| entity.components & $mask == $mask)
    }
}

#[macro_export]
macro_rules! entity_iter {
    ($w: expr $(,$T: ty)+) => {
        {
            let mask = 0 as u128 $(+ $w.get_component_id::<$T>().unwrap())+;
            $w.entity_store.borrow().iter().filter(|entity| entity.components & mask == mask)
        }
    }
}

#[macro_export]
macro_rules! entity_iter_mut {
    ($w: expr $(,$T: ty)+) => {
        {
            let mask = 0 as u128 $(+ $w.get_component_id::<$T>().unwrap())+;
            $w.entity_store.borrow_mut().iter_mut().filter(|entity| entity.components & mask == mask)
        }
    }
}

