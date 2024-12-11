use crate::task::context::PhysicalContext;

pub mod city;
pub mod task;

pub trait Physics {
    fn physics(&self) -> &PhysicalContext;
}
