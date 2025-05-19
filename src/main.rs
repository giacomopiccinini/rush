use clap::{Args, Parser, Subcommand};
use rush::{
    AudioResampleArgs, AudioSplitArgs, AudioSummaryArgs, AudioTrimArgs, FileCountArgs,
    FileExtensionArgs, ImageDuplicatesArgs, ImageResizeArgs, ImageSummaryArgs, ImageTessellateArgs,
    ImageToLandscapeArgs, ImageToPortraitArgs, TableSchemaArgs, TableSummaryArgs, TableToCsvArgs,
    TableToParquetArgs, VideoDuplicatesArgs, VideoFromFramesArgs, VideoSummaryArgs,
    VideoThumbnailArgs, VideoToFramesArgs,
};

/// Swiss-army knife for media inspection and manipulation
#[derive(Debug, Parser)]
#[clap(name = "rush", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Commands for audio files
    Audio(AudioCommand),
    /// Commands for image files
    Image(ImageCommand),
    /// Commands for video files
    Video(VideoCommand),
    /// Commands for generic files
    File(FileCommand),
    /// Commands for tabular files
    Table(TableCommand),
}

#[derive(Debug, Args)]
struct AudioCommand {
    #[clap(subcommand)]
    command: AudioSubCommand,
}

#[derive(Debug, Subcommand)]
enum AudioSubCommand {
    /// Summary of audio content of file or directory
    Summary(AudioSummaryArgs),
    /// Split audio file in chunks of fixed length
    Split(AudioSplitArgs),
    /// Resample audio file
    Resample(AudioResampleArgs),
    /// Trim audio file to given length (possibly with initial offset)
    Trim(AudioTrimArgs),
}

#[derive(Debug, Args)]
struct ImageCommand {
    #[clap(subcommand)]
    command: ImageSubCommand,
}

#[derive(Debug, Subcommand)]
enum ImageSubCommand {
    /// Summary of image content of file or directory
    Summary(ImageSummaryArgs),
    /// Resize image to a fixed height and width
    Resize(ImageResizeArgs),
    /// Divide image into tiles
    Tessellate(ImageTessellateArgs),
    /// Rotate to landscape
    ToLandscape(ImageToLandscapeArgs),
    /// Rotate to portrait
    ToPortrait(ImageToPortraitArgs),
    /// Find duplicated images
    Duplicates(ImageDuplicatesArgs),
}

#[derive(Debug, Args)]
struct VideoCommand {
    #[clap(subcommand)]
    command: VideoSubCommand,
}

#[derive(Debug, Subcommand)]
enum VideoSubCommand {
    /// Summary of video content of file or directory
    Summary(VideoSummaryArgs),
    /// Extract frames from video file
    ToFrames(VideoToFramesArgs),
    /// Collect frames into a video
    FromFrames(VideoFromFramesArgs),
    /// Find duplicated video files
    Duplicates(VideoDuplicatesArgs),
    /// Create video thumbnail
    Thumbnail(VideoThumbnailArgs),
}

#[derive(Debug, Args)]
struct FileCommand {
    #[clap(subcommand)]
    command: FileSubCommand,
}

#[derive(Debug, Subcommand)]
enum FileSubCommand {
    /// Count files and directories
    Count(FileCountArgs),
    /// Find unique extensions
    Extension(FileExtensionArgs),
}

#[derive(Debug, Args)]
struct TableCommand {
    #[clap(subcommand)]
    command: TableSubCommand,
}

#[derive(Debug, Subcommand)]
enum TableSubCommand {
    /// Get tabular file schema
    Schema(TableSchemaArgs),
    /// Convert CSV file to parquet
    ToParquet(TableToParquetArgs),
    /// Convert parquet file to CSV
    ToCsv(TableToCsvArgs),
    /// Summary of tabular file
    Summary(TableSummaryArgs),
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
            ImageSubCommand::Duplicates(args) => rush::commands::image::duplicates::execute(args),
        },
        Command::Video(video_command) => match video_command.command {
            VideoSubCommand::Summary(args) => rush::commands::video::summary::execute(args),
            VideoSubCommand::ToFrames(args) => rush::commands::video::to_frames::execute(args),
            VideoSubCommand::FromFrames(args) => rush::commands::video::from_frames::execute(args),
            VideoSubCommand::Duplicates(args) => rush::commands::video::duplicates::execute(args),
            VideoSubCommand::Thumbnail(args) => rush::commands::video::thumbnail::execute(args),
        },
        Command::File(file_command) => match file_command.command {
            FileSubCommand::Count(args) => rush::commands::file::count::execute(args),
            FileSubCommand::Extension(args) => rush::commands::file::extension::execute(args),
        },
        Command::Table(table_command) => match table_command.command {
            TableSubCommand::Schema(args) => rush::commands::table::schema::execute(args),
            TableSubCommand::ToParquet(args) => rush::commands::table::to_parquet::execute(args),
            TableSubCommand::ToCsv(args) => rush::commands::table::to_csv::execute(args),
            TableSubCommand::Summary(args) => rush::commands::table::summary::execute(args),
        },
    };

    if let Err(e) = result {
        rush::handle_error(e);
    }
}
