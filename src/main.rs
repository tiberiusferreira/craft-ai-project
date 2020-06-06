use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

mod lineage;
mod visualization;
use lineage::{Lineage, ParentChildInfo};
fn main() {
    let file = std::fs::File::open("got_families.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut lineage = Lineage::new();

    for deserialization_result in rdr.deserialize() {
        let person_csv: ParentChildInfo = deserialization_result.unwrap();
        lineage.insert(person_csv);
    }

    println!("{:?}", lineage.next_in_line("Minisa Whent"));
    lineage.kill("Edmure Tully");
    println!("{:?}", lineage.next_in_line("Minisa Whent"));
    lineage.kill("Catelyn Tully");
    println!("{:?}", lineage.next_in_line("Minisa Whent"));
}
