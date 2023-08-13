use std::env;
use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fountain.pest"]
pub struct OurParser;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let unparsed_file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let file = OurParser::parse(Rule::file, &unparsed_file);
        //.expect("unsuccessful parse"); // unwrap the parse result
        //.next().unwrap(); // get and unwrap the `file` rule; never fails
    println!("{:?}", file);
}
