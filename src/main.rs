mod commands;
mod dashboard;
mod id_gen;
mod models;
mod parser;
mod serializer;
mod tags;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use chrono::NaiveDateTime;
use models::Priority;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Manage project tasks in TODO.md")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new TODO.md file
    Init {
        /// Force overwrite if file exists
        #[arg(long)]
        force: bool,
    },
    /// Add a task, actor, or comment
    Add {
        /// Add a task with description
        #[arg(long)]
        task: Option<String>,
        /// Tags to assign to the task (comma-separated)
        #[arg(long)]
        tag: Vec<String>,
        /// Actor IDs to assign to the task (comma-separated)
        #[arg(long)]
        actors: Option<String>,
        /// Add an actor (provide pseudo)
        #[arg(long)]
        actor: Option<String>,
        /// Picture path/URL for the actor
        #[arg(long)]
        pic: Option<String>,
        /// Add a comment with text
        #[arg(long)]
        comment: Option<String>,
        /// Task ID to attach the comment to (required with --comment)
        #[arg(long)]
        task_id: Option<String>,
        /// Priority: low, medium, high
        #[arg(long)]
        priority: Option<String>,
        /// Due date for the task (YYYY-MM-DD HH:MM)
        #[arg(long)]
        due: Option<String>,
    },
    /// List tasks, actors, or comments
    List {
        #[arg(long)]
        tasks: bool,
        #[arg(long)]
        actors: bool,
        #[arg(long)]
        comments: bool,
        /// Filter tasks by tags (comma-separated)
        #[arg(long)]
        tag: Vec<String>,
        /// Filter by priority: low, medium, high
        #[arg(long)]
        priority: Option<String>,
        /// Filter tasks by search query
        #[arg(long)]
        search: Option<String>,
        /// Show only overdue tasks
        #[arg(long)]
        overdue: bool,
    },
    /// Update fields by ID
    Update {
        /// ID of the item
        id: String,
        /// New description (for tasks)
        #[arg(long)]
        description: Option<String>,
        /// New due date (for tasks)
        #[arg(long)]
        due: Option<String>,
        /// New actor IDs (for tasks, comma-separated)
        #[arg(long)]
        actors: Option<String>,
        /// New comment IDs (for tasks, comma-separated)
        #[arg(long)]
        comments: Option<String>,
        /// New tags (for tasks, comma-separated)
        #[arg(long)]
        tag: Option<String>,
        /// New pseudo (for actors)
        #[arg(long)]
        name: Option<String>,
        /// New pic URL/path (for actors)
        #[arg(long)]
        pic: Option<String>,
        /// New text (for comments)
        #[arg(long)]
        text: Option<String>,
        /// New priority: low, medium, high
        #[arg(long)]
        priority: Option<String>,
    },
    /// Delete an item by ID
    Delete {
        /// ID of the item to delete
        id: String,
    },
    /// Install todo in PATH
    Install,
    /// Generate shell completion scripts
    Completion {
        shell: Shell,
    },
    /// Start web dashboard
    Dashboard,
    /// List all tags with task counts
    Tags,
    /// Search tasks by query
    Search {
        query: String,
    },
    /// Change task status
    Status {
        /// ID of the task
        id: String,
        /// New status: todo, en-cours, done, bloqued
        #[arg(long)]
        set: String,
        /// Reason for blocked status
        #[arg(long)]
        reason: Option<String>,
    },
}

fn main() {
    if std::env::args().len() <= 1 {
        if commands::is_installed() {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            return;
        }
        let result = commands::install();
        if let Err(e) = result {
            eprintln!("Error: {}", e);
        }
        println!("  Press Enter to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
        return;
    }

    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init { force } => commands::init(*force),
        Commands::Add { task, actors, actor, pic, comment, task_id, tag, priority, due } => {
            if let Some(desc) = task {
                let actor_ids: Vec<String> = actors.as_ref()
                    .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
                    .unwrap_or_default();
                let prio = priority.as_deref().and_then(Priority::from_str);
                let due_date = due.as_deref().and_then(|d| NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M").ok());
                commands::add_task(desc, &actor_ids, tag, prio, due_date, None)
            } else if let Some(pseudo) = actor {
                commands::add_actor(Some(pseudo), pic.as_deref())
            } else if let Some(text) = comment {
                match task_id.as_deref() {
                    Some(tid) => commands::add_comment(text, tid, &[]),
                    None => Err("--task-id is required for comments".to_string()),
                }
            } else {
                Err("Use --task, --actor, or --comment".to_string())
            }
        }
        Commands::List { tasks, actors, comments, tag, priority, search, overdue } => {
            let prio_filter = priority.as_deref().and_then(Priority::from_str);
            commands::list(*tasks, *actors, *comments, tag, prio_filter, search.as_deref(), *overdue)
        }
        Commands::Update { id, description, due, actors, comments, name, pic, text, tag, priority } => {
            commands::update(id, description.as_deref(), due.as_deref(), name.as_deref(), text.as_deref(), actors.as_deref(), comments.as_deref(), pic.as_deref(), tag.as_deref(), priority.as_deref(), None, None)
        }
        Commands::Install => commands::install(),
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(*shell, &mut cmd, "todo", &mut std::io::stdout());
            Ok(())
        }
        Commands::Dashboard => dashboard::serve(8383),
        Commands::Tags => commands::list_tags(),
        Commands::Search { query } => commands::search(query),
        Commands::Delete { id } => commands::delete(id),
        Commands::Status { id, set, reason } => {
            commands::set_status(id, set, reason.as_deref())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
