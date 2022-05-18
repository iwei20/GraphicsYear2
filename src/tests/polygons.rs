use std::fs::File;

use crate::MDLParser;
#[test]
fn main() {
    let mut p: MDLParser = Default::default();
    p.parse_file(File::open("src/tests/torusfractal").expect("File open failed")).expect("Program parse failed");
}