use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "workq",
    version = "v0.1.0",
    about = "Manage temporary workspaces",
    author = "Penpen7, Raosys@live.jp"
)]

struct Cli {
    #[clap(subcommand)]
    sub_command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(about = "List workspaces")]
    List,
    #[clap(about = "Create a new workspace")]
    Create,
}

fn main() {
    let cli = Cli::parse();
    match cli.sub_command {
        SubCommands::List => workq::list().map_err(|e| eprintln!("{:?}", e)).is_ok(),
        SubCommands::Create => workq::create().map_err(|e| eprintln!("{:?}", e)).is_ok(),
    };
}
