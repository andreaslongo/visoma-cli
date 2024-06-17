use clap::{Args, Parser, Subcommand};

fn main() {
    let _args = Cli::parse();
}



#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Don't do anything, just show what would be done
    #[arg(short='n', long, global=true)]
    dry_run: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct VisomaLogin {
    /// Visoma server
    #[arg(short, long)]
    server: String,
    /// Visoma user
    #[arg(short, long)]
    user: String,
    /// Visoma password
    #[arg(short, long)]
    password: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creates a new ticket
    #[command(arg_required_else_help = true)]
    Ticket {
        #[command(flatten)]
        visoma_login: VisomaLogin,
        /// Ticket title
        #[arg(long)]
        title: String,
        /// Ticket description
        #[arg(long)]
        description: String,
        /// Ticket customer ID
        #[arg(long)]
        customer_id: usize,
        /// Ticket customer address ID
        #[arg(long)]
        address_id: usize,
    },
}
