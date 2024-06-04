#![warn(missing_docs)]

//! Plugin for the Bevy game engine which provides the ability to extract the frambuffer image after rendering
//! to use for whatever you want.

use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin, graph::CameraDriverLabel,
        render_asset::RenderAssetPlugin, render_graph::RenderGraph, Render, RenderApp, RenderSet,
    },
};
use components::FramebufferExtractDestination;
use nodes::{FramebufferExtractLabel, FramebufferExtractNode};
use render_assets::FramebufferExtractSource;

/// Components used by this plugin.
pub mod components;
/// Render assets used by this plugin.
pub mod render_assets;

mod nodes;
mod systems;

/// Plugin which handles framebuffer extraction.
pub struct FramebufferExtractPlugin;

impl Plugin for FramebufferExtractPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FramebufferExtractSource>()
            .init_asset::<FramebufferExtractSource>()
            .register_asset_reflect::<FramebufferExtractSource>()
            .add_plugins((
                RenderAssetPlugin::<FramebufferExtractSource>::default(),
                ExtractComponentPlugin::<FramebufferExtractDestination>::default(),
            ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            systems::extract_framebuffers
                .after(RenderSet::Render)
                .before(RenderSet::Cleanup),
        );
        let mut graph = render_app.world.resource_mut::<RenderGraph>();
        graph.add_node(FramebufferExtractLabel, FramebufferExtractNode);
        graph.add_node_edge(CameraDriverLabel, FramebufferExtractLabel);
    }
}
