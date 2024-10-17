use clap::{Parser, Subcommand, Args};

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
    /// Audio-related commands
    Audio(AudioCommand),
    /// Image-related commands
    Image(ImageCommand),
    /// Video-related commands
    Video(VideoCommand),
    /// File operations
    File(FileCommand),
    /// Table operations
    Table(TableCommand),
}

#[derive(Debug, Args)]
struct AudioCommand {
    #[clap(subcommand)]
    command: AudioSubCommand,
}

#[derive(Debug, Subcommand)]
enum AudioSubCommand {
    /// Get audio metadata
    Summary(AudioSummaryArgs),
    /// Split audio file into chunks
    Split(AudioSplitArgs),
    /// Resample audio file
    Resample(AudioResampleArgs),
}

#[derive(Debug, Args)]
struct ImageCommand {
    #[clap(subcommand)]
    command: ImageSubCommand,
}

#[derive(Debug, Subcommand)]
enum ImageSubCommand {
    /// Get image metadata
    Summary(ImageSummaryArgs),
    /// Resize image
    Resize(ImageResizeArgs),
    // Tassellat the image
    Tessellate(ImageTessellateArgs),
}

#[derive(Debug, Args)]
struct VideoCommand {
    #[clap(subcommand)]
    command: VideoSubCommand,
}

#[derive(Debug, Subcommand)]
enum VideoSubCommand {
    /// Get video metadata
    Summary(VideoSummaryArgs),
    // Add other video-related commands here
}

#[derive(Debug, Args)]
struct FileCommand {
    #[clap(subcommand)]
    command: FileSubCommand,
}

#[derive(Debug, Subcommand)]
enum FileSubCommand {
    /// Copy files from a source to a target
    Cp(CpArgs),
    /// Move files from a source to a target
    Mv(MvArgs),
    /// Count files and directories in a target directory
    Count(CountArgs),
}

#[derive(Debug, Args)]
struct TableCommand {
    #[clap(subcommand)]
    command: TableSubCommand,
}

#[derive(Debug, Subcommand)]
enum TableSubCommand {
    /// Get table schema
    Schema(TableSchemaArgs),
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
pub struct ImageSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

#[derive(Debug, Parser)]
pub struct ImageResizeArgs {
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

#[derive(Debug, Parser)]
pub struct ImageTessellateArgs {
    /// Target file
    #[arg(required = true)]
    target: String,

    #[arg(required = true)]
    n_vertical: u32,

    #[arg(required = true)]
    n_horizontal: u32,

    /// Output file
    #[arg(required = true)]
    output: String,
}

#[derive(Debug, Parser)]
pub struct AudioSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,

    /// Flag for printing info on single file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug, Parser)]
pub struct AudioSplitArgs {
    /// Input file or directory
    #[arg(required = true)]
    input: String,

    /// Chunk duration in seconds
    #[arg(required = true)]
    chunk_duration: f32,

    /// Output directory
    #[arg(required = true)]
    output: String,

    /// Delete original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    delete_original: bool,
}

#[derive(Debug, Parser)]
pub struct AudioResampleArgs {
    /// Input file or directory
    #[arg(required = true)]
    input: String,

    /// Target sample rate
    #[arg(required = true)]
    sr: u32,

    /// Output directory
    #[arg(required = true)]
    output: String,

    /// Replace original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    replace_original: bool,
}


#[derive(Debug, Parser)]
pub struct VideoSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    target: String,
}


#[derive(Debug, Args)]
pub struct TableSchemaArgs {
    /// Target file
    #[arg(required = true)]
    target: String,
}


fn main() {
    // Init app
    let app = App::parse();

    // Run appropriate sub-command
    match app.command {
        Command::Audio(audio_command) => match audio_command.command {
            AudioSubCommand::Summary(args) => commands::audio::summary::execute(args),
            AudioSubCommand::Split(args) => commands::audio::split::execute(args),
            AudioSubCommand::Resample(args) => commands::audio::resample::execute(args),
        },
        Command::Image(image_command) => match image_command.command {
            ImageSubCommand::Summary(args) => commands::image::summary::execute(args),
            ImageSubCommand::Resize(args) => commands::image::resize::execute(args),
            ImageSubCommand::Tessellate(args) => commands::image::tessellate::execute(args),
        },
        Command::Video(video_command) => match video_command.command {
            VideoSubCommand::Summary(args) => commands::video::summary::execute(args),
        },
        Command::File(file_command) => match file_command.command {
            FileSubCommand::Cp(args) => commands::file::cp::execute(args),
            FileSubCommand::Mv(args) => commands::file::mv::execute(args),
            FileSubCommand::Count(args) => commands::file::count::execute(args),
        },
        Command::Table(table_command) => match table_command.command {
            TableSubCommand::Schema(args) => commands::table::schema::execute(args),
        },
    }
}