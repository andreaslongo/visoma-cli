use clap::{Args, Parser, Subcommand};
use std::process;
use visoma_cli::{run, Config};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Ticket(ticket) => match ticket.command {
            TicketCommands::New {
                server,
                user,
                password,
                title,
                description,
                customer_id,
                address_id,
            } => {
                let config = Config::new(
                    args.dry_run,
                    server,
                    user,
                    password,
                    title,
                    description,
                    customer_id,
                    address_id,
                );
                if let Err(e) = run(&config) {
                    println!("Application error: {e}");
                    process::exit(1);
                }
            }
        },
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Don't do anything, just show what would be done
    #[arg(short = 'n', long, global = true)]
    dry_run: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Ticket(TicketArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
//#[command(flatten_help = true)]
struct TicketArgs {
    #[command(subcommand)]
    command: TicketCommands,
}

#[derive(Debug, Subcommand)]
enum TicketCommands {
    /// Creates a new ticket
    New {
        /// Visoma server
        #[arg(short, long)]
        server: String,
        /// Visoma user
        #[arg(short, long)]
        user: String,
        /// Visoma password
        #[arg(short, long)]
        password: String,
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
