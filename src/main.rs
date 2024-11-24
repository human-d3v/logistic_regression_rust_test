use std::path::PathBuf;

pub mod data_utils;
use data_utils::{BostonRecord, DataLoader, FieldAccessor};

pub mod error;
use error::Result;

fn main() -> Result<()>{
    let mut p:PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")
        .expect("Unable to find crate root")
    );
    p.push("src/data/boston.csv");

    let data = BostonRecord::from_csv(&p)
        .expect("Error loading data from csv file");

    let zn = match data.get_field("zn") {
        Some(v) => v,
        None => return Err("Failed to get proportion of 25K zoned land".into())
    };

    let dis = match data.get_field("dis") {
        Some(v) => v,
        None => return Err("Failed to get distance to employment centers".into())
    };

    let nox = match data.get_field("nox") {
        Some(v) => v, 
        None => return Err("Failed to get nitric oxide concentration".into())
    };

    
    Ok(())
}
