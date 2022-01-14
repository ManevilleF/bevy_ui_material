use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::{Query, Res, ResMut};
use bevy_math::Vec2;
use bevy_render::prelude::Visibility;
use bevy_render::texture::DEFAULT_IMAGE_HANDLE;
use bevy_render::RenderWorld;
use bevy_sprite::ColorMaterial;
use bevy_transform::prelude::GlobalTransform;
use bevy_ui::{CalculatedClip, ExtractedUiNode, ExtractedUiNodes, Node};

#[allow(clippy::type_complexity)]
pub fn extract_uinodes(
    mut render_world: ResMut<RenderWorld>,
    materials: Res<Assets<ColorMaterial>>,
    uinode_query: Query<(
        &Node,
        &GlobalTransform,
        &Handle<ColorMaterial>,
        &Visibility,
        Option<&CalculatedClip>,
    )>,
) {
    let mut extracted_uinodes = render_world.get_resource_mut::<ExtractedUiNodes>().unwrap();
    extracted_uinodes.uinodes.clear();
    for (uinode, transform, handle, visibility, clip) in uinode_query.iter() {
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
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            transform: transform.compute_matrix(),
            color,
            rect: bevy_sprite::Rect {
                min: Vec2::ZERO,
                max: uinode.size,
            },
            image,
            atlas_size: None,
            clip: clip.map(|clip| clip.clip),
        });
    }
}
