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

pub trait FieldAccessor<T> {
    fn get_field(&self, field: &str) -> Option<T>;
}

impl FieldAccessor<f64> for BostonRecord {
    fn get_field(&self, field: &str) -> Option<f64> {
        let field_value = match field {
            "crim" => self.crim,
            "zn" => self.zn,
            "indus" => self.indus,
            "chas" => self.chas,
            "nox" => self.nox,
            "rm" => self.rm,
            "age" => self.age,
            "dis" => self.dis,
            "rad" => self.rad,
            "tax" => self.tax,
            "ptratio" => self.ptratio,
            "b" => self.b,
            "lstat" => self.lstat,
            "medv" => self.medv,
            _ => return None
        };
        Some(field_value)
    }
}


impl FieldAccessor<Vec<f64>> for Vec<BostonRecord> {
    fn get_field(&self, field: &str) -> Option<Vec<f64>> {
        Some(self.iter()
            .filter_map(|r| r.get_field(field))
            .collect::<Vec<f64>>())
                
    }
}

pub trait DatasetUtils<T> {
    fn median(&self, field: &str) -> Result<T>;
    fn dich(&self, field: &str, median: Option<&T>) -> Result<Vec<u8>>;
}

impl DatasetUtils<f64> for Vec<BostonRecord> {
    fn median(&self, field: &str) -> Result<f64> {
        let mut sorted = match self.get_field(field) {
            Some(v) => v,
            None => return Err("Unable to filter vector for field".into())
        };

        sorted.sort_by(|a,b| a.total_cmp(b));
        let mid = sorted.len() / 2;

        if mid % 2 == 0 {
            Ok((sorted[mid-1] + sorted[mid]) / 2.0)   
        } else {
            Ok(sorted[mid])
        }

    }

    fn dich(&self, field: &str, median: Option<&f64>) -> Result<Vec<u8>> {
        // create a vec of u8 that represent a dichotomous variable
        let values = match self.get_field(field) {
            Some(v) => v,
            None => return Err("Failed to retrieve vector for dichotomous variable".into())
        };

        let m = match median {
            Some(v) => v,
            None => &self.median(field)
                .expect("Failed to build median for dichotomous variable")
        };

        Ok(values.iter()
            .map(|n| if n > m {1} else {0})
            .collect::<Vec<u8>>())
    }

}
