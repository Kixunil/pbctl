#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = std::env::var_os("OUT_DIR").unwrap();

    let mut app = build_cli();

    app.gen_completions("pbctl", Shell::Bash, outdir);
}
