use std::path::Path;
use std::fs::File;
use std::io::Read;


fn main() {

    // create a String called Body that takes input from stdin
    let mut body: String = String::new();

    // fetch all of the command line arguments
    let args: Vec<String> = std::env::args().collect();

    let mut is_stdin: bool = false;

    // if the final argument is a file path and not a command line argument, read the file.
    // otherwise, read from stdin
    if args.len() > 1 && Path::new(&args[args.len() - 1]).exists() {
        let mut file = File::open(&args[args.len() - 1]).expect("File not found");
        file.read_to_string(&mut body).expect("Failed to read file");
    } else {
        is_stdin = true;
        std::io::stdin().read_to_string(&mut body).expect("Failed to read from stdin");
    }

    let print_all: bool = !args.contains(&String::from("-c")) &&
        !args.contains(&String::from("-w")) &&
        !args.contains(&String::from("-l")) &&
        !args.contains(&String::from("-m"));

    // if '-l' is present or print_all is true, print the number of lines in the string
    if print_all || args.contains(&String::from("-l")) {
        let lines: Vec<&str> = body.split('\n').collect();
        print!("\t{}", lines.len() - 1);
    }

    // if '-w' is present or print_all is true, print the number of words in the string
    if print_all || args.contains(&String::from("-w")) {
        let words: Vec<&str> = body.split_whitespace().collect();
        print!("\t{}", words.len());
    }

    // if '-c' is present or print_all is true, print the number of bytes in the string
    if print_all || args.contains(&String::from("-c")) {
        print!("\t{}", body.len());
    }

    // if '-m' is present, print the number of characters in the string
    if args.contains(&String::from("-m")) {
        print!("\t{}", body.chars().count());
    }

    // if a file was provided in the arguments, show the filename last
    if args.len() > 1 && !is_stdin {
        print!("\t{}", args[args.len() - 1]);
    }

    println!();

}
