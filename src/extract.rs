use bevy::asset::{Assets, Handle};
use bevy::ecs::prelude::{Query, Res, ResMut};
use bevy::math::Vec2;
use bevy::render::prelude::Visibility;
use bevy::render::texture::{Image, DEFAULT_IMAGE_HANDLE};
use bevy::render::RenderWorld;
use bevy::sprite::{ColorMaterial, Rect};
use bevy::transform::prelude::GlobalTransform;
use bevy::ui::{CalculatedClip, ExtractedUiNode, ExtractedUiNodes, Node};

#[allow(clippy::type_complexity)]
pub fn extract_uinodes(
    mut render_world: ResMut<RenderWorld>,
    materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
    uinode_query: Query<(
        &Node,
        &GlobalTransform,
        &Handle<ColorMaterial>,
        &Visibility,
        Option<&CalculatedClip>,
    )>,
) {
    let mut extracted_uinodes = render_world.get_resource_mut::<ExtractedUiNodes>().unwrap();
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
