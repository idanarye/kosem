use actix::{Message, MessageResponse};

pub struct GetInfo<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for GetInfo<T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T: 'static> Message for GetInfo<T> {
    type Result = T;
}

#[derive(MessageResponse)]
pub struct HumanDetails {
    pub name: String,
}
