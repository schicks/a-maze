use bevy::prelude::*;

#[derive(Component)]
pub struct LobbyText;
#[derive(Component)]
pub struct LobbyUI;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Lobby,
    InGame,
}
