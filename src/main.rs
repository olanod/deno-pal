#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{App, Arg};
use serde_json::{json, to_vec_pretty};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use which::which;

#[allow(dead_code)]
mod deno_flags;
// needed for deno_flags
pub(crate) mod version {
    pub const DENO: &str = "X";
    pub const TYPESCRIPT: &str = "Y";
    pub fn v8() -> &'static str {
        "Z"
    }
}

static DP_HELP: &str = "A small wrapper for Deno the secure JavaScript and TypeScript runtime.
It adds some opinionated bits on how to do dependency management.

Deno Docs: https://deno.land/manual

To start the Deno REPL:
  dp

To execute a script:
  dp run https://deno.land/std/examples/welcome.ts

To evaluate code in the shell:
  dp eval \"console.log(30933 + 404)\"
";

fn main() -> Result<()> {
    let app = deno_flags::clap_root();
    let matches = app
        .name("deno-pal")
        .bin_name("dp")
        .subcommand(App::new("init").about("Initializes the current dir as a deno-pal project"))
        .subcommand(
            App::new("add")
                .about("Adds a dependency to the project")
                .arg(
                    Arg::with_name("dependency")
                        .value_name("DEPENDENCY")
                        .required(true),
                ),
        )
        .long_about(DP_HELP)
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init()?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        add(matches.value_of("dependency").unwrap())?;
    } else {
        run_deno()?;
    }
    Ok(())
}

fn init() -> Result<()> {
    let imports_path = Path::new("import_map.json");
    if imports_path.exists() {
        return Ok(());
    }
    let imports = to_vec_pretty(&json!({
        "imports": {
            "@std": "https://deno.land/std/"
        }
    }))?;
    File::create(imports_path)?.write_all(&imports)?;
    Ok(())
}

fn add(name: &str) -> Result<()> {
    println!(
        "One day I will add {} as a dependency of your project ;)",
        name
    );
    Ok(())
}

fn run_deno() -> Result<()> {
    let deno = which("deno")?;
    Command::new(deno)
        .args(std::env::args().skip(1).collect::<Vec<String>>())
        .status()?;
    Ok(())
}
