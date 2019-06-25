extern crate ironbed;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};
use ironbed::union::union_main;
use ironbed::random::{rand_bed_inf, rand_bed_with_lines};


fn main() {
    let matches = App::new("ironbed")
                          .version(crate_version!())
                          .about("An implementation of bedtools in Rust")
                          .author("William S. Owens <wowens@ufl.edu>")
                          .subcommand(SubCommand::with_name("unionbedg")
                                      .version(crate_version!())
                                      .about("Combines multiple bedGraph files into a single file.")
                                      .arg(Arg::with_name("input")
                                           .short("i")
                                           .multiple(true)
                                           .required(true)
                                           .takes_value(true)
                                           .value_name("FILE")
                                           .help("Input bedGraph files. Input files cannot contain overlapping intervals and should be sorted by chrom, start. (Use the command 'sort -k1,1 -k2,2n for the correct order.')"))
                                      .arg(Arg::with_name("filler")
                                           .long("filler")
                                           .takes_value(true)
                                           .value_name("TEXT")
                                           .help("Use <TEXT> when representing intervals having no value. [Default: '0']"))
                                      .arg(Arg::with_name("empty")
                                           .long("empty")
                                           .requires("genome")
                                           .help("Report empty regions (i.e. start/end intervals with no values in any file). Requires '-g <FILE>' parameter."))
                                      .arg(Arg::with_name("genome")
                                           .short("g")
                                           .long("genome")
                                           .takes_value(true)
                                           .value_name("FILE")
                                           .help("Use genome file <FILE> to calculate empty regions.")))
                          .subcommand(SubCommand::with_name("random")
                                      .version(crate_version!())
                                      .about("Generate random BED files")
                                      .arg(Arg::with_name("genome")
                                           .short("g")
                                           .long("genome")
                                           .takes_value(true)
                                           .required(true)
                                           .value_name("FILE")
                                           .help("Use genome file <FILE> for random values"))
                                      .arg(Arg::with_name("lines")
                                           .short("n")
                                           .long("lines")
                                           .takes_value(true)
                                           .value_name("NUM")
                                           .help("Output <NUM> lines [default: infinite]")))
                          .get_matches();

    match matches.subcommand() {
        ("unionbedg", Some(ubg_matches)) => {
            //this operation is safe because get_matches() will halt execution if '-i' is not provided
            let filenames: Vec<&str> = ubg_matches.values_of("input").unwrap().collect();
            // filler has a default value of "0"
            let filler = ubg_matches.value_of("filler").unwrap_or("0");
            union_main(filenames, filler, ubg_matches.is_present("empty"), ubg_matches.value_of("genome")).unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            });
        },
        ("random", Some(rand_matches)) => {
            //this operation is safe because --genome is required
            let fname = rand_matches.value_of("genome").unwrap();
            match rand_matches.value_of("lines") {
                //if no lines provided, generate an infinite bed
                None => rand_bed_inf(fname),
                //otherwise, grab the number of lines
                Some(n) => match n.parse() {
                    Err(_) => Err(format!("Expected unsigned integer for --lines, received '{}'", n)),
                    Ok(n) => rand_bed_with_lines(fname, n),
                }
            //unwrap if any errors occurred
            }.unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            })
        }
        ("", None) => eprintln!("No subcommand provided. Try 'ironbed help' for available subcommands."),
        _ => unreachable!(),
    }
}