use async_std::{fs::File, io, path::Path, prelude::*};
use serde_json::{json, to_vec_pretty};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Deno's best friend")]
enum Args {
    /// Print useful information
    Info,
    /// Initializes the current dir as a deno project
    Init,
    /// Adds a dependency to the project
    Add { name: String },
}

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> io::Result<()> {
    match args {
        Args::Info => info().await?,
        Args::Init => init().await?,
        Args::Add { name } => add(&name).await?,
    }
    Ok(())
}

async fn init() -> io::Result<()> {
    let imports_path = Path::new("import_map.json");
    if imports_path.exists().await {
        return Ok(());
    }
    let imports = to_vec_pretty(&json!({
        "imports": {
            "@std": "https://deno.land/std/"
        }
    }))?;
    File::create(imports_path)
        .await?
        .write_all(&imports)
        .await?;
    Ok(())
}

async fn info() -> io::Result<()> {
    println!("Not much info for now");
    Ok(())
}
async fn add(name: &str) -> io::Result<()> {
    println!("Feeling lazy, {} is a nice name though", name);
    Ok(())
}
