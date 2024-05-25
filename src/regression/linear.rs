use crate::ndarray::ndarray::NDArray;
use crate::ndarray::ops::*;
use crate::loss::mse::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    features: NDArray<f64>,
    outputs: NDArray<f64>,
    predicted_outputs: NDArray<f64>,
    weights: NDArray<f64>, 
    bias: f64,
    learning_rate: f64,
    loss_function: fn(y_true: &NDArray<f64>, y_pred: &NDArray<f64>) -> Result<f64, String>,
    model_loss: f64
}


impl Linear {

    pub fn new(features: NDArray<f64>, y: NDArray<f64>, learning_rate: f64) -> Result<Linear, String> {

        if learning_rate < 0.0 || learning_rate > 1.0 {
            return Err("Learning rate must be between 1 and 0".to_string());
        }

        Ok(Self {
            features: features.clone(),
            outputs: y.clone(),
            predicted_outputs: NDArray::new(y.shape().to_vec()).unwrap(),
            weights: NDArray::new(vec![features.shape()[1], 1]).unwrap(),
            bias: 0.00,
            learning_rate: learning_rate,
            loss_function: mse,
            model_loss: 0.0
        })
    }

    pub fn set_loss(&mut self, loss_func: fn(y_true: &NDArray<f64>, y_pred: &NDArray<f64>) -> Result<f64, String>) {
        self.loss_function = loss_func;
    }


    pub fn predict(&mut self, inputs: NDArray<f64>) -> Result<NDArray<f64>, String> {

        if inputs.shape() != self.features.shape() {
            return Err("Provided shape values don't match model params".to_string());
        }

        self.features = inputs; 
        let y_pred = self.forward().unwrap();
        Ok(y_pred)
    }


    pub fn forward(&self) -> Result<NDArray<f64>, String> {
        let result = self.features.dot(self.weights.clone()).unwrap();
        let bias_add = result.scalar_add(self.bias).unwrap();
        Ok(bias_add)
    }

    pub fn weight_update(&mut self, y_pred: NDArray<f64>) {
        let x_t = self.features.clone().transpose().unwrap();
        let error = y_pred.subtract(self.outputs.clone()).unwrap();
        let grad = x_t.dot(error).unwrap();
        let d_w = grad.scalar_mult(self.learning_rate / y_pred.size() as f64).unwrap(); 
        self.weights = self.weights.subtract(d_w).unwrap(); 
    }

    pub fn bias_update(&mut self, y_pred: NDArray<f64>)  {
        let error = y_pred.subtract(self.outputs.clone()).unwrap();
        let grad = self.learning_rate/y_pred.size() as f64;
        let db: f64 = error.scalar_mult(grad).unwrap().values().iter().sum();
        self.bias = self.bias - db; 
    }

    pub fn train(&mut self, epochs: usize, log_output: bool, batch_size: usize) {

        let mut loss: f64 = 0.0;

        if batch_size > 0 {

            let input_train: Vec<NDArray<f64>> = self.features.batch(batch_size).unwrap();
            let output_train: Vec<NDArray<f64>> = self.outputs.batch(batch_size).unwrap();

            for epoch in 0..epochs {

                let mut batch_index = 0; 
                for batch in &input_train {

                    self.features = batch.clone(); 
                    self.outputs = output_train[batch_index].clone();

                    let y_pred = self.forward().unwrap();
                    loss = (self.loss_function)(&y_pred, &self.outputs).unwrap(); 
                    self.weight_update(y_pred.clone());
                    self.bias_update(y_pred); 
                    batch_index += 1; 
                }

                if log_output {
                    println!("Epoch [{:?}/{:?}]: {:?}", epoch, epochs, loss);
                }
            }

            self.model_loss = loss;

        } else {

            self.forward().unwrap();
            for epoch in 0..epochs {
                let  y_pred = self.forward().unwrap(); 
                let loss = (self.loss_function)(&y_pred, &self.outputs).unwrap(); 
                self.weight_update(y_pred.clone());
                self.bias_update(y_pred); 

                if log_output {
                    println!("Epoch [{:?}/{:?}]: {:?}", epoch, epochs, loss);
                }
            }

            self.model_loss = loss; 

        }


    }

}