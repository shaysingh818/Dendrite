
#[cfg(test)]
mod logistic_tests {

    use regression::logistic::Logistic;
    use ndarray::ndarray::NDArray;
    use ndarray::ops::*;
    use metrics::loss::*;
    use metrics::activations::*;

    #[test]
    fn test_logistic_model() {

        let x_path = "data/logistic_modeling_data/inputs";
        let y_path = "data/logistic_modeling_data/outputs"; 

        let x: NDArray<f64> = NDArray::load(x_path).unwrap();
        let y: NDArray<f64> = NDArray::load(y_path).unwrap();

        let model = Logistic::new(
            x.clone(), 
            y.clone(), 
            sigmoid_vec,
            0.01
        ).unwrap();

        assert_eq!(model.features.val().shape().values(), x.shape().values()); 
        assert_eq!(model.features.val().rank(), 2); 
        assert_eq!(
            model.features.val().values(), 
            x.values()
        );

        assert_eq!(model.outputs.val().shape().values(), y.shape().values()); 
        assert_eq!(model.outputs.val().rank(), 2); 
        assert_eq!(
            model.outputs.val().values(), 
            y.values()
        );

        let expected_shape = vec![model.features.val().shape().dim(1), 1];
        assert_eq!(model.weights.val().shape().values(), expected_shape); 
        assert_eq!(model.weights.val().rank(), 2);
        for item in model.weights.val().values() {
            let expected: f64 = 0.0;
            assert_eq!(item, &expected); 
        }

        let expected_shape = vec![1, 1];
        assert_eq!(model.bias.val().shape().values(), expected_shape); 
        assert_eq!(model.bias.val().rank(), 2);
        for item in model.bias.val().values() {
            let expected: f64 = 0.0;
            assert_eq!(item, &expected); 
        }

    }

    #[test]
    fn test_logistic_train() {

        let x_path = "data/logistic_modeling_data/inputs";
        let y_path = "data/logistic_modeling_data/outputs"; 

        let x: NDArray<f64> = NDArray::load(x_path).unwrap();
        let y: NDArray<f64> = NDArray::load(y_path).unwrap();

        let mut model = Logistic::new(
            x.clone(), 
            y.clone(), 
            sigmoid_vec,
            0.01
        ).unwrap();

        let weights_binding = model.weights.val(); 
        let bias_binding = model.bias.val();
        let weights_prior = weights_binding.values(); 
        let bias_prior = bias_binding.values(); 

        model.train(1000, false);

        let w_binding = model.weights.val(); 
        let b_binding = model.bias.val(); 
        let weights_after = w_binding.values(); 
        let bias_after = b_binding.values(); 

        assert_ne!(weights_prior, weights_after); 
        assert_ne!(bias_prior, bias_after);

        let results = model.predict(x);
        let loss = mse(&results, &y).unwrap();
        let loss_condition = loss < 0.1; 
        assert_eq!(loss_condition, true); 

    }

    #[test]
    fn test_logistic_sgd() {
        
        let x_path = "data/logistic_modeling_data/inputs";
        let y_path = "data/logistic_modeling_data/outputs"; 

        let batch_size: usize = 2; 
        let x: NDArray<f64> = NDArray::load(x_path).unwrap();
        let y: NDArray<f64> = NDArray::load(y_path).unwrap();

        let mut model = Logistic::new(
            x.clone(), 
            y.clone(), 
            sigmoid_vec,
            0.001
        ).unwrap();

        let weights_binding = model.weights.val(); 
        let bias_binding = model.bias.val();
        let weights_prior = weights_binding.values(); 
        let bias_prior = bias_binding.values(); 

        model.sgd(1000, false, 2);

        let w_binding = model.weights.val(); 
        let b_binding = model.bias.val(); 
        let weights_after = w_binding.values(); 
        let bias_after = b_binding.values();

        assert_ne!(weights_prior, weights_after); 
        assert_ne!(bias_prior, bias_after);

        let x_train = x.batch(batch_size).unwrap();
        let y_train = y.batch(batch_size).unwrap();
        let results = model.predict(x_train[1].clone());
        let loss = mse(&results, &y_train[1].clone()).unwrap();
        let loss_condition = loss < 0.1; 
        assert_eq!(loss_condition, true);

    }


    #[test]
    fn test_logistic_save_load() -> std::io::Result<()> {

        let model_path = "data/models/logistic";
        let x_path = "data/logistic_modeling_data/inputs";
        let y_path = "data/logistic_modeling_data/outputs"; 

        let batch_size: usize = 2; 
        let x: NDArray<f64> = NDArray::load(x_path).unwrap();
        let y: NDArray<f64> = NDArray::load(y_path).unwrap();

        let mut model = Logistic::new(
            x.clone(),
            y.clone(), 
            sigmoid_vec,
            0.001
        ).unwrap();

        model.sgd(1000, false, 2);
        model.save(model_path).unwrap();

        let x_train = x.batch(batch_size).unwrap();
        let y_train = y.batch(batch_size).unwrap();

        let mut loaded_model = Logistic::load(
            model_path, 
            x.clone(), 
            y.clone(), 
            sigmoid_vec,
            0.01
        ).unwrap();

        let results = loaded_model.predict(x_train[1].clone());
        let loss = mse(&results, &y_train[1].clone()).unwrap();
        let loss_condition = loss < 0.1; 
        assert_eq!(loss_condition, true); 

        Ok(())

    }

    #[test]
    fn test_multi_class_logistic() {

        // multi class data
        let x_path = "data/logistic_modeling_data/multi_class_input";
        let y_path = "data/logistic_modeling_data/multi_class_output";

        let x_train = NDArray::load(x_path).unwrap();
        let y_train = NDArray::load(y_path).unwrap();


    }

}
