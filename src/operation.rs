use super::value::Value;

pub trait Backpropagation {
    fn propagate(&self, value: &f64, grad: &Value);
}

pub enum Op {
    Add(Value, Value),
    Mul(Value, Value),
    Pow(Value, f64),
    ReLu(Value),
    Tanh(Value),
    Softmax(Vec<Value>, Vec<f64>),
}

impl Op {
    pub fn equation(&self) -> Vec<Value> {
        match self {
            Op::Add(lhs, rhs) => vec![lhs.clone(), rhs.clone()],
            Op::Mul(lhs, rhs) => vec![lhs.clone(), rhs.clone()],
            Op::Pow(value, _) => vec![value.clone()],
            Op::ReLu(value) => vec![value.clone()],
            Op::Tanh(value) => vec![value.clone()],
            Op::Softmax(inputs, _) => inputs.clone(),
        }
    }
}

impl Backpropagation for Op {
    fn propagate(&self, value: &f64, grad: &Value) {
        match self {
            Op::Add(Value(lhs), Value(rhs)) => {
                let mut lhs_data = lhs.borrow_mut();
                *lhs_data.grad_mut() += grad;

                let mut rhs_data = rhs.borrow_mut();
                *rhs_data.grad_mut() += grad;
            }
            Op::Mul(lhs, rhs) => {
                *lhs.0.borrow_mut().grad_mut() += grad * rhs;
                *rhs.0.borrow_mut().grad_mut() += grad * lhs;
            }
            Op::Pow(variable, exponent) => {
                let derivative = &variable.powf(exponent - 1.0) * exponent;

                *variable.0.borrow_mut().grad_mut() += grad * &derivative;
            }
            Op::ReLu(Value(unactivated)) => {
                let mut unactivated_data = unactivated.borrow_mut();

                let one_if_greater_than_zero = Value::from(value.ceil().min(1.0));

                *unactivated_data.grad_mut() += grad * &one_if_greater_than_zero;
            }
            Op::Tanh(Value(unactivated)) => {
                let mut unactivated_data = unactivated.borrow_mut();

                let derivative = Value::from(1.0 - value.powf(2.0));

                *unactivated_data.grad_mut() += grad * &derivative;
            }
            Op::Softmax(unactivated_inputs, jacobian) => {
                for (idx, unactivated) in unactivated_inputs.iter().enumerate() {
                    let mut unactivated_data = unactivated.0.borrow_mut();

                    *unactivated_data.grad_mut() += grad * &jacobian[idx];
                }
            }
        };
    }
}
