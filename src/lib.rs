//! bevy_tile_atlas is a `TextureAtlas` builder for ordered tilesets.
//!
//! In other words, this crate is used to generate a `TileAtlas` that respects the order of insertion,
//! allowing its sub-textures to exist at known indices. This is helpful for texture animations, where
//! the frames are designated by a range of indices to loop through. It can also be helpful for retrieving
//! a sub-texture without needing access to its handle (i.e., "get texture at index at index 7" instead of
//! storing/passing around a `Handle<Image>`).
//!
//! ## Example
//! ```
//! # use bevy::prelude::*;
//! # use bevy_tile_atlas::TileAtlasBuilder;
//!
//! /// Creates a tile-based, ordered `TextureAtlas`
//! ///
//! /// Assumes that the given handles are all loaded and in their desired order
//! fn build_tileset(handles: Vec<Handle<Image>>, textures: &mut Assets<Image>) -> TextureAtlas {
//!     let mut builder = TileAtlasBuilder::default();
//!
//!     for handle in handles {
//!         let texture = textures.get(&handle).unwrap();
//!         builder.add_texture(handle, texture);
//!     }
//!
//!     builder.finish(textures).unwrap()
//! }
//! ```

mod store;
mod tile_atlas;

pub use store::TextureStore;
pub use tile_atlas::{TileAtlasBuilder, TileAtlasBuilderError};
