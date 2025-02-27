mod rerolls;
mod serde_utils;
mod target_config;
mod weapon_config;

use crate::target_config::TargetConfig;
use crate::weapon_config::WeaponConfig;
use anyhow::{Result, bail};
use aos_statshammer_core::target::Target;
use aos_statshammer_core::weapon::Weapon;
use clap::Parser;
use log::debug;
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    weapon: PathBuf,

    #[arg(short, long)]
    save: Option<u8>,

    #[arg(short, long, value_name = "FILE")]
    target: Option<PathBuf>,

    #[arg(short, long)]
    verbose: bool,
}
impl Cli {
    pub fn execute(&self) -> Result<()> {
        SimpleLogger::new().init()?;
        if self.verbose {
            log::set_max_level(LevelFilter::Debug);
        } else {
            log::set_max_level(LevelFilter::Info);
        }

        let contents = fs::read_to_string(&self.weapon)?;
        let weapon_config: WeaponConfig = toml::from_str(&contents)?;
        let weapon: Weapon = weapon_config.try_into()?;

        let mut target: Option<Target> = None;
        if let Some(target_file) = &self.target {
            let target_contents = fs::read_to_string(target_file)?;
            let target_config: TargetConfig = toml::from_str(&target_contents)?;
            target = Some(target_config.try_into()?);
        }
        if let Some(save) = self.save {
            match target {
                Some(ref mut target) => target.save = save.into(),
                None => target = Some(save.into()),
            }
        }

        match target {
            Some(target) => {
                debug!("{:?}", &weapon);
                debug!("{:?}", &target);
                info!("Average Damage: {:.3}", weapon.average_damage(&target))
            }
            None => bail!("No target specified. Please use either --save or --target."),
        }

        Ok(())
    }
}

fn main() {
    let args = Cli::parse();
    args.execute().unwrap();
}
