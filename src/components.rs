use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use bevy::{ecs::query::QueryItem, prelude::*, render::extract_component::ExtractComponent};

use crate::render_assets;

/// Headless render destination. Contains the image which the rendered frame is copied to.
#[derive(Component, Default, Clone)]
pub struct HeadlessRenderDestination(pub Arc<Mutex<Image>>);

impl ExtractComponent for HeadlessRenderDestination {
    type QueryData = (&'static Self, &'static HeadlessRenderSource);

    type QueryFilter = ();

    type Out = (Self, HeadlessRenderSource);

    fn extract_component(
        (destination, source_handle): QueryItem<'_, Self::QueryData>,
    ) -> Option<Self::Out> {
        Some((destination.clone(), source_handle.clone()))
    }
}

impl HeadlessRenderDestination {
    /// Get lock on this destination's image
    pub fn image(&self) -> Result<MutexGuard<Image>, PoisonError<MutexGuard<Image>>> {
        self.0.lock()
    }
}

/// Headless render source
#[derive(Component, Debug, Clone, DerefMut, Deref)]
#[require(HeadlessRenderDestination, Camera3d)]
pub struct HeadlessRenderSource(pub Handle<render_assets::HeadlessRenderSource>);

impl HeadlessRenderSource {
    /// Create a new headless render source from the provided image
    pub fn new(asset_server: &AssetServer, image: Handle<Image>) -> Self {
        Self(asset_server.add(render_assets::HeadlessRenderSource(image)))
    }
}
