//! An example showing how the [`TileAtlasBuilder`] can be used
//!
//! The end result should show the tiles in the following order:
//! 1. Grass
//! 2. Dirt
//! 3. Wall
//! 4. Dirt
//! 5. Grass
//!
//! This example uses a state system to manage the loading of the various tiles
//! in order to be easier to follow, but other methods may be used of course

use bevy::{asset::LoadState, prelude::*};
use bevy_tile_atlas::TileAtlasBuilder;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.init_resource::<TileHandles>()
		.init_resource::<MyAtlas>()
		.add_state::<AppState>()
		.add_systems(OnEnter(AppState::LoadTileset), load_tiles)
		.add_systems(OnEnter(AppState::DisplayTileset), display_atlas)
		.add_systems(
			Update,
			create_atlas.run_if(in_state(AppState::CreateTileset)),
		)
		.run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum AppState {
	#[default]
	LoadTileset,
	CreateTileset,
	DisplayTileset,
}

/// The resultant atlas (or `None` if not yet generated)
#[derive(Resource, Default)]
struct MyAtlas(Option<TextureAtlas>);

/// Contains the list of handles we need to be loaded before we can build the atlas
#[derive(Resource, Default)]
struct TileHandles(Vec<Handle<Image>>);

fn load_tiles(
	mut commands: Commands,
	mut handles: ResMut<TileHandles>,
	asset_server: Res<AssetServer>,
) {
	let tiles = vec![
		asset_server.load("tiles/grass.png"),
		asset_server.load("tiles/dirt.png"),
		asset_server.load("tiles/wall.png"),
		asset_server.load("tiles/dirt.png"),
		asset_server.load("tiles/grass.png"),
	];
	handles.0 = tiles;
	commands.insert_resource(NextState(Some(AppState::CreateTileset)));
}

fn create_atlas(
	mut commands: Commands,
	mut atlas: ResMut<MyAtlas>,
	mut textures: ResMut<Assets<Image>>,
	handles: Res<TileHandles>,
	asset_server: Res<AssetServer>,
) {
	let ids = handles.0.iter().map(|h| h.id());
	for id in ids.into_iter() {
		if LoadState::Loaded != asset_server.load_state(id) {
			return;
		}
	}

	let mut builder = TileAtlasBuilder::default();
	let mut is_first = true;

	for handle in &handles.0 {
		if let Some(texture) = textures.get(handle.id()) {
			if let Ok(index) = builder.add_texture(handle.clone(), texture) {
				println!("Added texture at index: {}", index);
			}
		}

		if is_first {
			is_first = false;
			if let Some(size) = builder.get_tile_size() {
				println!("Detected tile size: {}", size);
			}
		}
	}

	atlas.0 = builder.finish(&mut textures).ok();

	commands.insert_resource(NextState(Some(AppState::DisplayTileset)));
}

fn display_atlas(
	mut atlas_res: ResMut<MyAtlas>,
	mut commands: Commands,
	mut atlases: ResMut<Assets<TextureAtlas>>,
) {
	commands.spawn(Camera2dBundle::default());

	let atlas = atlas_res.0.take().unwrap();
	let handle = atlas.texture.clone();
	let atlas_handle = atlases.add(atlas);

	// Display the third tile (Wall)
	commands.spawn(SpriteSheetBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 48.0, 0.0),
			..Default::default()
		},
		sprite: TextureAtlasSprite::new(2),
		texture_atlas: atlas_handle,
		..Default::default()
	});

	// Display the whole tileset
	commands.spawn(SpriteBundle {
		texture: handle,
		..Default::default()
	});

	atlas_res.0 = None;
}
