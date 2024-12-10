# bevy_headless_render

[![Crates](https://img.shields.io/crates/v/bevy_headless_render)](https://crates.io/crates/bevy_headless_render)
![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_headless_render)
[![Docs](https://img.shields.io/docsrs/bevy_headless_render)](https://exvacuum.github.io/bevy_headless_render)

A plugin for the [Bevy](https://bevyengine.org) engine which allows for headless rendering.

Every frame will be copied from `HeadlessRenderSource` render textures into `HeadlessRenderDestination` images each frame.

## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.1           | 0.14         |
| 0.2           | 0.15         |

## Installation

### crates.io
```toml
[dependencies]
bevy_headless_render = "0.2"
```

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_headless_render]
git = "https://git.exvacuum.dev/bevy_headless_render"
```

## Usage

In `main.rs`:
```rs
use bevy::prelude::*;
use bevy_headless_render;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            bevy_headless_render::HeadlessRenderPlugin,
        ))
        .run();
}
```

When spawning a camera:
```rs
let size = Extent3d {
    width: 640,
    height: 480,
    depth_or_array_layers: 1,
};

let mut image = Image {
    texture_descriptor: TextureDescriptor {
        label: None,
        size,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC
            | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    },
    ..default()
};

image.resize(size);

let image_handle = asset_server.add(image);

commands.spawn((
    HeadlessRenderSource::new(&asset_server, image_handle.clone()),
    Camera3d::default(),
    Camera {
        target: image_handle.into(),
        ..Default::default()
    },
));
```

The HeadlessRenderDestination component will contain the extracted image which can be used or saved for whatever you need.

## License

This crate is licensed under your choice of 0BSD, Apache-2.0, or MIT license.

