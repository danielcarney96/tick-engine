use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

const DEFAULT_ADDR: &str = "127.0.0.1:43594";

fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    let mut stream = TcpStream::connect(&addr)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let stdin = io::stdin();

    let mut greeting = String::new();
    reader.read_line(&mut greeting)?;
    print!("{greeting}");

    print_help();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        stream.write_all(line.as_bytes())?;
        stream.flush()?;

        let mut response = String::new();
        if reader.read_line(&mut response)? == 0 {
            eprintln!("server closed connection");
            break;
        }
        print!("{response}");

        let command = line.trim();
        if command == "quit" || command == "exit" {
            break;
        }
    }

    Ok(())
}

fn print_help() {
    println!("commands:");
    println!("  move <x> <y>");
    println!("  run <on|off>");
    println!("  attack <npc_id>");
    println!("  retaliate <on|off>");
    println!("  tick");
    println!("  state");
    println!("  help");
    println!("  quit");
}
