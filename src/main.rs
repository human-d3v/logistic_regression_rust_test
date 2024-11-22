use std::path::PathBuf;

pub mod data_utils;
use data_utils::{BostonRecord, DataLoader};

pub mod error;
use error::Result;

fn main() -> Result<()>{
    let mut p:PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")
        .expect("Unable to find crate root")
    );
    p.push("src/data/boston.csv");

    let data = BostonRecord::from_csv(&p)
        .expect("Error loading data from csv file");

    println!("{:?}", data); 
    Ok(())
}
