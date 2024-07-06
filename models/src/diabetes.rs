use postgres::{Client, NoTls, Error};
use arrow_schema::{Schema, Field, DataType};
use arrow_array::RecordBatch;
use arrow_array::array::{Float64Array};
use arrow::json::*; 
use std::sync::Arc;
use serde::{Serialize};

use ndarray::ndarray::NDArray;
use ndarray::ops::*;
use regression::logistic::Logistic;
use preprocessing::standard_scalar::*;


#[derive(Debug, serde::Deserialize, Serialize)]
pub struct Record {
    pregnancies: f64, 
    glucose: f64, 
    blood_pressure: f64, 
    skin_thickness: f64, 
    insulin: f64, 
    bmi: f64,
    pedigree_function: f64,
    age: f64,
    outcome: f64
}


pub struct DiabetesModel {
    pub records: Vec<Record>,
    pg_host: String,
    pg_user: String,
    pg_pass: String,
    database: String,
    view_name: String,
    learning_rate: f64,
    model: Logistic
}


impl DiabetesModel {

    pub fn new(
        pg_host: String, 
        pg_user: String, 
        pg_pass: String,
        database: String,
        view_name: String,
        learning_rate: f64) -> Self {

        Self {
            records: Vec::new(),
            pg_host: pg_host,
            pg_user: pg_user,
            pg_pass: pg_pass,
            database: database,
            view_name: view_name,
            learning_rate: learning_rate.clone(),
            model: Logistic::new(
                NDArray::new(vec![1, 1]).unwrap(),
                NDArray::new(vec![1, 1]).unwrap(),
                learning_rate
            ).unwrap()
        }
    }


    pub fn load_records(&mut self) -> Result<(), Error> {

        let conn_str = format!(
            "host={} user={} dbname={} password={}",
            self.pg_host, self.pg_user, self.database, self.pg_pass
        );

        let query_str = format!(
            "SELECT * FROM \"PUBLIC\".\"{}\"",
            self.view_name
        );
        
        let mut client = Client::connect(&conn_str,  NoTls)?;
        let results = client.query(&query_str, &[])?;

        for row in results { 
            let patient_record = Record {
                pregnancies: row.get(0),
                glucose: row.get(1),
                blood_pressure: row.get(2),
                skin_thickness: row.get(3),
                insulin: row.get(4),
                bmi: row.get(5),
                pedigree_function: row.get(6),
                age: row.get(7),
                outcome: row.get(8)
            };
            self.records.push(patient_record);
        }

        Ok(())
    }


    pub fn process_column(
        &self, 
        batch: 
        RecordBatch, 
        name: &str) -> Vec<f64> {

        batch.column_by_name(name)
            .unwrap()
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap()
            .iter()
            .flatten()
            .collect()
    }

    pub fn create_batch(&mut self) -> RecordBatch {

        let schema = Schema::new(vec![
            Field::new("pregnancies", DataType::Float64, false),
            Field::new("glucose", DataType::Float64, false),
            Field::new("blood_pressure", DataType::Float64, false),
            Field::new("skin_thickness", DataType::Float64, false),
            Field::new("insulin", DataType::Float64, false),
            Field::new("bmi", DataType::Float64, false),
            Field::new("pedigree_function", DataType::Float64, false),
            Field::new("age", DataType::Float64, false),
            Field::new("outcome", DataType::Float64, false)
        ]);

        let mut decoder = ReaderBuilder::new(Arc::new(schema))
            .build_decoder()
            .unwrap();

        self.load_records().unwrap();
        decoder.serialize(&self.records).unwrap();
        decoder.flush().unwrap().unwrap()
    }


    pub fn select_features(
        &self,
        batch: RecordBatch,
        input_cols: Vec<&str>,
        output_col: &str
    ) -> (NDArray<f64>, NDArray<f64>) {

        let mut feature_vec: Vec<f64> = Vec::new();
        for col in &input_cols {
            let mut feature = self.process_column(batch.clone(), col);
            feature_vec.append(&mut feature);
        }

        let output_col = self.process_column(batch.clone(), output_col);

        let temp: NDArray<f64> = NDArray::array(
            vec![input_cols.len(), output_col.len()], 
            feature_vec.clone()
        ).unwrap();
        let input = temp.transpose().unwrap();
        
        let output: NDArray<f64> = NDArray::array(
            vec![output_col.len(), 1], 
            output_col
        ).unwrap();

        (input, output)

    }


    pub fn train(&mut self) {

        let batch = self.create_batch();
        let (x_train, y_train) = self.select_features(
            batch,
            vec![
                "pregnancies", 
                "glucose",
                "blood_pressure",
                "skin_thickness",
                "insulin",
                "bmi",
                "pedigree_function",
                "age"
            ],
            "outcome"
        );

        let x_train_processed = min_max_scalar(x_train).unwrap(); 
        //let y_train_processed = min_max_scalar(y_train).unwrap(); 
        //println!("{:?}", y_train); 

        self.model = Logistic::new(
            x_train_processed, 
            y_train, 
            self.learning_rate
        ).unwrap();

        self.model.sgd(1000, true, 5); 
    }


    pub fn training_data(&mut self) -> (NDArray<f64>, NDArray<f64>) {
        let batch = self.create_batch();
        let (x_train, y_train) = self.select_features(
            batch,
            vec![
                "pregnancies", 
                "glucose",
                "blood_pressure",
                "skin_thickness",
                "insulin",
                "bmi",
                "pedigree_function",
                "age"
            ],
            "outcome"
        );

        let x_train_processed = min_max_scalar(x_train).unwrap();
        (x_train_processed, y_train)
    }


    pub fn save(&mut self, filepath: &str) -> std::io::Result<()> {
        self.model.save(filepath)?;
        Ok(())
    }


    pub fn load(&mut self, filepath: &str) {

        let batch = self.create_batch();
        let (x_train, y_train) = self.select_features(
            batch,
            vec![
                "pregnancies", 
                "glucose",
                "blood_pressure",
                "skin_thickness",
                "insulin",
                "bmi",
                "pedigree_function",
                "age"
            ],
            "outcome"
        );

        let x_train_processed = min_max_scalar(x_train).unwrap(); 

        self.model = Logistic::load(
            filepath,
            x_train_processed, 
            y_train,
            self.learning_rate
        ).unwrap();
    }


    pub fn predict(&mut self, input: NDArray<f64>)  -> NDArray<f64> {
        self.model.predict(input) 
    }  

}