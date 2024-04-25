use bevy::{prelude::*, render::{render_asset::RenderAssetPlugin, extract_component::ExtractComponentPlugin, RenderApp, Render, RenderSet, render_graph::RenderGraph, graph::CameraDriverLabel}};
use components::FramebufferExtractDestination;
use nodes::{FramebufferExtractNode, FramebufferExtractLabel};
use render_assets::FramebufferExtractSource;

pub mod components;
mod systems;
mod nodes;
pub mod render_assets;


pub enum FramebufferExtractSet {
    Set
}

pub struct FramebufferExtractPlugin;

impl Plugin for FramebufferExtractPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<FramebufferExtractSource>()
            .init_asset::<FramebufferExtractSource>()
            .register_asset_reflect::<FramebufferExtractSource>()
            .add_plugins((
                RenderAssetPlugin::<FramebufferExtractSource>::default(),
                ExtractComponentPlugin::<FramebufferExtractDestination>::default(),
            ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(
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
