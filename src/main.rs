//! Entry point for the command line program.

#![forbid(unsafe_code)]

use crate::args::{parse_args, Command};
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};
use structopt::StructOpt;

use cargo_deps_ding::{get_dep_graph, render_dep_graph, Result};

mod args;

fn main() {
    // Call the real entry point and handle any errors that occur.
    real_main().unwrap_or_else(|e| e.exit());
}

fn real_main() -> Result<()> {
    let command = Command::from_args();
    let Command::Deps(args) = command;

    let cfg = parse_args(args);
    let dot_file = cfg.dot_file.clone();

    // Get dependency graph & render it.
    let out = get_dep_graph(cfg).and_then(render_dep_graph)?;

    // Output to stdout or render the dot file.
    match dot_file {
        None => Box::new(io::stdout()) as Box<dyn Write>,
        Some(file) => Box::new(File::create(&Path::new(&file))?),
    }
    .write_all(&out.into_bytes())?;

    Ok(())
}
