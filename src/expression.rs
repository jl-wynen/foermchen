pub struct Expression {
    pub terms: Vec<Term>,
}

impl Expression {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }
}

pub struct Term {
    pub coef: i32,
    pub factors: Vec<usize>,
}

impl Term {
    pub fn new() -> Self {
        Self {
            coef: 1,
            factors: Vec::new(),
        }
    }
}
