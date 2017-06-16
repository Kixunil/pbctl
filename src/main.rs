extern crate progressbar_hackerspace_ctl as phc;
#[macro_use]
extern crate clap;

mod cli;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("lights") {
        use phc::lights::EnumerateLights;

        let mut lights = phc::lights::Lights::new().unwrap();

        if let Some(_) = matches.subcommand_matches("on") {
            lights.enumerate_lights(|mut light| light.turn_on())
        }
        if let Some(_) = matches.subcommand_matches("off") {
            lights.enumerate_lights(|mut light| light.turn_off())
        }
        if let Some(matches) = matches.subcommand_matches("main") {
            if let Some(_) = matches.subcommand_matches("on") {
                lights.main_room().enumerate_lights(|mut light| light.turn_on())
            }
            if let Some(_) = matches.subcommand_matches("off") {
                lights.main_room().enumerate_lights(|mut light| light.turn_off())
            }
            if let Some(matches) = matches.subcommand_matches("front") {
                if let Some(_) = matches.subcommand_matches("on") {
                    lights.main_room().front_light().turn_on()
                }
                if let Some(_) = matches.subcommand_matches("off") {
                    lights.main_room().front_light().turn_off()
                }
            }
            if let Some(matches) = matches.subcommand_matches("back") {
                if let Some(_) = matches.subcommand_matches("on") {
                    lights.main_room().back_light().turn_on()
                }
                if let Some(_) = matches.subcommand_matches("off") {
                    lights.main_room().back_light().turn_off()
                }
            }
        }
        if let Some(matches) = matches.subcommand_matches("lab") {
            if let Some(_) = matches.subcommand_matches("on") {
                lights.lab().enumerate_lights(|mut light| light.turn_on())
            }
            if let Some(_) = matches.subcommand_matches("off") {
                lights.lab().enumerate_lights(|mut light| light.turn_off())
            }
            if let Some(matches) = matches.subcommand_matches("left") {
                if let Some(_) = matches.subcommand_matches("on") {
                    lights.lab().left_light().turn_on()
                }
                if let Some(_) = matches.subcommand_matches("off") {
                    lights.lab().left_light().turn_off()
                }
            }
            if let Some(matches) = matches.subcommand_matches("right") {
                if let Some(_) = matches.subcommand_matches("on") {
                    lights.lab().right_light().turn_on()
                }
                if let Some(_) = matches.subcommand_matches("off") {
                    lights.lab().right_light().turn_off()
                }
            }
        }
    }
}
