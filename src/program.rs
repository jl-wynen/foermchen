use crate::expression::Expression;
use crate::instruction::Instruction;

pub struct Program {
    pub symbol_table: SymbolTable,
    pub expression: Expression,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            expression: Expression::new(),
            instructions: Vec::new(),
        }
    }
}

pub struct SymbolTable {
    symbols: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str) -> usize {
        match self.find_symbol(&name) {
            Some(i) => i,
            None => {
                self.symbols.push(name.to_owned());
                self.symbols.len() - 1
            }
        }
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        self.find_symbol(&name).is_some()
    }

    fn find_symbol(&self, name: &str) -> Option<usize> {
        self.symbols.iter().position(|x| x == name)
    }

    pub fn repr(&self, symbol: usize) -> Result<String, String> {
        if symbol < self.symbols.len() {
            Ok(self.symbols[symbol as usize].clone())
        } else {
            Err("Unknown symbol".to_owned())
        }
    }
}
