use bevy::prelude::*;
use clap::Parser;

#[derive(Debug, Default, Parser)]
enum Cli {
    #[default]
    Launch,
    Update,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse() {
        Cli::Launch => App::new().add_plugins(DefaultPlugins).run(),
        Cli::Update => {
            let status = self_update::backends::github::Update::configure()
                .repo_owner("washed-up-devs")
                .repo_name(env!("CARGO_PKG_NAME"))
                .bin_name("github")
                .show_download_progress(true)
                .current_version(self_update::cargo_crate_version!())
                .build()?
                .update()?;

            eprintln!("Update status: `{}`!", status.version());
        }
    }

    Ok(())
}
