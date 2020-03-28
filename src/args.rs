use std::{result::Result, str::FromStr};
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

use cargo_deps_ding::Config;

const USAGE: &str = "\
cargo-deps writes a graph in dot format to standard output.

    Typical usage is `cargo deps | dot -Tpng > graph.png`.";

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    #[structopt(name = "deps", usage = USAGE, author, setting(ColoredHelp))]
    /// Cargo subcommand for building dependency graphs of Rust projects.
    Deps(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "depth", short = "d", value_name = "DEPTH", parse(try_from_str = parse_depth))]
    /// The maximum dependency depth to display. The default is no limit
    pub depth: Option<usize>,

    #[structopt(long = "dot-file", short = "o", value_name = "PATH")]
    /// Output file, or stdout if not specified
    pub dot_file: Option<String>,

    #[structopt(long = "filter", value_name = "DEPNAMES")]
    /// Only display provided deps
    pub filter: Option<Vec<String>>,

    #[structopt(long = "exclude", value_name = "DEPNAMES")]
    /// Exclude provided deps
    pub exclude: Option<Vec<String>>,

    #[structopt(
        long = "manifest-path",
        value_name = "PATH",
        default_value = "Cargo.toml"
    )]
    /// Specify location of manifest file
    pub manifest_path: String,

    #[structopt(long = "subgraph", value_name = "DEPNAMES")]
    /// Group provided deps in their own subgraph
    pub subgraph: Option<Vec<String>>,

    #[structopt(long = "subgraph-name", value_name = "NAME", requires = "subgraph")]
    /// Optional name of subgraph
    pub subgraph_name: Option<String>,

    #[structopt(long = "all-deps")]
    /// Include all dependencies in the graph. Can be used with --no-regular-deps
    pub all_deps: bool,

    #[structopt(long = "build-deps")]
    /// Include build dependencies in the graph (purple)
    pub build_deps: bool,

    #[structopt(long = "dev-deps")]
    /// Include dev dependencies in the graph (blue)
    pub dev_deps: bool,

    #[structopt(long = "optional-deps")]
    /// Include optional dependencies in the graph (red)
    pub optional_deps: bool,

    #[structopt(long = "include-orphans")]
    /// Don't purge orphan nodes (yellow). This is useful in some workspaces
    pub include_orphans: bool,

    #[structopt(long = "include-versions", short = "I")]
    /// Include the dependency version on nodes
    pub include_versions: bool,

    #[structopt(long = "no-regular-deps")]
    /// Exclude regular dependencies from the graph
    pub no_regular_deps: bool,

    #[structopt(long = "no-transitive-deps")]
    /// Filter out edges that point to a transitive dependency
    pub no_transitive_deps: bool,
}

fn parse_depth(src: &str) -> Result<usize, String> {
    usize::from_str(src).map_err(|e| format!("'{}': {}", src, e))
}

pub fn parse_args(args: Args) -> Config {
    Config {
        depth: args.depth,
        dot_file: args.dot_file,
        filter: args.filter,
        exclude: args.exclude,
        include_orphans: args.include_orphans,
        include_versions: args.include_versions,
        manifest_path: args.manifest_path,
        subgraph: args.subgraph,
        subgraph_name: args.subgraph_name,

        regular_deps: !args.no_regular_deps,
        build_deps: args.all_deps || args.build_deps,
        dev_deps: args.all_deps || args.dev_deps,
        optional_deps: args.all_deps || args.optional_deps,
        transitive_deps: !args.no_transitive_deps,
    }
}
