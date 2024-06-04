use std::sync::{Arc, Mutex};

use bevy::{ecs::query::QueryItem, prelude::*, render::extract_component::ExtractComponent};

use crate::render_assets::FramebufferExtractSource;

/// Framebuffer extraction destination. Contains the image which the framebuffer is extracted to.
#[derive(Component, Default, Clone)]
pub struct FramebufferExtractDestination(pub Arc<Mutex<Image>>);

impl ExtractComponent for FramebufferExtractDestination {
    type QueryData = (&'static Self, &'static Handle<FramebufferExtractSource>);

    type QueryFilter = ();

    type Out = (Self, Handle<FramebufferExtractSource>);

    fn extract_component(
        (destination, source_handle): QueryItem<'_, Self::QueryData>,
    ) -> Option<Self::Out> {
        Some((destination.clone(), source_handle.clone()))
    }
}

/// Bundle containing both a source and destination for framebuffer extraction.
#[derive(Bundle)]
pub struct ExtractFramebufferBundle {
    /// Source
    pub source: Handle<FramebufferExtractSource>,
    /// Destination
    pub dest: FramebufferExtractDestination,
}
