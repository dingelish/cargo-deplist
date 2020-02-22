# Changelog

## [1.4.1]

- Fix error when depth is specified and deps have been excluded.

## [1.4.0]

- Add --exclude option to remove dependencies from graphs.

## [1.3.0]

- Fix parsing of newer Cargo.lock files
- Move to structopt

## [1.2.0]

- Extract main logic into separate library.
- Add two functions to external API: `get_dep_graph` and `render_dep_graph`
- Enable colored help
- Forbid unsafe code and add badge
- Fix bugs with Cargo.toml detection

## [1.1.1]

- Update help output.

## [1.1.0]

- Add --depth option to limit the size of graphs.

## [1.0.4]

- Add --no-transitive-deps option to filter out edges of transitive deps.

## [1.0.3]

- First (working) release of cargo-deps on crates.io.
