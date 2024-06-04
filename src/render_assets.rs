use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetUsages, RenderAssets},
        render_resource::{Buffer, BufferDescriptor, BufferUsages, Extent3d, TextureFormat},
        renderer::RenderDevice,
    },
};

/// Render-world version of FramebufferExtractSource
pub struct GpuFramebufferExtractSource {
    pub(crate) buffer: Buffer,
    pub(crate) source_handle: Handle<Image>,
    pub(crate) source_size: Extent3d,
    pub(crate) bytes_per_row: u32,
    pub(crate) padded_bytes_per_row: u32,
    pub(crate) format: TextureFormat,
}

/// Framebuffer extraction source. Contains a handle to the render texture which will be extracted
/// from.
#[derive(Asset, Reflect, Clone, Default)]
pub struct FramebufferExtractSource(pub Handle<Image>);

impl RenderAsset for FramebufferExtractSource {
    type PreparedAsset = GpuFramebufferExtractSource;
    type Param = (SRes<RenderDevice>, SRes<RenderAssets<Image>>);

    fn asset_usage(&self) -> RenderAssetUsages {
        RenderAssetUsages::default()
    }

    fn prepare_asset(
        self,
        (device, images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
        let Some(gpu_image) = images.get(&self.0) else {
            warn!("Failed to get GPU image");
            return Err(PrepareAssetError::RetryNextUpdate(self));
        };

        let size = gpu_image.texture.size();
        let format = gpu_image.texture_format;
        let bytes_per_row =
            (size.width / format.block_dimensions().0) * format.block_copy_size(None).unwrap();
        let padded_bytes_per_row =
            RenderDevice::align_copy_bytes_per_row(bytes_per_row as usize) as u32;

        Ok(GpuFramebufferExtractSource {
            buffer: device.create_buffer(&BufferDescriptor {
                label: Some("framebuffer_extract_buffer"),
                size: (size.height * padded_bytes_per_row) as u64,
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            source_handle: self.0.clone(),
            source_size: size,
            bytes_per_row,
            padded_bytes_per_row,
            format,
        })
    }
}
