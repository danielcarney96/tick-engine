use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use osrs_engine::{
    EngineRequest, EngineResponse, EngineRuntime, GameCommand, GameState, Npc, Position, TickResult,
};

const DEFAULT_ADDR: &str = "127.0.0.1:43594";
const TICK_RATE: Duration = Duration::from_millis(600);

pub fn run() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    run_at(&addr)
}

fn run_at(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let (request_tx, request_rx) = mpsc::channel::<ServerRequest>();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => spawn_client_handler(stream, request_tx.clone()),
                Err(error) => eprintln!("failed to accept client: {error}"),
            }
        }
    });

    let mut game_state = GameState::new();
    game_state.add_npc(Npc::new(1, (5, 0), 10));
    let mut runtime = EngineRuntime::new(game_state);
    let mut next_tick = Instant::now() + TICK_RATE;

    println!("engine server listening on {addr}");
    println!("tick rate: {:?}", TICK_RATE);
    println!("{}", format_state(runtime.state()));

    loop {
        while let Ok(request) = request_rx.try_recv() {
            handle_request(request, &mut runtime);
        }

        let now = Instant::now();
        if now >= next_tick {
            log_tick(runtime.tick());
            next_tick += TICK_RATE;
            continue;
        }

        match request_rx.recv_timeout(next_tick - now) {
            Ok(request) => handle_request(request, &mut runtime),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

#[derive(Debug)]
struct ServerRequest {
    input: ServerInput,
    response_tx: mpsc::Sender<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ServerInput {
    Command(GameCommand),
    Tick,
    State,
    Help,
    Quit,
    Empty,
}

pub(crate) fn parse_server_input(input: &str) -> Result<ServerInput, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let Some(command) = parts.first().copied() else {
        return Ok(ServerInput::Empty);
    };

    match command {
        "move" => parse_move_command(&parts),
        "run" => parse_run_command(&parts),
        "tick" => Ok(ServerInput::Tick),
        "state" => Ok(ServerInput::State),
        "help" => Ok(ServerInput::Help),
        "quit" | "exit" => Ok(ServerInput::Quit),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn parse_move_command(parts: &[&str]) -> Result<ServerInput, String> {
    if parts.len() != 3 {
        return Err("usage: move <x> <y>".to_string());
    }

    let destination = parse_position(parts[1], parts[2])?;
    Ok(ServerInput::Command(GameCommand::MovePlayer {
        destination,
    }))
}

fn parse_run_command(parts: &[&str]) -> Result<ServerInput, String> {
    if parts.len() != 2 {
        return Err("usage: run <on|off>".to_string());
    }

    let enabled = match parts[1] {
        "on" => true,
        "off" => false,
        value => return Err(format!("invalid run mode: {value}; expected on or off")),
    };

    Ok(ServerInput::Command(GameCommand::SetRun { enabled }))
}

fn parse_position(x: &str, y: &str) -> Result<Position, String> {
    let x = x
        .parse::<u32>()
        .map_err(|_| format!("invalid x coordinate: {x}"))?;
    let y = y
        .parse::<u32>()
        .map_err(|_| format!("invalid y coordinate: {y}"))?;

    Ok((x, y))
}

fn spawn_client_handler(stream: TcpStream, request_tx: mpsc::Sender<ServerRequest>) {
    thread::spawn(move || {
        if let Err(error) = handle_client(stream, request_tx) {
            eprintln!("client disconnected with error: {error}");
        }
    });
}

fn handle_client(mut stream: TcpStream, request_tx: mpsc::Sender<ServerRequest>) -> io::Result<()> {
    writeln!(stream, "connected to osrs-engine")?;

    let mut reader = BufReader::new(stream.try_clone()?);
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        let should_quit = matches!(parse_server_input(&line), Ok(ServerInput::Quit));
        let response = dispatch_line(&line, &request_tx);
        writeln!(stream, "{response}")?;

        if should_quit {
            break;
        }
    }

    Ok(())
}

fn dispatch_line(line: &str, request_tx: &mpsc::Sender<ServerRequest>) -> String {
    let input = match parse_server_input(line) {
        Ok(input) => input,
        Err(error) => return format!("error: {error}"),
    };

    let (response_tx, response_rx) = mpsc::channel();
    let request = ServerRequest { input, response_tx };

    if request_tx.send(request).is_err() {
        return "error: engine server is not running".to_string();
    }

    response_rx
        .recv()
        .unwrap_or_else(|_| "error: engine server did not respond".to_string())
}

fn handle_request(request: ServerRequest, runtime: &mut EngineRuntime) {
    let response = match request.input {
        ServerInput::Command(command) => {
            format_response(runtime.handle_request(EngineRequest::Command(command)))
        }
        ServerInput::Tick => format_response(runtime.handle_request(EngineRequest::Tick)),
        ServerInput::State => format_response(runtime.handle_request(EngineRequest::GetState)),
        ServerInput::Help => help_text(),
        ServerInput::Quit => "bye".to_string(),
        ServerInput::Empty => String::new(),
    };

    let _ = request.response_tx.send(response);
}

fn log_tick(result: TickResult) {
    if !result.events.is_empty() {
        println!("tick events: {:?}", result.events);
    }
}

fn format_response(response: EngineResponse) -> String {
    match response {
        EngineResponse::Queued => "queued".to_string(),
        EngineResponse::Tick(result) => format_tick_result(result),
        EngineResponse::State(state) => format_state(&state),
    }
}

fn format_tick_result(result: TickResult) -> String {
    if result.events.is_empty() {
        "events: []".to_string()
    } else {
        format!("events: {:?}", result.events)
    }
}

fn format_state(state: &GameState) -> String {
    format!(
        "player: position={:?}, destination={:?}, hp={}, running={}; npcs: {:?}",
        state.player.position,
        state.player.movement_destination,
        state.player.health,
        state.player.is_running,
        state.npcs
    )
}

fn help_text() -> String {
    "commands: move <x> <y>; run <on|off>; tick; state; help; quit".to_string()
}
