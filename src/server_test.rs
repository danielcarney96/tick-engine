use osrs_engine::GameCommand;

use crate::server::{ServerInput, parse_server_input};

#[test]
fn parses_move_command() {
    assert_eq!(
        parse_server_input("move 10 4"),
        Ok(ServerInput::Command(GameCommand::MovePlayer {
            destination: (10, 4)
        }))
    );
}

#[test]
fn parses_run_commands() {
    assert_eq!(
        parse_server_input("run on"),
        Ok(ServerInput::Command(GameCommand::SetRun { enabled: true }))
    );
    assert_eq!(
        parse_server_input("run off"),
        Ok(ServerInput::Command(GameCommand::SetRun { enabled: false }))
    );
}

#[test]
fn parses_server_control_commands() {
    assert_eq!(parse_server_input("tick"), Ok(ServerInput::Tick));
    assert_eq!(parse_server_input("state"), Ok(ServerInput::State));
    assert_eq!(parse_server_input("help"), Ok(ServerInput::Help));
    assert_eq!(parse_server_input("quit"), Ok(ServerInput::Quit));
    assert_eq!(parse_server_input("exit"), Ok(ServerInput::Quit));
    assert_eq!(parse_server_input("   "), Ok(ServerInput::Empty));
}

#[test]
fn rejects_invalid_commands() {
    assert!(parse_server_input("move 10").is_err());
    assert!(parse_server_input("move x 4").is_err());
    assert!(parse_server_input("run maybe").is_err());
    assert!(parse_server_input("dance").is_err());
}
