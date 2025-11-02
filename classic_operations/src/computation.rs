// badprog.com
pub struct Computation {
    val_add: f64,
    val_sub: f64,
    val_div: f64,
    val_mul: f64,
}

impl Computation {
    // ========================================================================
    // new()
    pub fn new() -> Self {
        Self {
            val_add: 0.0,
            val_sub: 0.0,
            val_div: 0.0,
            val_mul: 0.0,
        }
    }

    // ========================================================================
    // add()
    pub fn add(&mut self, left: f64, right: f64) -> f64 {
        let result = left + right;

        self.val_add += result;

        result
    }

    // ========================================================================
    // get_val_add()
    pub fn get_val_add(&self) -> f64 {
        self.val_add
    }

    // ========================================================================
    // sub()
    pub fn sub(&mut self, left: f64, right: f64) -> f64 {
        let result = left - right;

        self.val_sub += result;

        result
    }

    // ========================================================================
    // get_val_sub()
    pub fn get_val_sub(&self) -> f64 {
        self.val_sub
    }

    // ========================================================================
    // div()
    pub fn div(&mut self, left: f64, right: f64) -> f64 {
        let result = left / right;

        self.val_div += result;

        result
    }

    // ========================================================================
    // get_val_div()
    pub fn get_val_div(&self) -> f64 {
        self.val_div
    }

    // ========================================================================
    // mul()
    pub fn mul(&mut self, left: f64, right: f64) -> f64 {
        let result = left * right;

        self.val_mul += result;

        result
    }

    // ========================================================================
    // get_val_div()
    pub fn get_val_mul(&self) -> f64 {
        self.val_mul
    }
}

mod tests {
    // use super::*;

    #[test]
    fn test_computation_add() {
        let left = 10;
        let right = 5;
        let expected_value = 15;
        let mut comp = computation::Computation::new();
        let result = comp.add(left, right);
        // assert_eq
        assert_eq!(result, expected_value);
        assert_eq!(comp.get_val_add(), expected_value);
    }

    #[test]
    fn test_computation_sub() {
        let left = 10;
        let right = 5;
        let expected_value = 5;
        let mut comp = computation::Computation::new();
        let result = comp.sub(left, right);
        // assert_eq
        assert_eq!(result, expected_value);
        assert_eq!(comp.get_val_sub(), expected_value);
    }

    #[test]
    fn test_computation_div() {
        let left = 10;
        let right = 5;
        let expected_value = 2;
        let mut comp = computation::Computation::new();
        let result = comp.div(left, right);
        // assert_eq
        assert_eq!(result, expected_value);
        assert_eq!(comp.get_val_div(), expected_value);
    }

    #[test]
    fn test_computation_mul() {
        let left = 10;
        let right = 5;
        let expected_value = 50;
        let mut comp = computation::Computation::new();
        let result = comp.mul(left, right);
        // assert_eq
        assert_eq!(result, expected_value);
        assert_eq!(comp.get_val_mul(), expected_value);
    }
}
