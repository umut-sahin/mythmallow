use crate::prelude::*;


/// Controls the locale.
#[derive(ConsoleCommand, Parser)]
#[command(name = "locale")]
#[command(disable_help_flag = true)]
pub struct LocaleCommand {
    #[clap(subcommand)]
    pub subcommand: LocaleCommands,
}

/// Locale commands.
#[derive(Debug, Subcommand)]
pub enum LocaleCommands {
    /// Lists available locales.
    List,
    /// Shows the current locale.
    Show,
    /// Sets the current locale.
    Set { locale: String },
}
