use bevy::prelude::*;

pub struct Plugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    LoadingScreen,
    MainMenu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Shop,
    Battle,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerTurnState {
    LocalPlayerTurn(Player),
    ComputerPlayerTurn(Player),
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player;
