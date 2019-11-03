use actix::Message;

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

#[derive(actix_derive::MessageResponse)]
pub struct HumanDetails {
    pub name: String,
}

impl Message for GetInfo<HumanDetails> {
    type Result = HumanDetails;
}
