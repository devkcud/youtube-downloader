use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct YTDPArgs {
    #[clap(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value_t=String::from("videos"), global(true))]
    /// Output folder
    pub output: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download a video/playlist
    Download(DefaultParameters),
    /// Get information about a video/playlist
    Info(DefaultParameters),
}

#[derive(Debug, Args)]
pub struct DefaultParameters {
    /// Video/Playlist link
    pub link: String,

    #[arg(long, action(ArgAction::SetTrue))]
    /// Download as mp3 instead of mp4
    pub mp3: bool,
}
