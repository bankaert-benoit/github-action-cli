use clap::{ Parser, Subcommand };

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
        #[arg(short, long, default_value_t = String::from("."))]
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::List { path }) => {
            println!("Running list on path: {}", path)
        },
        None => {}
    }

}
