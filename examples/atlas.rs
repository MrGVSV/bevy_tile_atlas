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
		.init_state::<AppState>()
		.add_systems(OnEnter(AppState::Load), load_tiles)
		.add_systems(OnEnter(AppState::Display), display_atlas)
		.add_systems(
			Update,
			create_atlas.run_if(in_state(AppState::Create)),
		)
		.run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum AppState {
	#[default]
	Load,
	Create,
	Display
}

/// The resultant atlas (or `None` if not yet generated)
#[derive(Resource, Default)]
struct MyAtlas(Option<(Handle<Image>, TextureAtlasLayout)>);

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
	commands.insert_resource(NextState(Some(AppState::Create)));
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

	commands.insert_resource(NextState(Some(AppState::Display)));
}

fn display_atlas(
	mut atlas_res: ResMut<MyAtlas>,
	mut commands: Commands,
	mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
	commands.spawn(Camera2dBundle::default());

	let (texture, atlas_layout) = atlas_res.0.take().unwrap();
	let atlas_handle = atlases.add(atlas_layout);

	// Display the third tile (Wall)
	commands.spawn(SpriteSheetBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 48.0, 0.0),
			..Default::default()
		},
		sprite: Sprite::default(),
		atlas: TextureAtlas {
			layout: atlas_handle,
			index: 2,
		},
		texture: texture.clone(),
		..Default::default()
	});

	// Display the whole tileset
	commands.spawn(SpriteBundle {
		texture,
		..Default::default()
	});

	atlas_res.0 = None;
}
