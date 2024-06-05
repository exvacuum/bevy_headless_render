# bevy_framebuffer_extract

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_framebuffer_extract)
![Build](https://img.shields.io/github/actions/workflow/status/exvacuum/bevy_framebuffer_extract/rust.yml)
[![Docs](https://img.shields.io/website?url=https%3A%2F%2Fexvacuum.github.io%2Fbevy_framebuffer_extract%2F&label=docs)](https://exvacuum.github.io/bevy_framebuffer_extract)

A plugin for the [Bevy](https://bevyengine.org) engine which allows for exporting framebuffer data from a camera.

Currently it only supports cameras which render to a render texture.

## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.1           | 0.13         |

## Installation

### Using git URL in Cargo.toml
```toml
[dependencies.bevy_framebuffer_extract]
git = "https://github.com/exvacuum/bevy_framebuffer_extract.git"
```

## Usage

In `main.rs`:
```rs
use bevy::prelude::*;
use bevy_framebuffer_extract;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_framebuffer_extract::FramebufferExtractPlugin,
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
        format: TextureFormat::R8Unorm,
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

let image_handle = images.add(image); // ResMut<Assets<Image>>

commands.spawn((
    Camera3dBundle {
        camera: Camera {
            target: image_handle.clone().into();
            ..Default::default()
        },
        ..Default::default()
    },
    bevy_framebuffer_extract::ExtractFramebufferBundle {
        source: framebuffer_extract_sources.add(FramebufferExtractSource(image_handle.clone())), // ResMut<Assets<FramebufferExtractSource>>
        destination: FramebufferExtractDestination::default(),
    },
));
```

The FramebufferExtractDestination component will contain the extracted image which can be used or saved for whatever you need.
