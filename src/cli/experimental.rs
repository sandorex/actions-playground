use clap::{Args, Subcommand};
use crate::features::FeaturePath;

#[derive(Args, Debug, Clone)]
pub struct CmdFeatureArgs {
    /// Features to install
    #[arg(required = true, value_name = "DIR|GIT|OCI", value_parser = FeaturePath::parse_cli)]
    pub feature: Vec<FeaturePath>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommandsDevContainer {
    /// Build a dev container image
    Build,

    /// Create and run dev container
    Up,

    /// Run user commands
    #[command(subcommand, name = "run-user-commands")]
    RunUserCommands,

    /// Read configuration to stdout
    #[command(subcommand, name = "read-configuration")]
    ReadConfiguration,

    /// Features commands
    Features,

    /// Templates commands
    Templates,

    /// Stops containers
    Stop,

    /// Stops and deletes containers
    Down,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommandsExperimental {
    /// Fetches a feature and executes `install.sh`
    Feature(CmdFeatureArgs),

    /// DevContainer cli implementation
    ///
    /// Security is nonexistant here due to many reasons so beware
    #[command(subcommand, name = "devcontainer")]
    DevContainer(CliCommandsDevContainer),
}
