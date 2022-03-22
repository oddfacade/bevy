use crate::world::World;
pub trait Proxy {
    type Target;
    fn resolve(self, world: &mut World) -> Option<Self::Target>;
}
