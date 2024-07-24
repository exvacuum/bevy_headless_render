use bevy::{
    prelude::*,
    render::{
        render_asset::{RenderAssetUsages, RenderAssets},
        render_resource::{Extent3d, Maintain, MapMode, TextureDimension},
        renderer::RenderDevice,
    },
};

use pollster::FutureExt;

use crate::{components::HeadlessRenderDestination, render_assets::{HeadlessRenderSource, GpuHeadlessRenderSource}};

pub fn copy_buffers(
    mut headless_render_query: Query<(
        &Handle<HeadlessRenderSource>,
        &mut HeadlessRenderDestination,
    )>,
    sources: Res<RenderAssets<GpuHeadlessRenderSource>>,
    device: Res<RenderDevice>,
) {
    for (source_handle, destination_handle) in headless_render_query.iter_mut() {
        let Some(gpu_source) = sources.get(source_handle) else {
            continue;
        };

        let mut image_bytes = {
            let slice = gpu_source.buffer.slice(..);

            {
                let (tx, rx) = oneshot::channel();
                device.map_buffer(&slice, MapMode::Read, move |res| {
                    tx.send(res).unwrap();
                });
                device.poll(Maintain::Wait);
                rx.block_on().unwrap().unwrap();
            }

            slice.get_mapped_range().to_vec()
        };

        gpu_source.buffer.unmap();

        let bytes_per_row = gpu_source.bytes_per_row as usize;
        let padded_bytes_per_row = gpu_source.padded_bytes_per_row as usize;
        let source_size = gpu_source.source_size;
        let destination_handle = destination_handle.clone();
        let source_format = gpu_source.format;

        std::thread::spawn(move || {
            if bytes_per_row != padded_bytes_per_row {
                let mut unpadded_bytes =
                    Vec::<u8>::with_capacity(source_size.height as usize * bytes_per_row);
                for padded_row in image_bytes.chunks(padded_bytes_per_row) {
                    unpadded_bytes.extend_from_slice(&padded_row[..bytes_per_row]);
                }
                image_bytes = unpadded_bytes;
            }

            *destination_handle.0.lock().unwrap() = Image::new(
                Extent3d {
                    width: source_size.width,
                    height: source_size.height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                image_bytes,
                source_format,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );
        });
    }
}
