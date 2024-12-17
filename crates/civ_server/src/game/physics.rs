use crate::task::context::GeoContext;

// TODO: try Into<Physics> ?
pub trait Geo {
    fn geo(&self) -> &GeoContext;
    fn geo_mut(&mut self) -> &mut GeoContext;
}
