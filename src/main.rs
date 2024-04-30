#![allow(unused_variables)]
#![allow(dead_code)]
mod asset_loader;
mod asteroid;
mod camera;
mod debug;
mod movement;
mod spaceship;

use ::bevy::prelude::*;
use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use camera::CameraPlugin;
//use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        // User Configured plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        //.add_plugins(DebugPlugin)
        .run();
}
