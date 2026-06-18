mod server;

#[cfg(test)]
mod server_test;

fn main() -> std::io::Result<()> {
    server::run()
}
