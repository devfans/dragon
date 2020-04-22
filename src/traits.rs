
#[macro_export]
macro_rules! impl_component {
    ($type: ty) => {
        impl dragon::ecs::Component for $type {}
    }
}
 
