use bevy::asset::{Assets, Handle};
use bevy::ecs::prelude::{Query, Res, ResMut};
use bevy::math::Vec2;
use bevy::render::prelude::Visibility;
use bevy::render::texture::{Image, DEFAULT_IMAGE_HANDLE};
use bevy::render::Extract;
use bevy::sprite::{ColorMaterial, Rect};
use bevy::transform::prelude::GlobalTransform;
use bevy::ui::{CalculatedClip, ExtractedUiNode, ExtractedUiNodes, Node};

#[allow(clippy::type_complexity)]
pub fn extract_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    materials: Extract<Res<Assets<ColorMaterial>>>,
    images: Extract<Res<Assets<Image>>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &GlobalTransform,
            &Handle<ColorMaterial>,
            &Visibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    for (node, transform, handle, visibility, clip) in uinode_query.iter() {
        if !visibility.is_visible {
            continue;
        }
        let material = materials.get(handle).cloned().unwrap_or_default();
        let (color, image) = (
            material.color,
            material
                .texture
                .unwrap_or_else(|| DEFAULT_IMAGE_HANDLE.typed()),
        );
        // Skip loading images
        if !images.contains(&image) {
            continue;
        }
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            transform: transform.compute_matrix(),
            color,
            rect: Rect {
                min: Vec2::ZERO,
                max: node.size,
            },
            image,
            atlas_size: None,
            clip: clip.map(|clip| clip.clip),
        });
    }
}
