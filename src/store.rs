//! Defines the `TextureStore` trait which is used by `TileAtlasBuilder` to manage its textures

use bevy_asset::{Assets, Handle, HandleId};
use bevy_ecs::system::{ResMut, Resource};
use bevy_render::texture::Image;
use std::ops::{Deref, DerefMut};

/// Trait used in the [`TileAtlasBuilder::finish`](crate::TileAtlasBuilder::finish) method to get and
/// add textures.
///
/// The reason for such a trait and not simply using `Assets<Image>` is to allow the builder to be used
/// in places where `Assets<Image>` might not be available (such as within a custom `AssetLoader`).
pub trait TextureStore {
	/// Add a texture to the store
	fn add(&mut self, asset: Image) -> Handle<Image>;
	/// Get a texture from the store
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
