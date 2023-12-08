use clap::{Parser, Subcommand};

mod commands;

/// Rust implementation of bash commands
#[derive(Debug, Parser)]
#[clap(name = "rush", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Copy files from a source to a target
    Cp(CpArgs),
    /// Move files from a source to a target
    Mv(MvArgs),
    /// Count files and directories in a target directory
    Count(CountArgs),
    /// Get images metadata
    Imagesum(ImagesumArgs),
    /// Get video metadata
    Videosum(VideosumArgs),
}

#[derive(Debug, Parser)]
pub struct CpArgs {
    /// Source directory or file
    #[arg(required = true)]
    source: String,

    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

#[derive(Debug, Parser)]
pub struct MvArgs {
    /// Source directory or file
    #[arg(required = true)]
    source: String,

    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

#[derive(Debug, Parser)]
pub struct CountArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

#[derive(Debug, Parser)]
pub struct ImagesumArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

#[derive(Debug, Parser)]
pub struct VideosumArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

fn main() {
    // Init app
    let app = App::parse();

    // Run appropriate sub-command
    match app.command {
        Command::Cp(args) => {
            // Call a function to handle the 'cp' command
            commands::cp::execute(args);
        }
        Command::Mv(args) => {
            // Call a function to handle the 'mv' command
            commands::mv::execute(args);
        }
        Command::Count(args) => {
            // Call a function to handle the 'count' command
            commands::count::execute(args);
        }
        Command::Imagesum(args) => {
            // Call a function to handle the 'imagesum' command
            commands::imagesum::execute(args);
        }
        Command::Videosum(args) => {
            // Call a function to handle the 'videosum' command
            commands::videosum::execute(args);
        }
    }
}
