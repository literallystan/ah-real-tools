use clap::ArgMatches;
use std::process::Command;

pub fn switch_audio(flag: &ArgMatches) {
    if flag.is_present("headphones") {
        Command::new("pacmd")
            .arg("set-default-sink")
            .arg("alsa_output.pci-0000_00_1b.0.analog-stereo")
            .output()
            .expect("failed to switch to headphones");

    } else if flag.is_present("speaker") {
        Command::new("pacmd")
            .arg("set-default-sink")
            .arg("alsa_output.pci-0000_01_00.1.hdmi-stereo")
            .output()
            .expect("failed to switch to speakers");
    }
}