pub struct ValueVec {
    pub values: Vec<f64>,
}

impl ValueVec {
    pub fn new() -> ValueVec {
        ValueVec { values: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).cloned()
    }
}
