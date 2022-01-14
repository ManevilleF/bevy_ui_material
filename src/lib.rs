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
use bevy_render::{RenderApp, RenderStage};

#[derive(Default)]
pub struct UiMaterialPlugin;

impl Plugin for UiMaterialPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_system_to_stage(RenderStage::Extract, extract::extract_uinodes);
        }
    }
}
