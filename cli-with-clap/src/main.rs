use regex::Regex;
use clap::{Arg, Command, ValueHint};

fn main() {
    let args = Command::new("grep-lite using regex and clap")
        .version("0.1")
        .about("searches for patterns in strings")
        .arg(Arg::new("pattern")
            .help("The pattern to search for")
            .required(true))
        .arg(Arg::new("Search_str")
            .help("String to search from")
            .required(true))
        .get_matches();
    let pattern = args.get_one::<String>("pattern").expect("pattern input");
    let re = Regex::new(pattern).expect("regex pattern fail");

    let quote = args.get_one::<String>("Search_str").expect("search string input");
    for line in quote.lines(){
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
