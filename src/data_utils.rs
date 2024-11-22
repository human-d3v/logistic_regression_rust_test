// Variables in order:
// CRIM     per capita crime rate by town
// ZN       proportion of residential land zoned for lots over 25,000 sq.ft.
// INDUS    proportion of non-retail business acres per town
// CHAS     Charles River dummy variable (= 1 if tract bounds river; 0 otherwise)
// NOX      nitric oxides concentration (parts per 10 million)
// RM       average number of rooms per dwelling
// AGE      proportion of owner-occupied units built prior to 1940
// DIS      weighted distances to five Boston employment centres
// RAD      index of accessibility to radial highways
// TAX      full-value property-tax rate per $10,000
// PTRATIO  pupil-teacher ratio by town
// B        1000(Bk - 0.63)^2 where Bk is the proportion of blacks by town
// LSTAT    % lower status of the population
// MEDV     Median value of owner-occupied homes in $1000's

use std::path::Path;
use csv::StringRecord;
use crate::error::Result;

#[derive(Debug)]
pub struct BostonRecord {
    pub crim: f64,
    pub zn: f64,
    pub indus: f64,
    pub chas: f64,
    pub nox: f64,
    pub rm: f64,
    pub age: f64,
    pub dis: f64,
    pub rad: f64,
    pub tax: f64,
    pub ptratio: f64,
    pub b: f64,
    pub lstat: f64,
    pub medv: f64,
}


pub trait DataLoader<T> {
    fn from_string_record(s: &StringRecord) -> Result<T>;
    fn from_csv(p: &Path) -> Result<Vec<T>>;
}

impl DataLoader<BostonRecord> for BostonRecord {
    fn from_string_record(s: &StringRecord) -> Result<BostonRecord> {
        let records:Vec<f64> = s.iter()
            .map(|r| r.parse::<f64>()
                .expect("Error parsing float"))
            .collect::<Vec<f64>>();
        Ok(BostonRecord{
            crim: records[0],
            zn: records[1],
            indus: records[2],
            chas: records[3],
            nox: records[4],
            rm: records[5],
            age: records[6],
            dis: records[7],
            rad: records[8],
            tax: records[9],
            ptratio: records[10],
            b: records[11],
            lstat: records[12],
            medv: records[13],
        })
    }

    fn from_csv(p: &Path) -> Result<Vec<BostonRecord>> {
        Ok(csv::Reader::from_path(p)
            .expect("Error reading file at path")
            .records()
            .map(|r| 
                BostonRecord::from_string_record(
                    &r.expect("Error parsing record"))
                .expect("Error converting record to record"))
            .collect::<Vec<BostonRecord>>()
        )
    }
}


