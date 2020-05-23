use structopt::StructOpt;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io::LineWriter;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use indicatif::ProgressBar;

#[derive(StructOpt)]
#[structopt(
	name = "linematcher", 
	about = "Matches lines in files. The first file is the reference file - this tools finds duplicate lines in the additional files and outputs them as csv.")]
struct Cli {
    
    #[structopt(
        parse(from_os_str),
        required(true),
        takes_value(true),
        multiple(true),
        help("A list of files to compare lines")
    )]
    paths: Vec<std::path::PathBuf>,

}


fn main() {

    let mut line_entries = HashMap::new();
    let cli = Cli::from_args();
    let progress_bar = ProgressBar::new((cli.paths.len() + 1) as u64);

    println!("Examining following files for duplicate lines: {:?}", cli.paths);

    for (index, path) in cli.paths.iter().enumerate() {
    	let file = File::open(path).expect("file not found...");
    	let reader = BufReader::new(file);
    	for (_, line) in reader.lines().enumerate(){
    		line_entries.entry(line.unwrap())
			.and_modify(|e| { if index > 0 { *e += 1 }})
			.or_insert(0);
	}
	progress_bar.println(format!("[+] finished reading file {:#?}", path));
        progress_bar.inc(1);
    }

    // clean up single elements
    line_entries.retain(|_, &mut v| v > 0);

    // compile output    
    let file = OpenOptions::new()
	.write(true)
	.create(true)
	.truncate(true)
        .open("duplicates.csv");    

    let mut writer = LineWriter::new(file.unwrap());

    for entry in &line_entries {
	writer.write_all(&String::from(entry.0).into_bytes()).expect("Could not write line.");
	writer.write_all(b"\n").expect("Could not write new line.");
    }
    writer.flush().expect("Could not flush");
    progress_bar.println(format!("[+] finished writing results to duplicates.csv."));
    progress_bar.finish();

}
