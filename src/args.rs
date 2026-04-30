use crate::strings::*;
use clap::{ Parser, ValueEnum };

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Parser)]
#[command(version,
          about,
          after_long_help = { AFTER_HELP.trim_matches('\n') })]
pub struct ProgramFlags {
    #[arg(long = "color",
          value_enum,
          default_value = "auto",
          alias = "colors",
          id = "WHEN")]
    pub colorize: WhenColors,

    /// Enumerate paths
    #[arg(short, long)]
    pub enumerate: bool,

    /// Pad line numbers with zeros
    #[arg(short, long)]
    pub zero_padding: bool,

    /// Self-explanatory
    #[arg(short = 'S', long)]
    pub no_status: bool,

    #[arg(short,
          long,
          value_enum,
          default_value = "text",
          id = "STYLE")]
    pub status_style: Option<StatusStyle>,

    /// Self-explanatory
    #[arg(short = 'd', long)]
    pub descriptions: bool,

    /// Display a label above each column
    #[arg(short = 'H', long)]
    pub header: bool,

    /// Display the valid path count below
    #[arg(short = 'F', long)]
    pub footer: bool,

    /// Specify a format for each path
    #[arg(long,
          default_value = DEFAULT_FORMAT,
          hide_default_value = true)]
    pub format: String,

    /// Always exit with code 0
    #[arg(long)]
    pub succeed: bool,

    // pub hide_invalid_paths
    // pub no_duplicates: bool
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(ValueEnum, Clone, Copy)]
pub enum StatusStyle {
    None,
    Text,
    Icons,
    Emoji,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(ValueEnum, Clone, Copy)]
pub enum WhenColors {
    Never,
    Always,
    Auto,
}

pub fn get_repr(style: Option<StatusStyle>) -> Option<StatusSet> {
    style.and_then(|style| match style {
        StatusStyle::Icons => Some( ICON_SET),
        StatusStyle::Text  => Some( TEXT_SET),
        StatusStyle::Emoji => Some(EMOJI_SET),
        StatusStyle::None  => None
    })
}
