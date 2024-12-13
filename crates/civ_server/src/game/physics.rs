use crate::task::context::PhysicalContext;

// TODO: try Into<Physics> ?
pub trait Physics {
    fn physics(&self) -> &PhysicalContext;
}
