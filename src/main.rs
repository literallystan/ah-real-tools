extern crate clap;

use clap::{Arg, App, SubCommand};
use std::{thread, time};
use indicatif::{ProgressBar, ProgressStyle};

static PBAR_FMT: &'static str = "{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}";

fn main() {
    let matches = App::new("ahtools")
        .version("0.1.0") //abstract to cargo.toml
        .author("Adam Hawkins <literallystan@gmail.com>")
        .about("dumb progress bar")
        .arg(Arg::with_name("task")
            .short("t")
            .long("task")
            .help("this does nothing"))
        .subcommand(SubCommand::with_name("progress")
            .arg(Arg::with_name("prog")
                .short("p")
                .help("do progress")))
        .get_matches();
    
    match matches.subcommand() {
        ("progress", Some(_)) => {indicate_progress()},
        _ => {println!("No command given")}
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