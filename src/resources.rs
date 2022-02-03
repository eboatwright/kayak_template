use rutoe::ResourceContainer;
use std::any::Any;

pub struct Resources {
}

impl ResourceContainer for Resources {
    fn as_any(&self) -> &dyn Any { self as &dyn Any }
}