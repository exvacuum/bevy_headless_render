use std::sync::{Mutex, Arc};

use bevy::{prelude::*, render::extract_component::ExtractComponent, ecs::query::QueryItem};

use crate::render_assets::FramebufferExtractSource;

#[derive(Component, Default, Clone)]
pub struct FramebufferExtractDestination(pub Arc<Mutex<Image>>);

impl ExtractComponent for FramebufferExtractDestination {
    type QueryData = (
        &'static Self,
        &'static Handle<FramebufferExtractSource>,
    );

    type QueryFilter = ();

    type Out = (Self, Handle<FramebufferExtractSource>);

    fn extract_component((destination, source_handle): QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some((
            destination.clone(),
            source_handle.clone(),
        ))
    }
}

#[derive(Bundle)]
pub struct ExtractFramebufferBundle {
    pub source: Handle<FramebufferExtractSource>,
    pub dest: FramebufferExtractDestination,
}

