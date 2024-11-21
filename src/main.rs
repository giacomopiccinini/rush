use clap::{Args, Parser, Subcommand};
use rush::{
    AudioResampleArgs, AudioSplitArgs, AudioSummaryArgs, AudioTrimArgs, CountArgs, ImageResizeArgs,
    ImageSummaryArgs, ImageTessellateArgs, ImageToLandscapeArgs, ImageToPortraitArgs,
    TableSchemaArgs, TableToCsvArgs, TableToParquetArgs, VideoSummaryArgs,
};

/// Rust implementation of bash commands
#[derive(Debug, Parser)]
#[clap(name = "rush", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Audio(AudioCommand),
    Image(ImageCommand),
    Video(VideoCommand),
    File(FileCommand),
    Table(TableCommand),
}

#[derive(Debug, Args)]
struct AudioCommand {
    #[clap(subcommand)]
    command: AudioSubCommand,
}

#[derive(Debug, Subcommand)]
enum AudioSubCommand {
    Summary(AudioSummaryArgs),
    Split(AudioSplitArgs),
    Resample(AudioResampleArgs),
    Trim(AudioTrimArgs),
}

#[derive(Debug, Args)]
struct ImageCommand {
    #[clap(subcommand)]
    command: ImageSubCommand,
}

#[derive(Debug, Subcommand)]
enum ImageSubCommand {
    Summary(ImageSummaryArgs),
    Resize(ImageResizeArgs),
    Tessellate(ImageTessellateArgs),
    ToLandscape(ImageToLandscapeArgs),
    ToPortrait(ImageToPortraitArgs),
}

#[derive(Debug, Args)]
struct VideoCommand {
    #[clap(subcommand)]
    command: VideoSubCommand,
}

#[derive(Debug, Subcommand)]
enum VideoSubCommand {
    Summary(VideoSummaryArgs),
}

#[derive(Debug, Args)]
struct FileCommand {
    #[clap(subcommand)]
    command: FileSubCommand,
}

#[derive(Debug, Subcommand)]
enum FileSubCommand {
    Count(CountArgs),
}

#[derive(Debug, Args)]
struct TableCommand {
    #[clap(subcommand)]
    command: TableSubCommand,
}

#[derive(Debug, Subcommand)]
enum TableSubCommand {
    Schema(TableSchemaArgs),
    ToParquet(TableToParquetArgs),
    ToCsv(TableToCsvArgs),
}

fn main() {
    let app = App::parse();

    let result = match app.command {
        Command::Audio(audio_command) => match audio_command.command {
            AudioSubCommand::Summary(args) => rush::commands::audio::summary::execute(args),
            AudioSubCommand::Split(args) => rush::commands::audio::split::execute(args),
            AudioSubCommand::Resample(args) => rush::commands::audio::resample::execute(args),
            AudioSubCommand::Trim(args) => rush::commands::audio::trim::execute(args),
        },
        Command::Image(image_command) => match image_command.command {
            ImageSubCommand::Summary(args) => rush::commands::image::summary::execute(args),
            ImageSubCommand::Resize(args) => rush::commands::image::resize::execute(args),
            ImageSubCommand::Tessellate(args) => rush::commands::image::tessellate::execute(args),
            ImageSubCommand::ToLandscape(args) => {
                rush::commands::image::to_landscape::execute(args)
            }
            ImageSubCommand::ToPortrait(args) => rush::commands::image::to_portrait::execute(args),
        },
        Command::Video(video_command) => match video_command.command {
            VideoSubCommand::Summary(args) => rush::commands::video::summary::execute(args),
        },
        Command::File(file_command) => match file_command.command {
            FileSubCommand::Count(args) => rush::commands::file::count::execute(args),
        },
        Command::Table(table_command) => match table_command.command {
            TableSubCommand::Schema(args) => rush::commands::table::schema::execute(args),
            TableSubCommand::ToParquet(args) => rush::commands::table::to_parquet::execute(args),
            TableSubCommand::ToCsv(args) => rush::commands::table::to_csv::execute(args),
        },
    };

    if let Err(e) = result {
        rush::handle_error(e);
    }
}
