mod db;

use clap::{Parser, Subcommand};

use db::{add_task, init_database, show_tasks,done_task};

#[derive(Parser)]
struct Cli {
    #[command[subcommand]]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },

    Show {},

    Done { task: String},
}

fn main() {
    let conn = init_database().expect("failed to connect to database");
    let args = Cli::parse();

    match args.command {
        Commands::Add { task } => {
            let task = lowercase(task);

            add_task(&conn, task).unwrap();
        }

        Commands::Show {} => {
            show_tasks(&conn).unwrap();
        }

        Commands::Done { task }=>{
            let task = lowercase(task);
            done_task(&conn,task).unwrap();
        }
    }
}

fn lowercase(task: String) -> String {
    task.trim().to_lowercase()
}
