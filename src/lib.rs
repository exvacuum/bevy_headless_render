#![warn(missing_docs)]

//! Plugin for the Bevy game engine which provides the ability to render to an image headlessly.

use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin, graph::CameraDriverLabel,
        render_asset::RenderAssetPlugin, render_graph::RenderGraph, Render, RenderApp, RenderSet,
    },
};
use components::HeadlessRenderDestination;
use nodes::{HeadlessRenderCopyLabel, HeadlessRenderCopyNode};
use render_assets::{HeadlessRenderSource, GpuHeadlessRenderSource};

/// Components used by this plugin.
pub mod components;
/// Render assets used by this plugin.
pub mod render_assets;

mod nodes;
mod systems;

/// Plugin which handles headless rendering.
pub struct HeadlessRenderPlugin;

impl Plugin for HeadlessRenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HeadlessRenderSource>()
            .init_asset::<HeadlessRenderSource>()
            .register_asset_reflect::<HeadlessRenderSource>()
            .add_plugins((
                RenderAssetPlugin::<GpuHeadlessRenderSource>::default(),
                ExtractComponentPlugin::<HeadlessRenderDestination>::default(),
            ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            systems::copy_buffers
                .after(RenderSet::Render)
                .before(RenderSet::Cleanup),
        );
        let mut graph = render_app.world_mut().resource_mut::<RenderGraph>();
        graph.add_node(HeadlessRenderCopyLabel, HeadlessRenderCopyNode);
        graph.add_node_edge(CameraDriverLabel, HeadlessRenderCopyLabel);
    }
}
