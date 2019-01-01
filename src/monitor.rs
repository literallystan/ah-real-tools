use clap::ArgMatches;
use std::process::Command;

pub fn switch_display(flag: &ArgMatches) {
    if flag.is_present("desktop") {
        Command::new("xrandr")
            .arg("--output")
            .arg("DP-2")
            .arg("--auto")
            .output()
            .expect("failed to turn monitor on");
        Command::new("xrandr")
            .arg("--output")
            .arg("DP-2")
            .arg("--right-of")
            .arg("DP-4")
            .output()
            .expect("failed to extend display");
        Command::new("pacmd")
            .arg("set-default-sink")
            .arg("alsa_output.pci-0000_00_1b.0.analog-stereo")
            .output()
            .expect("failed to switch to headphones");

    } else if flag.is_present("console") {
        Command::new("xrandr")
            .arg("--output")
            .arg("DP-2")
            .arg("--off")
            .output()
            .expect("failed to turn monitor off");
    }
}

//TODO: remove hardcoded display values and make some sort of config that holds your particular monitor values