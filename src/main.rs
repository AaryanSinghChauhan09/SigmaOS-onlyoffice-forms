#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "onlyoffice-forms", about = "SigmaOS OnlyOffice forms editor")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("onlyoffice-forms: no entries configured."),
        Commands::Status => println!("onlyoffice-forms: ready."),
    }
}
