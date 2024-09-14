
#[cfg(test)]
mod naive_bayes_tests {

    use ndarray::ndarray::NDArray;
    use ndarray::ops::*;
    use bayes::naive_bayes::*;

    #[test]
    fn test_class_indices() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let x_path_2 = "data/weather_multi_feature/inputs";
        let y_path_2 = "data/weather_multi_feature/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        let features2 = NDArray::load(x_path_2).unwrap();
        let target2 = NDArray::load(y_path_2).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        assert_eq!(features2.shape().values(), vec![14, 4]);
        assert_eq!(target2.shape().values(), vec![14, 1]);

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let class_idxs = clf.class_idxs();
        assert_eq!(class_idxs.len(), 2);
        assert_eq!(class_idxs[0].len(), 4);
        assert_eq!(class_idxs[1].len(), 10);

        let expected: Vec<Vec<usize>> = vec![
            vec![4, 8, 9, 11],
            vec![0, 1, 2, 3, 5, 6, 7, 10, 12, 13],
        ];

        let mut idx = 0;
        for item in &expected {
            assert_eq!(class_idxs[idx], *item);
            idx += 1; 
        } 

        let clf2 = NaiveBayes::new(
            &features2,
            &target2
        ).unwrap(); 

        let class_idxs_2 = clf2.class_idxs();
        assert_eq!(class_idxs_2.len(), 2);
        assert_eq!(class_idxs_2[0].len(), 5);
        assert_eq!(class_idxs_2[1].len(), 9);

        let expected2: Vec<Vec<usize>> = vec![
            vec![0, 1, 5, 7, 13],
            vec![2, 3, 4, 6, 8, 9, 10, 11, 12],
        ];

        idx = 0;
        for item in &expected2 {
            assert_eq!(class_idxs_2[idx], *item);
            idx += 1; 
        } 

    }


    #[test]
    fn test_class_probabilities() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let x_path_2 = "data/weather_multi_feature/inputs";
        let y_path_2 = "data/weather_multi_feature/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        let features2 = NDArray::load(x_path_2).unwrap();
        let target2 = NDArray::load(y_path_2).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        assert_eq!(features2.shape().values(), vec![14, 4]);
        assert_eq!(target2.shape().values(), vec![14, 1]);

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let class_probs = clf.class_probabilities();
        assert_eq!(class_probs.len(), 2);

        let expected = vec![
            0.2857142857142857,
            0.7142857142857143
        ];

        let mut idx = 0;
        for item in &expected {
            assert_eq!(class_probs[idx], *item);
            idx += 1; 
        }

        let clf2 = NaiveBayes::new(
            &features2,
            &target2
        ).unwrap(); 

        let class_probs_2 = clf2.class_probabilities();
        assert_eq!(class_probs_2.len(), 2);

        let expected2 = vec![
            0.35714285714285715,
            0.6428571428571429
        ];

        idx = 0;
        for item in &expected2 {
            assert_eq!(class_probs_2[idx], *item);
            idx += 1; 
        }


    }


    #[test]
    fn test_frequency_table() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let x_path_2 = "data/weather_multi_feature/inputs";
        let y_path_2 = "data/weather_multi_feature/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        let features2 = NDArray::load(x_path_2).unwrap();
        let target2 = NDArray::load(y_path_2).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        assert_eq!(features2.shape().values(), vec![14, 4]);
        assert_eq!(target2.shape().values(), vec![14, 1]);

        let feature1 = features.axis(1, 0).unwrap();
        let feature2 = features2.axis(1, 1).unwrap();

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let clf2 = NaiveBayes::new(
            &features2,
            &target2
        ).unwrap();

        let class_idxs = clf.class_idxs();
        let freq_table = clf.frequency_table(
            feature1, 
            class_idxs.clone()
        ).unwrap();
        let table_rows = freq_table.shape().dim(0);

        assert_eq!(class_idxs.len(), 2);
        assert_eq!(class_idxs[0].len(), 4);
        assert_eq!(class_idxs[1].len(), 10);
        assert_eq!(freq_table.shape().values(), vec![3, 3]);

        let class_idxs_2 = clf2.class_idxs();
        let freq_table_2 = clf2.frequency_table(
            feature2, 
            class_idxs.clone()
        ).unwrap();
        let table_rows_2 = freq_table_2.shape().dim(0);

        assert_eq!(class_idxs_2.len(), 2);
        assert_eq!(class_idxs_2[0].len(), 5);
        assert_eq!(class_idxs_2[1].len(), 9);
        assert_eq!(freq_table_2.shape().values(), vec![3, 3]);

        let expected: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 2.0],
            vec![1.0, 2.0, 3.0],
            vec![2.0, 0.0, 5.0],
        ];

        let expected2: Vec<Vec<f64>> = vec![
            vec![0.0, 0.0, 4.0],
            vec![1.0, 2.0, 4.0],
            vec![2.0, 2.0, 2.0],
        ];

        let mut idx = 0;
        for row in 0..table_rows {
            let item = freq_table.axis(0, row).unwrap(); 
            assert_eq!(item.values(), &expected[idx]);
            idx += 1;
        } 

        idx = 0;
        for row in 0..table_rows_2 {
            let item = freq_table_2.axis(0, row).unwrap(); 
            assert_eq!(item.values(), &expected2[idx]);
            idx += 1;
        }  

    }

    
    #[test]
    fn test_likelihood_table() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let x_path_2 = "data/weather_multi_feature/inputs";
        let y_path_2 = "data/weather_multi_feature/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        let features2 = NDArray::load(x_path_2).unwrap();
        let target2 = NDArray::load(y_path_2).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        assert_eq!(features2.shape().values(), vec![14, 4]);
        assert_eq!(target2.shape().values(), vec![14, 1]);

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let clf2 = NaiveBayes::new(
            &features2,
            &target2
        ).unwrap();

        let expected: Vec<Vec<f64>> = vec![
            vec![0.0, 0.5, 0.2],
            vec![1.0, 0.5, 0.3],
            vec![2.0, 0.0, 0.5]
        ]; 

        let expected2: Vec<Vec<f64>> = vec![
            vec![0.0, 0.6, 0.2222222222222222],
            vec![1.0, 0.0, 0.4444444444444444],
            vec![2.0, 0.4, 0.3333333333333333]
        ]; 

        let class_idxs = clf.class_idxs();
        let freq_table = clf.frequency_table(
            features.axis(1, 0).unwrap(), 
            class_idxs
        ).unwrap();

        let class_idxs_2 = clf2.class_idxs();
        let freq_table_2 = clf2.frequency_table(
            features2.axis(1, 0).unwrap(), 
            class_idxs_2
        ).unwrap();

        let mut idx = 0; 
        let lh_table = clf.likelihood_table(freq_table);
        let table_rows = lh_table.shape().dim(0);
        for row in 0..table_rows {
            let item = lh_table.axis(0, row).unwrap();
            assert_eq!(item.values(), &expected[idx]);
            idx += 1;
        }

        idx = 0; 
        let lh_table_2 = clf2.likelihood_table(freq_table_2);
        let table_rows_2 = lh_table_2.shape().dim(0);
        for row in 0..table_rows_2 {
            let item = lh_table_2.axis(0, row).unwrap();
            assert_eq!(item.values(), &expected2[idx]);
            idx += 1;
        }

    }


    #[test]
    fn test_fit_single_feature() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let mut class_idx: f64 = 1.0;
        let sunny = clf.predict_feature(0, 1.0, class_idx);
        let class_prob = clf.class_probabilities()[class_idx as usize];
        let prior = clf.feature_prior_probability(0, 1.0);
        let output = sunny * class_prob / prior;
        assert_eq!(output, 0.6); 

        class_idx = 0.0;
        let sunny_no = clf.predict_feature(0, 1.0, class_idx);
        let class_prob_no = clf.class_probabilities()[class_idx as usize];
        let prior_no = clf.feature_prior_probability(0, 1.0);
        let output_no = sunny_no * class_prob_no / prior_no;
        assert_eq!(output_no, 0.39999999999999997);

    } 


    #[test]
    fn test_fit_multi_feature() {

        let x_path_2 = "data/weather_multi_feature/inputs";
        let y_path_2 = "data/weather_multi_feature/outputs";

        let features2 = NDArray::load(x_path_2).unwrap();
        let target2 = NDArray::load(y_path_2).unwrap();

        assert_eq!(features2.shape().values(), vec![14, 4]);
        assert_eq!(target2.shape().values(), vec![14, 1]);

        let clf2 = NaiveBayes::new(
            &features2,
            &target2
        ).unwrap();

        let class_idx: f64 = 1.0;
        let overcast = clf2.predict_feature(0, 1.0, class_idx);
        let mild = clf2.predict_feature(1, 1.0, class_idx);
        let output = overcast * mild;
        assert_eq!(output, 0.19753086419753085);

        let class_prob = clf2.class_probabilities()[class_idx as usize];
        assert_eq!(class_prob, 0.6428571428571429);

        let final_output = output * class_prob;
        assert_eq!(final_output, 0.12698412698412698);

    } 


    #[test]
    fn test_feature_prior() {

        let x_path = "data/weather/inputs";
        let y_path = "data/weather/outputs";

        let features = NDArray::load(x_path).unwrap();
        let target = NDArray::load(y_path).unwrap();

        assert_eq!(features.shape().values(), vec![14, 1]);
        assert_eq!(target.shape().values(), vec![14, 1]);

        let clf = NaiveBayes::new(
            &features,
            &target
        ).unwrap();

        let prior = clf.feature_prior_probability(0, 1.0);
        assert_eq!(prior, 0.35714285714285715); 

    }


    #[test]
    fn test_naive_bayes_error_handling() {

        let features: NDArray<f64> = NDArray::array(
            vec![3, 3],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        ).unwrap();

        let outputs: NDArray<f64> = NDArray::array(
            vec![4, 1],
            vec![0.0, 1.0, 2.0, 3.0]
        ).unwrap();

        let clf = NaiveBayes::new(&features, &outputs);
        assert_eq!(
            clf.unwrap_err(), 
            "Feature rows must match output rows"
        ); 

    }


    #[test]
    fn test_frequency_table_error_handling() {

        let features: NDArray<f64> = NDArray::array(
            vec![3, 6],
            vec![
                0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0
            ]
        ).unwrap();

        let features_bad: NDArray<f64> = NDArray::array(
            vec![2, 2],
            vec![0.0, 1.0, 2.0, 3.0]
        ).unwrap();

        let outputs: NDArray<f64> = NDArray::array(
            vec![3, 1],
            vec![0.0, 1.0, 2.0]
        ).unwrap();

        let clf = NaiveBayes::new(&features, &outputs).unwrap();
        let class_idxs = clf.class_idxs();
        let freq_table = clf.frequency_table(
            features, 
            class_idxs.clone()
        );

        assert_eq!(
            freq_table.unwrap_err(),
            "Feature to frequency table must be shape (N, 1)"
        );

        let freq_table_bad = clf.frequency_table(
            features_bad, 
            class_idxs.clone()
        );

        assert_eq!(
            freq_table_bad.unwrap_err(),
            "Rows of feature must match rows of output"
        );

    }

}



