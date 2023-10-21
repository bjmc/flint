use std::env;
use std::fs;
use pest::Parser;
//use pest_derive::Parser;
//


mod example {
    #[derive(pest_derive::Parser)]
    #[grammar = "fountain.pest"]
    pub struct OurParser;
}

//#[derive(Parser)]
//#[grammar = "hackery.pest"]
//pub struct OurParser;
// Generate tests for all test cases in tests/pest/foo/ and all subdirectories. Since
// `lazy_static = true`, a single `PestTester` is created and used by all tests; otherwise a new
// `PestTester` would be created for each test. 
use pest_test_gen::pest_tests;
#[pest_tests(
  super::example::OurParser,
  super::example::Rule,
  "file",
  ext = "test",
  //subdir = "foo",
  recursive = true,
  lazy_static = true,
)]
#[cfg(test)]
mod tests {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let unparsed_file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let file = example::OurParser::parse(example::Rule::file, &unparsed_file);
        //.expect("unsuccessful parse"); // unwrap the parse result
        //.next().unwrap(); // get and unwrap the `file` rule; never fails
    println!("{:?}", file);
}
