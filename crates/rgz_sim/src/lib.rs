mod gz;
mod app;

use bevy::prelude::Resource;
pub use app::run;

#[derive(Resource)]
struct AppArgs {
    world: String,
    running: bool,
}
