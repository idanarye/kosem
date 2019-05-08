mod not_yet_identified;
pub use not_yet_identified::NotYetIdentifiedActor;

use actix_web::actix;

pub enum ActorRoleState {
    Init,
    NotYetIdentifiedActor(actix::Addr<NotYetIdentifiedActor>),
}
