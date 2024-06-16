use clap::{Args, Parser, Subcommand};

fn main() {
    let _args = Cli::parse();
}

#[derive(Debug, Args)]
struct VisomaLogin {
    /// Visoma hostname
    #[arg(short, long)]
    server: String,
    /// Visoma user
    #[arg(short, long)]
    user: String,
    /// Visoma password
    #[arg(short, long)]
    password: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creates a new ticket
    #[command(arg_required_else_help = true)]
    Ticket {
        #[command(flatten)]
        visoma_login: VisomaLogin,
        /// Ticket title
        #[arg(short, long)]
        title: String,
        /// Ticket description
        #[arg(short, long)]
        description: String,
        /// Ticket customer ID
        #[arg(short, long)]
        customer_id: usize,
        /// Ticket customer address ID
        #[arg(short, long)]
        address_id: usize,
    },
}
