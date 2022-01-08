use bevy_asset::{Assets, Handle, HandleId};
use bevy_ecs::system::{ResMut, Resource};
use bevy_render::texture::Image;
use std::ops::{Deref, DerefMut};

pub trait TextureStore {
	fn add(&mut self, asset: Image) -> Handle<Image>;
	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Image>;
}

impl TextureStore for Assets<Image> {
	fn add(&mut self, asset: Image) -> Handle<Image> {
		self.add(asset)
	}

	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Image> {
		self.get(handle)
	}
}

impl<'w, T: TextureStore + Resource> TextureStore for ResMut<'w, T> {
	fn add(&mut self, asset: Image) -> Handle<Image> {
		self.deref_mut().add(asset)
	}

	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Image> {
		self.deref().get(handle)
	}
}
