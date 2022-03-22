use std::marker::PhantomData;

use crate::{handle::Handle, path::AssetPath, Asset, AssetServer};
use bevy_ecs::{proxy::Proxy, reflect::ReflectProxyComponent, world::World};
use bevy_reflect::{FromReflect, Reflect};

#[derive(Reflect, FromReflect)]
#[reflect(ProxyComponent)]
pub struct ProxyHandle<T: Asset> {
    pub path: String,
    #[reflect(ignore)]
    marker: PhantomData<fn() -> T>,
}

impl<T: Asset> Default for ProxyHandle<T> {
    fn default() -> Self {
        Self {
            path: Default::default(),
            marker: PhantomData,
        }
    }
}

impl<T: Asset> Proxy for ProxyHandle<T> {
    type Target = Handle<T>;
    fn resolve(self, world: &mut World) -> Option<Self::Target> {
        let asset_server = world.get_resource::<AssetServer>()?;
        Some(asset_server.load(AssetPath::from(&self.path)))
    }
}
