use std::path::PathBuf;

pub mod data_utils;
use data_utils::{BostonRecord, DataLoader, DatasetUtils, FieldAccessor};

pub mod error;
use error::Result;
use rusty_machine::{learning::logistic_reg::LogisticRegressor, linalg::{Matrix, Vector}, prelude::SupModel};

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

    let indus = match data.get_field("indus") {
        Some(v) => v,
        None => return Err("Failed to get prportion of non-retail business".into())
    };

    let predictors = Matrix::new(
        data.len(),
        4,
        zn.into_iter()
            .zip(dis)
            .zip(nox)
            .zip(indus)
            .flat_map(|(((a,b),c),d) | vec![a,b,c,d])
            .collect::<Vec<f64>>()
    );

    let target = data.dich("crim", None)
        .expect("Failed to create target vector")
        .iter()
        .map(|v| *v as f64)
        .collect::<Vec<f64>>();
    
    let t = Vector::new(target);

    let mut logit_model = LogisticRegressor::default();
    logit_model.train(&predictors, &t)?;

    let coef = match logit_model.parameters() {
        Some(v) => v.into_iter().map(|x| x.exp()).collect::<Vec<f64>>(),
        None => return Err("Failed to fetch model coefficients".into()) 
    };

    println!("Exponentiated Coefs: {:?}", coef);
    Ok(())

}
