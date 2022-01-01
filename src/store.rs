use bevy::asset::{Assets, Handle, HandleId};
use bevy::ecs::component::Component;
use bevy::prelude::{ResMut, Texture};
use std::ops::{Deref, DerefMut};

pub trait TextureStore {
	fn add(&mut self, asset: Texture) -> Handle<Texture>;
	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Texture>;
}

impl TextureStore for Assets<Texture> {
	fn add(&mut self, asset: Texture) -> Handle<Texture> {
		self.add(asset)
	}

	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Texture> {
		self.get(handle)
	}
}

impl<'w, T: TextureStore + Component> TextureStore for ResMut<'w, T> {
	fn add(&mut self, asset: Texture) -> Handle<Texture> {
		self.deref_mut().add(asset)
	}

	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Texture> {
		self.deref().get(handle)
	}
}
