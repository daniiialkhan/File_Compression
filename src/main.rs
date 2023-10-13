use std::fs::File;
use std::io::Write;
mod random;
mod filling;
use std::env::args;

use std::path::PathBuf;

fn generate(path: &str) {
	// large random text file
	let ab_list = random::random::gen_random_ab();
	println!("{:#?}", ab_list); 

	// write this to file
	filling::filling::write_to_file(path, &ab_list).expect("Unable to write to file\n\n\tMaybe the path was incorrect?\n\n");
}

fn compress(path: &str) { // path is for file to be compressed

	// Read the file and compress it
    let mut compressed = String::new();
    filling::filling::read_file(path, &mut compressed).expect("Unable to read file\n\n\tMaybe the path was incorrect?\n\n");
    println!("file compressed: {}", compressed);

	// get the directory of the uncompressed file
	let mut dir = PathBuf::from(path);
	dir.pop();
	let dir = dir.to_str().unwrap();
	println!("directory: {}", dir);

    // write compressed string to new file
    let mut file = File::create(format!("{}/compressed.txt", dir)).expect("Unable to create file");
    file.write_all(compressed.as_bytes()).expect("Unable to write to file");

}

fn compare(path: &str) {
	
	let orignal = format!("{}/ab_list.txt",path);
	let compressed = format!("{}/compressed.txt",path);

	let before = filling::filling::get_file_size(&orignal).expect("Unable to get file size");
    let after = filling::filling::get_file_size(&compressed).expect("Unable to get file size");

    println!("File size before compression: {} bytes", before);
    println!("File size after compression: {} bytes", after);
    println!("Compression ratio: {}%", (1.0 - (after as f64 / before as f64)) * 100.0);
}

fn main() {


	let pattern = match args().nth(1) {
		Some(pattern) => pattern,
		None => panic!("No Pattern Given. \n\n\tUsage:  cargo run <pattern> <path>\n\n Available patterns:\n\n\t1. generate => generates new file with random text at specified path\n\n\t2. compress => compresses generated file into same directory in specified path\n\n\t3. compare => compares change in size of orignal and compressed files in specified directory\n\n"), // if no pattern given, use empty string
	}; // generate or compress
	
	let path = match args().nth(2) { // path to file if compressing
		Some(path) => path,
		None => panic!("No Path Given.  \n\n\tUsage:  cargo run <pattern> <path>\n\n Available patterns:\n\n\t1. generate => generates new file with random text at specified path\n\n\t2. compress => compresses generated file into same directory in specified path\n\n\t3. compare => compares change in size of orignal and compressed files in specified directory\n\n"), // if no path given, use empty string
	};

	println!("Got Pattern: {}", pattern);
	println!("Got Path: {:?}", path);

	if pattern == "generate" {
		generate(&path);
	} else if pattern == "compress" {
		compress(&path);
	} else if pattern == "compare" {
		compare(&path);
	} else {
		println!("Invalid pattern, please use \n\n\t'generate<space>path/to/file'\n\tor\n\t'compress<space>path/to/file'\n\n");
	}
    
}
