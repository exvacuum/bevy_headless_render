use std::sync::{Arc, Mutex};

use bevy::{ecs::query::QueryItem, prelude::*, render::extract_component::ExtractComponent};

use crate::render_assets::HeadlessRenderSource;

/// Headless render destination. Contains the image which the rendered frame is copied to.
#[derive(Component, Default, Clone)]
pub struct HeadlessRenderDestination(pub Arc<Mutex<Image>>);

impl ExtractComponent for HeadlessRenderDestination {
    type QueryData = (&'static Self, &'static Handle<HeadlessRenderSource>);

    type QueryFilter = ();

    type Out = (Self, Handle<HeadlessRenderSource>);

    fn extract_component(
        (destination, source_handle): QueryItem<'_, Self::QueryData>,
    ) -> Option<Self::Out> {
        Some((destination.clone(), source_handle.clone()))
    }
}

/// Bundle containing both a source and destination for headless rendering.
#[derive(Bundle)]
pub struct HeadlessRenderBundle {
    /// Source
    pub source: Handle<HeadlessRenderSource>,
    /// Destination
    pub dest: HeadlessRenderDestination,
}
