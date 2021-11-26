# bevy_tile_atlas

*A `TextureAtlas` builder for ordered tilesets*

## Purpose

This crate was made specifically to work with [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap) (which I highly recommend for tile-based games in Bevy), but could potentially be used for more general purposes.

Bevy's standard `TextureAtlasBuilder` is great in that it tries to pack textures in as smartly and compactly as possible. However, this means that the indexes of the packed textures may be all over the place. For crates, such as `bevy_ecs_tilemap`, this can be an issue when a proper order matters.

Along with `bevy_ecs_tilemap`, this can more generally affect:

* Tile Animations (at least the ones that cycle through a set of tiles by incrementing index)
  * For example, `GPUAnimated` in `bevy_ecs_tilemap`
* Tile Indexing (where a designated order should be maintained)
  * For example, the first texture added should have index `0` and so on

## How it Works

This crate is essentially an augmentation of Bevy's own  `TextureAtlasBuilder`. However, instead of placing textures wherever they fit best, this builder places them in order of insertion. This order is maintained when building the finished `TextureAtlas`.

## Installation

Add to your `[dependencies]` list in `Cargo.toml`:

```toml
bevy_tile_atlas = "0.1.0"
```

## Usage

```rust
use bevy::prelude::*;
use bevy_tile_atlas::TileAtlasBuilder;

/// Creates a tile-based, ordered `TextureAtlas`
///
/// Assumes that the given handles are all loaded and in their desired order
fn build_tileset(handles: Vec<Handle<Texture>>, textures: &mut Assets<Texture>) -> TextureAtlas {
  let mut builder = TileAtlasBuilder::default();
  
  for handle in handles {
    let texture = textures.get(&handle).unwrap();
    builder.add_texture(handle, texture);
  }
  
  builder.finish(textures).unwrap()
}
```

> **Note:** Duplicate textures can be added. This is helpful for when tiles need to be at multiple indices at once.

## FAQ

**If this was made for `bevy_ecs_tilemap`, why did you not submit it as a PR?**

I chose to not open a PR for `bevy_ecs_tilemap` because it acts as more of a helper for a specific use case, as opposed to being a needed feature. It also could *potentially* stand on its own in other applications.

**Why is this limited to tiles?**

I didn't have to enforce the tile restriction, but I didn't see this being used much outside of a tile system. It may be worth while to generalize it more in the future, especially for tilesets of differing tile size. However, for now, this was the minimum required to make it work with `bevy_ecs_tilemap` (which again was its original purpose).

**Is the order guaranteed for whole folders of textures?**

If you load from a whole folder, the order of insertion will depend on how Bevy chooses to load the files (which I think can vary, though I'm not sure). Therefore, it's recommended to either manually place the tile handles into a `Vec` or array, or use some other mechanism to automatically order them (i.e. a config file).