extern crate ironbed;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

use ironbed::bedgraph::BgIterator;


fn main() {
    let matches = App::new("ironbed")
                          .version(crate_version!())
                          .about("An implementation of bedtools in Rust")
                          .author("William S. Owens <wowens@ufl.edu>")
                          .subcommand(SubCommand::with_name("unionbedg")
                                      .about("combines multiple BedGraph files into a single file")
                                      .arg(Arg::with_name("input")
                                           .short("i")
                                           .multiple(true)
                                           .required(true)
                                           .help("Input BedGraph files. Assumes that file is sorted and that intervals are non-overlapping.")))
                          .get_matches();

    match matches.subcommand() {
        ("unionbedg", Some(ubg_matches)) => {
            //this operation is safe, because "-i" is required
            //if there wasn't a value, clap would have caught it

        }, 
        ("", None) => eprintln!("No subcommand provided. Try 'ironbed help' for available subcommands."),
        _ => unreachable!(),
    }
}