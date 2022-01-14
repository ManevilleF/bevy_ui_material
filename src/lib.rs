//! # bevy_ui_material
//!
//! Materials for bevy UI Nodes
//!
//! [![workflow](https://github.com/ManevilleF/bevy_ui_material/actions/workflows/rust.yaml/badge.svg)](https://github.com/ManevilleF/bevy_ui_material/actions/workflows/rust.yaml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style,
    rustdoc::broken_intra_links
)]

pub use bundle::*;

mod bundle;
mod extract;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_render::{RenderApp, RenderStage};
use bevy_ui::RenderUiSystem::ExtractNode;

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
