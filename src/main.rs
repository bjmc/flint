use std::env;
use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hackery.pest"]
pub struct OurParser;

#[cfg(test)]
mod tests {
  use super::*;
  //use mycrate::parser::{OurParser, Rule};
  use lazy_static::lazy_static;
  use pest_test::{Error, PestTester};

  lazy_static! {
    static ref TESTER: PestTester<Rule, OurParser> =
      PestTester::from_defaults(Rule::root_rule);
  }

  // Loads test from `tests/pest/mytest.txt` and evaluates it. Returns an `Err<pest_test::Error>`
  // if there was an error evaluating the test, or if the expected and actual values do not match.
  fn test_my_parser() -> Result<(), Error> {
    (*TESTER).evaluate_strict("mytest")
  }
}

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
