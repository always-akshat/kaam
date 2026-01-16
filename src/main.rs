mod commands;
mod storage;
mod task;

use clap::{Parser, Subcommand};
use task::{Priority, Status};

#[derive(Parser)]
#[command(name = "kaam")]
#[command(about = "A simple CLI todo application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
        /// Priority level (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
        /// Due date (YYYY-MM-DD format)
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List all tasks
    List {
        /// Filter by status (pending, done)
        #[arg(short, long)]
        status: Option<Status>,
        /// Filter by priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    /// Mark a task as done
    Done {
        /// Task ID to mark as done
        id: u32,
    },
    /// Edit an existing task
    Edit {
        /// Task ID to edit
        id: u32,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
        /// New due date (YYYY-MM-DD format)
        #[arg(long)]
        due: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task ID to delete
        id: u32,
    },
    /// Show detailed usage examples
    Usage,
}

fn show_help() {
    println!(r#"kaam - A simple CLI todo application

USAGE:
    kaam <COMMAND>

COMMANDS:
    add      Add a new task
    list     List all tasks
    done     Mark a task as done
    edit     Edit an existing task
    delete   Delete a task
    usage    Show this help message with examples

EXAMPLES:
    Add a task:
        kaam add "Buy groceries"
        kaam add "Finish report" --priority high
        kaam add "Call mom" --priority medium --due 2026-01-20

    List tasks:
        kaam list
        kaam list --status pending
        kaam list --status done
        kaam list --priority high

    Mark task as done:
        kaam done 1

    Edit a task:
        kaam edit 1 --description "Updated task"
        kaam edit 1 --priority low
        kaam edit 1 --due 2026-02-01

    Delete a task:
        kaam delete 1

PRIORITY LEVELS:
    low, medium, high

STATUS VALUES:
    pending, done

DATA STORAGE:
    Tasks are stored in ~/.kaam.json
"#);
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add {
            description,
            priority,
            due,
        } => commands::add_task(description, priority, due),
        Commands::List { status, priority } => commands::list_tasks(status, priority),
        Commands::Done { id } => commands::mark_done(id),
        Commands::Edit {
            id,
            description,
            priority,
            due,
        } => commands::edit_task(id, description, priority, due),
        Commands::Delete { id } => commands::delete_task(id),
        Commands::Usage => {
            show_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
