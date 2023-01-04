use super::{Op, Value};

pub enum BinaryOps {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Pow(Value, Value),
}

impl Op for BinaryOps {
    fn variables(&self) -> Vec<Value> {
        match self {
            Self::Add(lhs, rhs) => vec![lhs.clone(), rhs.clone()],
            Self::Sub(lhs, rhs) => vec![lhs.clone(), rhs.clone()],
            Self::Mul(lhs, rhs) => vec![lhs.clone(), rhs.clone()],
            Self::Div(numer, denom) => vec![numer.clone(), denom.clone()],
            Self::Pow(value, exponent) => vec![value.clone(), exponent.clone()],
        }
    }

    fn propagate(&self, source: &Value) {
        let grad = &source.grad().unwrap();

        match self {
            Self::Add(lhs, rhs) => {
                if lhs.should_compute_grad() {
                    *lhs.grad_mut() += grad;
                }

                if rhs.should_compute_grad() {
                    *rhs.grad_mut() += grad;
                }
            }
            Self::Sub(lhs, rhs) => {
                if lhs.should_compute_grad() {
                    *lhs.grad_mut() += grad;
                }

                if rhs.should_compute_grad() {
                    *rhs.grad_mut() += -grad;
                }
            }
            Self::Mul(lhs, rhs) => {
                if lhs.should_compute_grad() {
                    *lhs.grad_mut() += grad * rhs;
                }
                if rhs.should_compute_grad() {
                    *rhs.grad_mut() += grad * lhs;
                }
            }
            Self::Div(numer, denom) => {
                if numer.should_compute_grad() {
                    *numer.grad_mut() += grad / denom;
                }
                if denom.should_compute_grad() {
                    let derivative = -(numer / &denom.powf(2.0));
                    *denom.grad_mut() += grad * &derivative;
                }
            }
            Self::Pow(variable, exponent) => {
                if variable.should_compute_grad() {
                    let wrt_variable = &variable.pow(exponent - &1.0) * exponent;
                    *variable.grad_mut() += grad * &wrt_variable;
                }
                if exponent.should_compute_grad() {
                    let wrt_exponent = variable.log() * variable.pow(exponent.clone());
                    *exponent.grad_mut() += grad * &wrt_exponent;
                }
            }
        };
    }
}