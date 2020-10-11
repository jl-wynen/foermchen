mod expression;
mod lexer;
mod parser;
mod program;

use parser::parse;
use program::Program;

const PROGRAM: &str = " abc + 2 *  3 -1*x + -3*r*-2 +0*3*abc - 5*abc*x";

fn main() {
    let prog = parse(PROGRAM).unwrap();
    for term in prog.expression.terms {
        print!("{}", term.coef);
        for var in term.factors {
            print!(" * {}({})", prog.symbol_table.repr(var).unwrap(), var);
        }
        println!();
    }
}
