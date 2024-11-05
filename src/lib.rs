use clap::{Args, Parser};

pub mod commands;
pub mod utils;

// Export all the Args structs as they're needed by both the CLI and tests
#[derive(Debug, Parser)]
pub struct CountArgs {
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct ImageSummaryArgs {
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct ImageResizeArgs {
    pub input: String,
    pub height: u32,
    pub width: u32,
    pub output: String,
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct ImageTessellateArgs {
    pub input: String,
    pub n_vertical: u32,
    pub n_horizontal: u32,
    pub output: String,
    pub delete_original: bool,
}

#[derive(Debug, Parser)]
pub struct AudioSummaryArgs {
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct AudioSplitArgs {
    pub input: String,
    pub chunk_duration: f32,
    pub output: String,
    pub delete_original: bool,
}

#[derive(Debug, Parser)]
pub struct AudioResampleArgs {
    pub input: String,
    pub sr: u32,
    pub output: String,
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct AudioTrimArgs {
    pub input: String,
    pub length: f32,
    pub output: String,
    pub offset: f32,
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct VideoSummaryArgs {
    pub target: String,
}

#[derive(Debug, Args)]
pub struct TableSchemaArgs {
    pub input: String,
}

// Error handling utility that can be used by both lib and binary
pub fn handle_error(e: anyhow::Error) {
    eprintln!("Error!");
    for (i, cause) in e.chain().enumerate() {
        eprintln!("  Cause {}: {}", i, cause);
    }
    std::process::exit(1);
}
