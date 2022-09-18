use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_ggrs::{GGRSPlugin, SessionType};
use ggrs::{Config, SessionBuilder};
use matchbox_socket::WebRtcSocket;

use crate::{
    domain::{AppState, LobbyText, LobbyUI},
    player::move_player,
};

struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = crate::input::InputSet;
    type State = u8;
    type Address = String;
}
const FPS: usize = 30;
const ROLLBACK_DEFAULT: &str = "rollback_default";

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/next_2";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    let task_pool = IoTaskPool::get();
    task_pool.spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn lobby_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::rgb(0.43, 0.41, 0.38).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Entering lobby...",
                        TextStyle {
                            font: asset_server.load("fonts/quicksand-light.ttf"),
                            font_size: 96.,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(LobbyText);
        })
        .insert(LobbyUI);
}

fn lobby(
    mut app_state: ResMut<State<AppState>>,
    mut socket: ResMut<Option<WebRtcSocket>>,
    mut commands: Commands,
) {
    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        return;
    }

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    let socket = socket.take().unwrap();
    let players = socket.players();

    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(12)
        .with_input_delay(2)
        .with_fps(FPS)
        .expect("invalid fps");

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let session = session_builder.start_p2p_session(socket);

    commands.insert_resource(session);
    commands.insert_resource(SessionType::P2PSession);

    app_state
        .set(AppState::InGame)
        .expect("Tried to go in-game while already in-game");
}

fn lobby_cleanup(query: Query<Entity, With<LobbyUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct OnlinePlugin;
impl Plugin for OnlinePlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(crate::input::input)
            .register_rollback_type::<Transform>()
            .with_rollback_schedule(Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel().with_system(move_player),
            ))
            .build(app);

        app.add_state(AppState::Lobby)
            .add_system_set(
                SystemSet::on_enter(AppState::Lobby)
                    .with_system(lobby_startup)
                    .with_system(start_matchbox_socket),
            )
            .add_system_set(SystemSet::on_update(AppState::Lobby).with_system(lobby))
            .add_system_set(SystemSet::on_exit(AppState::Lobby).with_system(lobby_cleanup));
    }
}
