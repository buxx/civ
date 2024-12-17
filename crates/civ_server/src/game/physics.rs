use crate::task::context::GeoContext;

// TODO: try Into<Physics> ?
pub trait Geo {
    fn physics(&self) -> &GeoContext;
    fn physics_mut(&mut self) -> &mut GeoContext;
}
