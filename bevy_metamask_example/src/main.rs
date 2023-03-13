use bevy::prelude::*;
use bevy_metamask::MetamaskPlugin;

mod components;
mod constants;
mod ethereum;
mod systems;
mod user;

use constants::*;
use ethereum::{Ethereum, EthereumStatusRequest};
use systems::*;
use user::{User, UserStatusRequest};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: "Bevy Metamask Example".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(on_button)
        // ethereum
        .insert_resource(Ethereum::default())
        .add_plugin(MetamaskPlugin::<EthereumStatusRequest>::new())
        .add_system(on_ethereum_status_response)
        .add_system(update_ethereum_status)
        // user
        .insert_resource(User::default())
        .add_plugin(MetamaskPlugin::<UserStatusRequest>::new())
        .add_system(on_user_status_response)
        .add_system(update_user_status)
        .run();
}
