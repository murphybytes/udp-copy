extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("udp-copy")
                    .arg(Arg::with_name("send")
                        )
                    .arg(Arg::with_name("server")
                            .takes_value(false)
                        )
                    .version("0.0.1")
                    .about("fast copy big files")
                    .get_matches();

    if matches.is_present("server") {
        println!("server set");
    }               

}
