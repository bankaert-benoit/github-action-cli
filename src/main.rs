use clap::{ Parser, Subcommand };
use list::list;
mod list;

/// TODO: Write documentation
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// List the workflows in the given directory
    List {
        #[arg(default_value_t = String::from("."))]
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::List { path }) => {
            list(path);
        },
        None => {}
    }

}
