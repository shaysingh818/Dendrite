use crate::ndarray::ndarray::NDArray;
use crate::ndarray::ops::Ops;
use crate::autodiff::node::{Node, Value}; 
use std::cell::{RefCell, RefMut}; 


pub struct L2Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{
    pub rhs: RefCell<RHS>,
    pub lhs: RefCell<LHS>,
    pub output: RefCell<Value<NDArray<f64>>>,
    pub gradient: RefCell<Value<NDArray<f64>>>,
    pub learning_rate: f64
}


impl<RHS, LHS> L2Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{

    pub fn new(rhs: RHS, lhs: LHS, learning_rate: f64) -> Self {

        let weights = rhs.value();
        let w_square = weights.square().unwrap();
        let w_sum = w_square.sum().unwrap();
        let op_result = lhs.value().mult(w_sum).unwrap();
        let op_value = Value::new(&op_result);

        L2Regularization {
            rhs: RefCell::new(rhs),
            lhs: RefCell::new(lhs),
            output: RefCell::new(op_value.clone()),
            gradient: RefCell::new(op_value),
            learning_rate: learning_rate
        }
    }

    pub fn rhs(&self) -> RefMut<dyn Node> {
        self.rhs.borrow_mut()
    }

    pub fn lhs(&self) -> RefMut<dyn Node> {
        self.lhs.borrow_mut()
    }

}


impl<LHS, RHS> Node for L2Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{

    fn forward(&mut self) {

        self.rhs().forward();
        self.lhs().forward();

        let weights = self.rhs().value();
        let w_square = weights.square().unwrap();
        let w_sum = w_square.sum().unwrap();
        let op_result = self.lhs.borrow().value().mult(w_sum).unwrap();
        self.output = Value::new(&op_result).into(); 
    } 

    fn backward(&mut self, upstream_gradient: NDArray<f64>) {
        let lr = self.learning_rate / upstream_gradient.size() as f64;
        let alpha = self.lhs().value().scalar_mult(2.0 * lr).unwrap();
        let weight_update = self.rhs().value().scale_mult(alpha).unwrap();
        self.gradient = Value::new(&weight_update).into();
    }

    fn value(&self) -> NDArray<f64> {
        self.output.borrow().val().clone()
    }

    fn grad(&self) -> NDArray<f64> {
        self.gradient.borrow().val().clone()
    }

    fn set_grad(&mut self, upstream_gradient: NDArray<f64>) {
        self.gradient = Value::new(&upstream_gradient).into();
    } 
}


pub struct L1Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{
    pub rhs: RefCell<RHS>,
    pub lhs: RefCell<LHS>,
    pub output: RefCell<Value<NDArray<f64>>>,
    pub gradient: RefCell<Value<NDArray<f64>>>,
    pub learning_rate: f64
}


impl<RHS, LHS> L1Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{

    pub fn new(rhs: RHS, lhs: LHS, learning_rate: f64) -> Self {

        let weights = rhs.value();
        let w_abs = weights.abs().unwrap();
        let w_sum = w_abs.sum().unwrap();
        let op_result = lhs.value().mult(w_sum).unwrap();
        let op_value = Value::new(&op_result);

        L1Regularization {
            rhs: RefCell::new(rhs),
            lhs: RefCell::new(lhs),
            output: RefCell::new(op_value.clone()),
            gradient: RefCell::new(op_value),
            learning_rate: learning_rate
        }
    }

    pub fn rhs(&self) -> RefMut<dyn Node> {
        self.rhs.borrow_mut()
    }

    pub fn lhs(&self) -> RefMut<dyn Node> {
        self.lhs.borrow_mut()
    }

}


impl<LHS, RHS> Node for L1Regularization<RHS, LHS> 
where
    RHS: Node,
    LHS: Node,
{

    fn forward(&mut self) {

        self.rhs().forward();
        self.lhs().forward();

        let weights = self.rhs().value();
        let w_abs = weights.abs().unwrap();
        let w_sum = w_abs.sum().unwrap();
        let op_result = self.lhs.borrow().value().mult(w_sum).unwrap();
        self.output = Value::new(&op_result).into(); 
    } 

    fn backward(&mut self, upstream_gradient: NDArray<f64>) {
        let lr = self.learning_rate / upstream_gradient.size() as f64;
        let alpha = self.lhs().value().scalar_mult(lr).unwrap();
        let sig = self.rhs().value().signum().unwrap();
        let weight_update = sig.scale_mult(alpha).unwrap();
        self.gradient = Value::new(&weight_update).into();
        
    }

    fn value(&self) -> NDArray<f64> {
        self.output.borrow().val().clone()
    }

    fn grad(&self) -> NDArray<f64> {
        self.gradient.borrow().val().clone()
    }

    fn set_grad(&mut self, upstream_gradient: NDArray<f64>) {
        self.gradient = Value::new(&upstream_gradient).into();
    } 
}
