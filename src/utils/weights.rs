use crate::{val, value::Value};
use rand::distributions::Uniform;
use rand_distr::{Distribution, Normal};

pub enum WeightInit {
    GlorotUniform,
    GlorotNormal,
    HeUniform,
    HeNormal,
    LecunUniform,
    LecunNormal,
}

pub struct Fanning(usize, usize);
impl From<[usize; 2]> for Fanning {
    fn from(val: [usize; 2]) -> Self {
        Fanning(val[0], val[1])
    }
}
impl From<[usize; 1]> for Fanning {
    fn from(val: [usize; 1]) -> Self {
        Fanning(val[0], val[0])
    }
}

impl WeightInit {
    pub fn sample<F: Into<Fanning>>(&self, fanning: F) -> Value {
        let Fanning(fan_in, fan_out) = fanning.into();

        match self {
            Self::GlorotUniform => self.glorot_uniform(fan_in, fan_out),
            Self::GlorotNormal => self.glorot_normal(fan_in, fan_out),
            Self::HeUniform => self.he_uniform(fan_in),
            Self::HeNormal => self.he_normal(fan_in),
            Self::LecunUniform => self.lecun_uniform(fan_in),
            Self::LecunNormal => self.lecun_normal(fan_in),
        }
    }

    fn glorot_uniform(&self, fan_in: usize, fan_out: usize) -> Value {
        let limit = (6.0 / (fan_in as f64 + fan_out as f64)).sqrt();
        let uniform = Uniform::new(-limit, limit);

        let mut rng = rand::thread_rng();

        let weight = uniform.sample(&mut rng);
        val!(weight)
    }

    fn glorot_normal(&self, fan_in: usize, fan_out: usize) -> Value {
        let stdev = (2.0 / (fan_in as f64 + fan_out as f64)).sqrt();
        let normal = Normal::new(0.0, stdev).unwrap();

        let mut rng = rand::thread_rng();

        let weight = normal.sample(&mut rng);
        val!(weight)
    }

    fn he_uniform(&self, fan_in: usize) -> Value {
        let limit = (6.0 / (fan_in as f64)).sqrt();
        let uniform = Uniform::new(-limit, limit);

        let mut rng = rand::thread_rng();

        let weight = uniform.sample(&mut rng);
        val!(weight)
    }

    fn he_normal(&self, fan_in: usize) -> Value {
        let stdev = (2.0 / fan_in as f64).sqrt();
        let normal = Normal::new(0.0, stdev).unwrap();

        let mut rng = rand::thread_rng();

        let weight = normal.sample(&mut rng);
        val!(weight)
    }

    fn lecun_uniform(&self, fan_in: usize) -> Value {
        let limit = (3.0 / fan_in as f64).sqrt();
        let uniform = Uniform::new(-limit, limit);

        let mut rng = rand::thread_rng();

        let weight = uniform.sample(&mut rng);
        val!(weight)
    }

    fn lecun_normal(&self, fan_in: usize) -> Value {
        let stdev = (1.0 / fan_in as f64).sqrt();
        let normal = Normal::new(0.0, stdev).unwrap();

        let mut rng = rand::thread_rng();

        let weight = normal.sample(&mut rng);
        val!(weight)
    }
}
