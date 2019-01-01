use clap::{Arg, App, SubCommand};
use std::{thread, time};
use indicatif::{ProgressBar, ProgressStyle};
use ah_real_tools::monitor;
use ah_real_tools::sound;

static PBAR_FMT: &'static str = "{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}";

fn main() {
    let matches = App::new("ahtools")
        .version("0.1.0") //abstract to cargo.toml
        .author("Adam Hawkins <literallystan@gmail.com>")
        .about("common commands simplified")
        .subcommand(SubCommand::with_name("progress")
            .about("display a dumb progress bar")
            .arg(Arg::with_name("prog")
                .short("p")
                .help("do progress")))
        .subcommand(SubCommand::with_name("display")
            .about("switch display modes")
            .arg(Arg::with_name("desktop")
                .short("d")
                .long("desktop")
                .conflicts_with("console")
                .help("switch to PC mode"))
            .arg(Arg::with_name("console")
                .short("c")
                .long("console")
                .conflicts_with("desktop")
                .help("switch to console mode")))
        .subcommand(SubCommand::with_name("sound")
            .about("switch sound modes")
            .arg(Arg::with_name("headphones")
                .short("h")
                .long("headphones")
                .help("switch to headphones"))
            .arg(Arg::with_name("speakers")
                .short("s")
                .long("speakers")
                .help("switch to speakers")))
        .get_matches();

    
    match matches.subcommand() {
        ("progress", Some(_)) => {indicate_progress()},
        ("display", Some(sub_com)) => {
            monitor::switch_display(&sub_com)},
        ("sound", Some(sub_com)) => {
            sound::switch_audio(&sub_com)},
        _ => {println!("Invalid command given")}
    };

}

fn indicate_progress() {
    let bar = ProgressBar::new(1000);
    bar.set_style(ProgressStyle::default_bar().template(PBAR_FMT).progress_chars("=> "));
    for i in 0..1000 {
        bar.inc(1);
        bar.set_message(&format!("doing {}", i.to_string()));
        thread::sleep(time::Duration::from_millis(10));
    }
    bar.finish();
}
