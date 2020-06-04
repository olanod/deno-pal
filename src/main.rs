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

#[paw::main]
fn main(args: Args) {
    match args {
        Args::Info => println!("Not much info for now"),
        Args::Init => println!("Doing ... nothing!"),
        Args::Add { name } => println!("Feeling lazy, {} is a nice name though", name),
    }
}
