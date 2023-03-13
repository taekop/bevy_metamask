use bevy::prelude::*;

use crate::{
    components::{EthereumStatus, MyButtonIndex, UserStatus},
    constants::*,
    ethereum::{Ethereum, EthereumStatusRequest, EthereumStatusResponse},
    user::{User, UserStatusRequest, UserStatusResponse},
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load(BUTTON_FONT),
                                font_size: BUTTON_FONT_SIZE,
                                color: BUTTON_TEXT_COLOR,
                            },
                        ),
                        EthereumStatus,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load(BUTTON_FONT),
                                font_size: BUTTON_FONT_SIZE,
                                color: BUTTON_TEXT_COLOR,
                            },
                        ),
                        UserStatus,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    let font = asset_server.load(BUTTON_FONT);
                    spawn_button(parent, 0, "Block Number", font.clone());
                    spawn_button(parent, 1, "Request Accounts", font.clone());
                    spawn_button(parent, 2, "Get Balance", font);
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn on_button(
    mut interaction_query: Query<
        (&Interaction, &MyButtonIndex, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut eth_event_writer: EventWriter<EthereumStatusRequest>,
    mut user_event_writer: EventWriter<UserStatusRequest>,
    user: Res<User>,
) {
    for (interaction, ind, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON_COLOR.into();
                match ind.0 {
                    0 => eth_event_writer.send(EthereumStatusRequest),
                    1 => user_event_writer.send(UserStatusRequest::Address),
                    2 => {
                        if let Some(address) = &user.address {
                            user_event_writer.send(UserStatusRequest::Balance(address.clone()));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn on_ethereum_status_response(
    mut ethereum: ResMut<Ethereum>,
    mut event_reader: EventReader<EthereumStatusResponse>,
) {
    for ev in event_reader.iter() {
        ethereum.block_number = Some(ev.block_number.clone());
    }
}

pub fn update_ethereum_status(
    ethereum: Res<Ethereum>,
    mut text_query: Query<&mut Text, With<EthereumStatus>>,
) {
    let mut text = text_query.get_single_mut().unwrap();
    text.sections[0].value = format!("{}", ethereum.as_ref());
}

pub fn on_user_status_response(
    mut user: ResMut<User>,
    mut event_reader: EventReader<UserStatusResponse>,
) {
    for ev in event_reader.iter() {
        match ev {
            UserStatusResponse::Address(address) => user.address = Some(address.clone()),
            UserStatusResponse::Balance(balance) => user.balance = Some(balance.clone()),
        }
    }
}

pub fn update_user_status(user: Res<User>, mut text_query: Query<&mut Text, With<UserStatus>>) {
    let mut text = text_query.get_single_mut().unwrap();
    text.sections[0].value = format!("{}", user.as_ref());
}

fn spawn_button(builder: &mut ChildBuilder, ind: usize, text: &str, font: Handle<Font>) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            MyButtonIndex(ind),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: BUTTON_FONT_SIZE,
                    color: BUTTON_TEXT_COLOR,
                },
            ));
        });
}
