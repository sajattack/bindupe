use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Error, copy, SeekFrom};
use clap::{Arg, App};

fn main() -> Result <(), Error>{
    let matches = App::new("bindupe")
        .version("0.1")
        .author("Paul Sajna <sajattack@gmail.com>")
        .about("Duplicates bytes of infile ndupes times in outfile")
        .arg(Arg::with_name("infile")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("outfile")
            .help("Sets the output file to use")
            .required(true)
            .index(2))
        .arg(Arg::with_name("ndupes")
            .help("Sets the number of times to duplicate bytes of infile to outfile")
            .required(true)
            .index(3))
        .get_matches();

    let infile = File::open(matches.value_of("infile").unwrap())?;
    let outfile = File::create(matches.value_of("outfile").unwrap())?;
    let mut reader = BufReader::new(infile);
    let mut writer = BufWriter::new(outfile);
    let ndupes = matches.value_of("ndupes").unwrap().parse::<usize>().unwrap();
    for _ in 0..ndupes { 
        copy(&mut reader, &mut writer)?;
        reader.seek(SeekFrom::Start(0))?;
    }
    Ok(())
}
