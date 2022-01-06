use crate::TextureStore;
use bevy::log::{debug, error, warn};
use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::{Rect, TextureAtlasBuilderError};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TileAtlasBuilderError {
	#[error("the given tile does not match the current tile size (expected {expected:?}, found {found:?})")]
	InvalidTileSize { expected: Vec2, found: Vec2 },
	#[error("the atlas does not contain any tiles")]
	EmptyAtlas,
	#[error("received internal TextureAtlas error ({0:?})")]
	Internal(TextureAtlasBuilderError),
}

/// Used to build a `TextureAtlas` that maintains the order of the textures added
/// and enforces a fixed tile size
pub struct TileAtlasBuilder {
	/// The size of _all_ tiles in the atlas
	///
	/// If `None`, the size will be auto-detected based on the first tile added
	tile_size: Option<Vec2>,
	/// The maximum number of columns to allow before wrapping
	///
	/// If `None`, then no wrapping (i.e. single row)
	max_columns: Option<usize>,
	/// The ordered collection of texture handles in this atlas
	handles: Vec<Handle<Texture>>,
	/// The texture format for the textures that will be loaded in the atlas.
	format: TextureFormat,
	/// Enable automatic format conversion for textures if they are not in the atlas format.
	auto_format_conversion: bool,
}

impl Default for TileAtlasBuilder {
	fn default() -> Self {
		Self {
			tile_size: None,
			max_columns: None,
			handles: Vec::default(),
			format: TextureFormat::Rgba8UnormSrgb,
			auto_format_conversion: true,
		}
	}
}

impl TileAtlasBuilder {
	/// Create a new [`TileAtlasBuilder`] with a set tile size
	///
	/// # Arguments
	///
	/// * `tile_size`: The tile size
	///
	/// returns: TileAtlasBuilder
	///
	/// # Examples
	///
	/// ```
	/// let mut builder = TileAtlasBuilder::new(Vec2::new(32.0, 32.0));
	/// ```
	pub fn new(tile_size: Vec2) -> Self {
		Self {
			tile_size: Some(tile_size),
			..Default::default()
		}
	}

	/// Sets the tile size.
	///
	/// If `None`, the size will be auto-detected based on the first tile added.
	///
	/// > _Note that this will remove all currently added textures._
	///
	/// # Arguments
	///
	/// * `size`: The new tile size
	///
	/// returns: TileAtlasBuilder
	///
	/// # Examples
	///
	/// ```
	/// // Auto-size
	///	let mut builder = TileAtlasBuilder::default();
	/// // Fixed-size
	/// let mut builder = builder.tile_size(Some(Vec2::new(32.0, 32.0)));
	/// // Back to auto-size
	/// let mut builder = builder.tile_size(None);
	/// ```
	pub fn tile_size(mut self, size: Option<Vec2>) -> Self {
		self.tile_size = size;

		// We need to clear the handles vector since we can't be sure they'll fit this new size
		self.handles.clear();

		self
	}

	/// Sets the maximum number of columns to allow before wrapping
	///
	/// If `None`, then no wrapping (i.e. single row)
	pub fn max_columns(&mut self, max_columns: Option<usize>) -> &mut Self {
		self.max_columns = max_columns;
		self
	}

	/// Sets the texture format for textures in the atlas.
	pub fn format(mut self, format: TextureFormat) -> Self {
		self.format = format;
		self
	}

	/// Control whether the added texture should be converted to the atlas format, if different.
	pub fn auto_format_conversion(mut self, auto_format_conversion: bool) -> Self {
		self.auto_format_conversion = auto_format_conversion;
		self
	}

	/// Gets the current tile size (if any)
	pub fn get_tile_size(&self) -> Option<Vec2> {
		self.tile_size
	}

	/// Gets the current maximum number of columns
	///
	/// If the columns property was not set, this will equal the number of added textures
	pub fn get_max_columns(&self) -> usize {
		self.max_columns.unwrap_or(self.handles.len())
	}

	/// Gets the current number of added textures
	pub fn len(&self) -> usize {
		self.handles.len()
	}

	/// Adds a texture to be copied to the texture atlas.
	///
	/// If a size has not been set, the size of the given texture will be the designated
	/// tile size for this atlas
	///
	/// If successful, returns the index of the inserted texture
	///
	/// # Arguments
	///
	/// * `texture_handle`: The texture handle
	/// * `texture`: The actual texture
	///
	/// returns: Result<usize, TileAtlasBuilderError>
	///
	pub fn add_texture(
		&mut self,
		texture_handle: Handle<Texture>,
		texture: &Texture,
	) -> Result<usize, TileAtlasBuilderError> {
		if let Some(size) = self.tile_size {
			if texture.size.width > size.x as u32 || texture.size.height > size.y as u32 {
				let expected = size;
				let found = texture.size.as_vec3().truncate();
				warn!(
					"The given texture does not fit into specified tile size (expected: {:?}, found: {:?}). Skipping...",
					expected, found,
				);
				return Err(TileAtlasBuilderError::InvalidTileSize { expected, found });
			}
		} else {
			let new_size = texture.size.as_vec3().truncate();
			self.tile_size = Some(new_size);
		};

		self.handles.push(texture_handle);
		Ok(self.handles.len() - 1usize)
	}

	/// Build the final `TextureAtlas`
	pub fn finish<TStore: TextureStore>(
		self,
		textures: &mut TStore,
	) -> Result<TextureAtlas, TileAtlasBuilderError> {
		let total = self.handles.len();
		if total == 0usize {
			return Err(TileAtlasBuilderError::EmptyAtlas);
		}

		let tile_size = &self.tile_size.unwrap();

		let total_rows = ((total as f32) / self.get_max_columns() as f32).ceil() as usize;

		let mut atlas_texture = Texture::new_fill(
			Extent3d::new(
				(self.get_max_columns() as f32 * tile_size.x) as u32,
				((total_rows as f32) * tile_size.y) as u32,
				1,
			),
			TextureDimension::D2,
			&[0, 0, 0, 0],
			self.format,
		);

		let mut row_idx = 0usize;
		let mut col_idx = 0usize;
		let mut texture_handles = HashMap::default();
		let mut texture_rects = Vec::with_capacity(total);
		for (index, handle) in self.handles.iter().enumerate() {
			let texture = textures.get(handle).unwrap();
			let x = (col_idx as f32) * tile_size.x;
			let y = (row_idx as f32) * tile_size.y;
			let min = Vec2::new(x, y);
			let max = min + Vec2::new(tile_size.x, tile_size.y);

			texture_handles.insert(handle.clone_weak(), index);
			texture_rects.push(Rect { min, max });
			if texture.format != self.format && !self.auto_format_conversion {
				warn!(
					"Loading a texture of format '{:?}' in an atlas with format '{:?}'",
					texture.format, self.format
				);
				return Err(TileAtlasBuilderError::Internal(
					TextureAtlasBuilderError::WrongFormat,
				));
			}
			self.copy_converted_texture(&mut atlas_texture, texture, col_idx, row_idx);

			if (index + 1usize).wrapping_rem(self.get_max_columns()) == 0usize {
				row_idx += 1usize;
				col_idx = 0usize;
			} else {
				col_idx += 1usize;
			}
		}

		Ok(TextureAtlas {
			size: atlas_texture.size.as_vec3().truncate(),
			texture: textures.add(atlas_texture),
			textures: texture_rects,
			texture_handles: Some(texture_handles),
		})
	}

	fn copy_converted_texture(
		&self,
		atlas_texture: &mut Texture,
		texture: &Texture,
		column_index: usize,
		row_index: usize,
	) {
		if self.format == texture.format {
			self.copy_texture_to_atlas(atlas_texture, texture, column_index, row_index);
		} else if let Some(converted_texture) = texture.convert(self.format) {
			debug!(
				"Converting texture from '{:?}' to '{:?}'",
				texture.format, self.format
			);
			self.copy_texture_to_atlas(atlas_texture, &converted_texture, column_index, row_index);
		} else {
			error!(
				"Error converting texture from '{:?}' to '{:?}', ignoring",
				texture.format, self.format
			);
		}
	}

	fn copy_texture_to_atlas(
		&self,
		atlas_texture: &mut Texture,
		texture: &Texture,
		column_index: usize,
		row_index: usize,
	) {
		let tile_size = self
			.tile_size
			.expect("Tile size should have been specified by this point.");
		let rect_width = tile_size.x as usize;
		let rect_height = tile_size.y as usize;
		let rect_x = column_index * tile_size.x as usize;
		let rect_y = row_index * tile_size.y as usize;
		let atlas_width = atlas_texture.size.width as usize;
		let format_size = atlas_texture.format.pixel_size();

		for (texture_y, bound_y) in (rect_y..rect_y + rect_height).enumerate() {
			let begin = (bound_y * atlas_width + rect_x) * format_size;
			let end = begin + rect_width * format_size;
			let texture_begin = texture_y * rect_width * format_size;
			let texture_end = texture_begin + rect_width * format_size;

			atlas_texture.data[begin..end]
				.copy_from_slice(&texture.data[texture_begin..texture_end]);
		}
	}
}
