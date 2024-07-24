use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraphContext, RenderLabel},
        render_resource::{ImageCopyBuffer, ImageDataLayout},
        renderer::RenderContext, texture::GpuImage,
    },
};

use crate::render_assets::GpuHeadlessRenderSource;

#[derive(RenderLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct HeadlessRenderCopyLabel;

#[derive(Default)]
pub struct HeadlessRenderCopyNode;

impl Node for HeadlessRenderCopyNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for (_, source) in world
            .resource::<RenderAssets<GpuHeadlessRenderSource>>()
            .iter()
        {
            let Some(gpu_image) = world
                .resource::<RenderAssets<GpuImage>>()
                .get(source.source_handle.id())
            else {
                return Ok(());
            };

            render_context.command_encoder().copy_texture_to_buffer(
                gpu_image.texture.as_image_copy(),
                ImageCopyBuffer {
                    buffer: &source.buffer,
                    layout: ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(source.padded_bytes_per_row),
                        rows_per_image: None,
                    },
                },
                source.source_size,
            );
        }
        Ok(())
    }
}
