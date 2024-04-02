use compact_str::format_compact;
use deck::cli::Args;

use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();
    let name = args.name;

    let socket_path = format_compact!("/tmp/deck-{name}");
    let mut stream = UnixStream::connect(&*socket_path)?;
    stream.write(name.as_bytes())?;

    Ok(())
}
