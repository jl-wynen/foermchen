use crate::lexer::{Lexer, Token};
use crate::program::{Program, SymbolTable};
use crate::expression::{Expression, Term};

pub fn parse(code: &str) -> Result<Program, String> {
    let mut lexer = Lexer::new(code);
    let mut prog = Program::new();
    parse_expression(&mut lexer, &mut prog.expression, &mut prog.symbol_table)?;
    Ok(prog)
}

enum Expect {
    Op, Var
}

fn parse_expression(lexer: &mut Lexer, expr: &mut Expression, symbol_table: &mut SymbolTable) -> Result<(), String> {
    let mut term = Term::new();
    let mut expect = Expect::Var;
    let mut negative = false;
    let mut have_term = false;

    while let Some(token) = lexer.next_token()? {
        match token {
            Token::Name(name) => {
                if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Var) {
                    term.factors.push(symbol_table.add(&name));
                    expect = Expect::Op;
                    have_term = true;
                }
                else {
                    return Err("Expecting an operator, found a variable".to_owned());
                }
            },
            Token::Number(num) => {
                if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Var) {
                    term.coef *= num;
                    expect = Expect::Op;
                    have_term = true;
                }
                else {
                    return Err("Expecting an operator, found a number".to_owned());
                }
            },
            Token::Op(op) => {
                match op {
                    '*' => {
                        if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Op) {
                            expect = Expect::Var;  // just multiply by the next var
                        }
                        else {
                            return Err("Expecting variable, found '*'".to_owned());
                        }
                    },
                    '+' => {
                        if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Op) {  // finish term and move on to next
                            if negative {
                                term.coef *= -1;
                            }
                            expr.terms.push(term);
                            term = Term::new();
                            negative = false;
                            expect = Expect::Var;
                            have_term = false;
                        }
                        else {
                            return Err("Expecting variable, found '+'".to_owned());
                        }
                    },
                    '-' => {
                        if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Op) {  // finish term and move on to next
                            if negative {
                                term.coef *= -1;
                            }
                            expr.terms.push(term);
                            term = Term::new();
                            negative = false;
                            expect = Expect::Var;
                            have_term = false;
                        }
                        // regardless of expected, next coef is negated
                        negative = !negative;
                    },
                    _ => return Err("Unknown operator".to_owned()),
                }
            },
        };
    }

    if std::mem::discriminant(&expect) == std::mem::discriminant(&Expect::Var) {
        // incomplete op
        return Err("Expression ended, expected variable".to_owned());
    }

    if have_term {
        if negative {
            term.coef *= -1;
        }
        expr.terms.push(term);
    }

    Ok(())
}
