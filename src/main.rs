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
    /// Get audio metadata
    Audiosum(AudiosumArgs),
    /// Get resize metadata
    Resize(ResizeArgs),
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

#[derive(Debug, Parser)]
pub struct AudiosumArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,

    /// Flag for printing info on single file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug, Parser)]
pub struct ResizeArgs {
    /// Target file
    #[arg(required = true)]
    target: String,

    #[arg(required = true)]
    height: u32,

    #[arg(required = true)]
    width: u32,

    /// Output file
    #[arg(required = true)]
    output: String,
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
        Command::Audiosum(args) => {
            // Call a function to handle the 'videosum' command
            commands::audiosum::execute(args);
        }
        Command::Resize(args) => {
            // Call a function to handle the 'resize' command
            commands::resize::execute(args);
        }
    }
}
