use bevy::{prelude::*, render::{render_graph::{Node, RenderGraphContext, NodeRunError, RenderLabel}, renderer::RenderContext, render_asset::RenderAssets, render_resource::{ImageCopyBuffer, ImageDataLayout}}};

use crate::render_assets::FramebufferExtractSource;

#[derive(RenderLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct FramebufferExtractLabel;

#[derive(Default)]
pub struct FramebufferExtractNode;

impl Node for FramebufferExtractNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for (_, source) in world.resource::<RenderAssets<FramebufferExtractSource>>().iter() {
            let Some(gpu_image) = world.resource::<RenderAssets<Image>>().get(&source.source_handle) else {
                return Ok(())
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
