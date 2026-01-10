// fiolet-core/src/types.rs

/// Simplified tensor type for no_std environments
#[derive(Debug, Clone)]
pub struct Tensor {
    pub data: &'static [f64],
    pub shape: TensorShape,
}

#[derive(Debug, Clone, Copy)]
pub struct TensorShape {
    pub dims: [usize; 4],
    pub rank: usize,
}

impl Tensor {
    pub fn new(data: &'static [f64], shape: TensorShape) -> Self {
        Self { data, shape }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
