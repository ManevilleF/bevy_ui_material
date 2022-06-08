//! # bevy_ui_material
//!
//! Materials for bevy UI Nodes
//!
//! [![workflow](https://github.com/ManevilleF/bevy_ui_material/actions/workflows/rust.yaml/badge.svg)](https://github.com/ManevilleF/bevy_ui_material/actions/workflows/rust.yaml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_ui_material.svg)](https://crates.io/crates/bevy_ui_material)
//! [![Docs.rs](https://docs.rs/bevy_ui_material/badge.svg)](https://docs.rs/bevy_ui_material)
//! [![dependency status](https://deps.rs/crate/bevy_ui_material/0.2.0/status.svg)](https://deps.rs/crate/bevy_ui_material)
//!
//! This [bevy] plugin changes the `bevy_ui` implementation using a material.
//!
//! > You might be interested in [bevy_sprite_material](https://github.com/ManevilleF/bevy_sprite_material) which is a similar plugin for `bevy_sprite` instead of `bevy_ui`.
//!
//! This plugin provides new implementation of the following bundles:
//! - `NodeBundle`
//! - `ButtonBundle`
//! - `ImageBundle`
//!
//! The new component bundles replaces the `color` field and the `image` field (`Handle<Image>`) by a `material` field (`Handle<ColorMaterial>`)
//!
//! ## Objective
//!
//! The goal of this plugin is to allow seamless edition of UI nodes `image` **and** `color` which was removed with [bevy] 0.6.
//!
//! This is very useful if you have many nodes and you have, for example, various themes and don't want to *query* every node to change its color.
//!
//! If you have a dedicated artist, you probably don't use the `color` tinting field anyway, so the base implementation is perfect for you.
//! This is specifically if you want to "massively update" the `color` and maybe the `image` as well.
//!
//! ## Disclaimer
//!
//! This plugin is very straightforward, and simply plugs itself in the `bevy_ui` render pipeline (in the *extraction* stage).
//! This system might be slower than the base implementation, because of the extra `Handle` involved.
//!
//! Also, there might be compatibility issues, so feel free to open issues or merge requests.
//!
//! > This plugin should work fine if you use both the plugin and the base ui implementation
//!
//! [bevy]: https://github.com/bevyengine/bevy
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::nursery,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]

pub use bundle::*;

mod bundle;
mod extract;

use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;
use bevy::render::{RenderApp, RenderStage};
use bevy::ui::RenderUiSystem::ExtractNode;

/// Plugin ot use UI Nodes with materials
///
/// Requires [`bevy::ui::UiPlugin`]
#[derive(Default)]
pub struct UiMaterialPlugin;

impl Plugin for UiMaterialPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_system_to_stage(
                RenderStage::Extract,
                extract::extract_uinodes.after(ExtractNode),
            );
        }
    }
}
