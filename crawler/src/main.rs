extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("crawler").
        version("0.0.1").
        author("Chris Sinjakli <chris@sinjakli.co.uk>").
        about("A single-threaded, single-domain web crawler").
        arg(Arg::with_name("start-url").
            short("s").
            long("start-url").
            value_name("URL").
            help("The URL of the page to start crawling from").
            takes_value(true).
            required(true)
            ).
        get_matches();

    let start_url = matches.value_of("start-url").unwrap();

    println!("Let's crawl {}!", start_url);
}
