use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

pub struct BinaryOperationExpressionTree {
    root: BinaryOperationLink,
}

pub type ExpressionTree = BinaryOperationExpressionTree;

impl Display for ExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let node = &self.root;

        fn _write(nbr: isize, f: &mut Formatter<'_>) -> Result {
            if nbr < 0 {
                write!(f, " {} {:08b} -{:#x}\n", nbr, nbr, nbr * -1)
            } else {
                write!(f, " {} {:08b} {:#x}\n", nbr, nbr, nbr)
            }
        }

        fn fmt_rec(node: &BOLink, f: &mut Formatter<'_>) -> Result {
            match node {
                BOLink::Null => {}
                BOLink::Operator(x) => {
                    fmt_rec(&x.left, f)?;
                    write!(f, "{}", x.operator)?;
                    fmt_rec(&x.right, f)?;

                    write!(f, "= ")?;
                    _write(x.res.unwrap(), f)?;
                }
                BOLink::Operand(x) => {
                    _write(**x, f)?;
                }
            };
            Ok(())
        }
        fmt_rec(node, f)
    }
}

type BOLink = BinaryOperationLink;

struct ResNode {
    operator: String,
    res: Option<isize>,
    left: BOLink,
    right: BOLink,
}

enum BinaryOperationLink {
    Null,
    Operator(Box<ResNode>),
    Operand(Box<isize>),
}

impl BinaryOperationExpressionTree {
    pub fn new() -> ExpressionTree {
        ExpressionTree { root: BOLink::Null }
    }

    fn new_node(&self, operator_: String, operand_x: BOLink, operand_y: BOLink) -> Box<ResNode> {
        Box::new(ResNode {
            operator: operator_,
            res: None,
            left: operand_x,
            right: operand_y,
        })
    }

    fn new_operand_node(&self, operand: String) -> Box<isize> {
        let nbr = operand.parse::<isize>().unwrap();

        Box::new(nbr)
    }

    fn is_operator(&self, expression: &String) -> bool {
        match expression.as_str() {
            "&" | "|" | "^" | "~" | ">>" | "<<" => true,
            _ => false,
        }
    }

    // [] = STACK
    // () = HEAP
    //
    // (12&3) | (42 ^ ~5)
    // expressions = ("12","3","&","42","5","~","^","|")
    //                                        [ptr]
    //                                          |
    //                            (BOL::Operator: '|'; res: ? )
    //                            /                           \
    //                           /                             \
    //                (BOL::Operator: '&'; res: ?)   (BOL::Operator: '^'; res: ?)
    //                /                \                /                \
    //   (BOL::Operand: 12)  (BOL::Operand: 3)    (BOL::Operand: 42)  (BOL::Operator: '~'; res: ?)
    //                                                                      /               \
    //                                                          (BOL::Operand: 5)          Null
    pub fn build(&mut self, expressions: &Vec<String>) {
        println!("expr {:?}", expressions);
        let mut stack: VecDeque<BOLink> = VecDeque::new();

        expressions.iter().for_each(|ex| {
            if self.is_operator(ex) {
                let x = stack.pop_back().unwrap();
                let y = if ex == "~" {
                    BOLink::Null
                } else {
                    stack.pop_back().unwrap()
                };
                let node = self.new_node(ex.to_string(), y, x);
                stack.push_back(BOLink::Operator(node));
            } else {
                stack.push_back(BOLink::Operand(self.new_operand_node(ex.to_string())));
            }
        });
        self.root = stack.pop_back().unwrap();
    }

    // (12&5) | 42 = 46
    // expressions = ("12","5","&","42","|")
    //                                        [ptr]
    //                                          |
    //                            (BOL::Operator: '|'; res: 46 )
    //                            /                           \
    //                           /                             \
    //                (BOL::Operator: '&'; res: 4)    (BOL::Operand: '42')
    //                /                \
    //   (BOL::Operand: 12)  (BOL::Operand: 5)
    fn evaluation(node: &mut BOLink) -> isize {
        match node {
            BOLink::Null => 0,
            BOLink::Operator(x) => {
                let operand_l = Self::evaluation(&mut x.left);
                let operand_r = Self::evaluation(&mut x.right);
                println!("left ::: {}", operand_l);
                println!("right ::: {}", operand_r);

                let r = match x.operator.as_str() {
                    "&" => operand_l & operand_r,
                    "|" => operand_l | operand_r,
                    "^" => operand_l ^ operand_r,
                    ">>" => operand_l >> operand_r,
                    "<<" => operand_l << operand_r,
                    "~" => !operand_r,
                    _ => panic!("There must be an error somewhere :/..."),
                };
                x.res = Some(r);
                r
            }
            BOLink::Operand(x) => **x,
        }
    }

    pub fn evaluate(&mut self) {
        let x = Self::evaluation(&mut self.root);
        println!("res = {}", x)
    }

    #[allow(dead_code)]
    pub fn read_infix_bot(&self) {
        self.read_infix(&self.root);
    }

    fn read_infix(&self, node: &BOLink) {
        match node {
            BOLink::Null => return,
            BOLink::Operator(x) => {
                self.read_infix(&x.left);
                println!("{} :: res {}", x.operator, x.res.unwrap_or(-0));
                self.read_infix(&x.right);
            }
            BOLink::Operand(x) => {
                println!("{}", x);
            }
        }
    }
}
