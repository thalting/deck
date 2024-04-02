use compact_str::{format_compact, CompactString};
use deck::cli::Args;
use deck::config;
use smallvec::{smallvec, SmallVec};

use std::{
    io::{self, Read, Write},
    os::unix::net::UnixListener,
    process::{Command, Output},
};

fn write_all(output: Output) -> std::io::Result<()> {
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let config = config::get()?;

    let args: Args = argh::from_env();
    let name = args.name;

    let socket_path: CompactString = format_compact!("/tmp/deck-{name}");
    if std::fs::metadata(&*socket_path).is_ok() {
        std::fs::remove_file(&*socket_path)?;
    }

    let (cmd, args) = config.get(&name).unwrap();
    let listener = UnixListener::bind(&*socket_path)?;
    let mut msg: SmallVec<[u8; 24]> = smallvec![0; 24];
    loop {
        let (mut stream, _address) = listener.accept()?;
        stream.read(&mut msg)?;

        if name.as_bytes() == &msg[..name.len()] {
            write_all(Command::new(cmd).args(args.as_deref().unwrap_or_else(|| &[])).output()?)?;
        }
    }
}
