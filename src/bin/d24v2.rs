use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum OpCode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, PartialEq)]
enum RHType {
    Num,
    Reg,
}

#[derive(Debug)]
struct Operand {
    typ: RHType,
    value: isize,
    reg: usize,
}

#[derive(Debug)]
struct Command {
    op: OpCode,
    lh: usize,
    rh: Operand,
}

struct OperandCheck {
    typ: RHType,
    value: Option<isize>,
    reg: Option<usize>,
}

struct CommandCheck {
    op: OpCode,
    lh: usize,
    rh: Option<OperandCheck>,
}

impl CommandCheck {
    fn new_ignore_operand(op: OpCode, lh: usize) -> CommandCheck {
        CommandCheck { op, lh, rh: None }
    }

    fn new_ignore_rh_num(op: OpCode, lh: usize) -> CommandCheck {
        CommandCheck {
            op,
            lh,
            rh: Some(OperandCheck {
                typ: RHType::Num,
                value: None,
                reg: None,
            }),
        }
    }

    fn new_with_num(op: OpCode, lh: usize, num: isize) -> CommandCheck {
        CommandCheck {
            op,
            lh,
            rh: Some(OperandCheck {
                typ: RHType::Num,
                value: Some(num),
                reg: None,
            }),
        }
    }

    fn new_with_reg(op: OpCode, lh: usize, reg: usize) -> CommandCheck {
        CommandCheck {
            op,
            lh,
            rh: Some(OperandCheck {
                typ: RHType::Reg,
                value: None,
                reg: Some(reg),
            }),
        }
    }
}

fn reg_to_ix(reg: &str) -> usize {
    match reg {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("Unknown register {}", reg),
    }
}

fn get_operand(rhs: &str) -> Operand {
    if let Ok(num_parse_res) = rhs.parse::<isize>() {
        Operand {
            typ: RHType::Num,
            value: num_parse_res,
            reg: 0,
        }
    } else {
        Operand {
            typ: RHType::Reg,
            value: 0,
            reg: reg_to_ix(rhs),
        }
    }
}

fn test_seq() -> Vec<CommandCheck> {
    vec![
        CommandCheck::new_ignore_operand(OpCode::Inp, 0), // 01 inp w
        CommandCheck::new_with_num(OpCode::Mul, 1, 0),    // 02 mul x 0
        CommandCheck::new_with_reg(OpCode::Add, 1, 3),    // 03 add x z
        CommandCheck::new_with_num(OpCode::Mod, 1, 26),   // 04 mod x 26
        CommandCheck::new_ignore_rh_num(OpCode::Div, 3),  // 05 div z ?  <-- A
        CommandCheck::new_ignore_rh_num(OpCode::Add, 1),  // 06 add x ?  <-- B
        CommandCheck::new_with_reg(OpCode::Eql, 1, 0),    // 07 eql x w
        CommandCheck::new_with_num(OpCode::Eql, 1, 0),    // 08 eql x 0
        CommandCheck::new_with_num(OpCode::Mul, 2, 0),    // 09 mul y 0
        CommandCheck::new_ignore_rh_num(OpCode::Add, 2),  // 10 add y ?  <-- C
        CommandCheck::new_with_reg(OpCode::Mul, 2, 1),    // 11 mul y x
        CommandCheck::new_with_num(OpCode::Add, 2, 1),    // 12 add y 1
        CommandCheck::new_with_reg(OpCode::Mul, 3, 2),    // 13 mul z y
        CommandCheck::new_with_num(OpCode::Mul, 2, 0),    // 14 mul y 0
        CommandCheck::new_with_reg(OpCode::Add, 2, 0),    // 15 add y w
        CommandCheck::new_ignore_rh_num(OpCode::Add, 2),  // 16 add y ?  <-- D
        CommandCheck::new_with_reg(OpCode::Mul, 2, 1),    // 17 mul y x
        CommandCheck::new_with_reg(OpCode::Add, 3, 2),    // 18 add z y
    ]
}

type Abcd = (isize, isize, isize, isize);

fn get_abcd(code: &[Command]) -> Vec<Abcd> {
    let mut result = Vec::new();
    let test = test_seq();
    assert!(code.len() % test.len() == 0);
    let (mut a, mut b, mut c) = (0, 0, 0);
    for (i, x) in code.iter().enumerate() {
        let ix = i % test.len();
        let check = &test[ix];
        assert_eq!(check.op, x.op);
        assert_eq!(check.lh, x.lh);
        if let Some(operand) = &check.rh {
            assert_eq!(operand.typ, x.rh.typ);
            if let Some(n) = operand.value {
                assert_eq!(n, x.rh.value);
            }
            if let Some(r) = operand.reg {
                assert_eq!(r, x.rh.reg);
            }
        }
        match ix {
            4 => a = x.rh.value,
            5 => b = x.rh.value,
            9 => c = x.rh.value,
            15 => {
                result.push((a, b, c, x.rh.value));
            }
            _ => {}
        }
    }
    result
}

fn execute_optimized(abcd: &Abcd, w: isize, x: &mut isize, z: &mut isize) {
    let (a, b, c, d) = abcd;
    *x = (*z % 26) + b;
    *z /= a;
    *x = (*x != w) as isize;
    *z *= c * *x + 1;
    *z += (w + d) * *x
}

type Candidates = HashMap<(isize, isize), isize>;

fn main() {
    let input = include_str!("day24.txt");
    let code = input
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');
            let op = match tokens.next().unwrap() {
                "inp" => OpCode::Inp,
                "add" => OpCode::Add,
                "mul" => OpCode::Mul,
                "div" => OpCode::Div,
                "mod" => OpCode::Mod,
                "eql" => OpCode::Eql,
                _ => panic!("Unknown opcode"),
            };
            let lh = reg_to_ix(tokens.next().unwrap());
            let rh = if op != OpCode::Inp {
                get_operand(tokens.next().unwrap())
            } else {
                Operand {
                    typ: RHType::Num,
                    value: 0,
                    reg: 0,
                }
            };
            Command { op, lh, rh }
        })
        .collect::<Vec<_>>();
    let all_abcd = get_abcd(&code);
    let mut best: Candidates = Candidates::new();
    best.insert((0, 0), 0);
    for abcd in all_abcd {
        println!("{}", best.len());
        let mut next = Candidates::new();
        for ((x, z), prefix) in best.drain() {
            for w in 1..=9 {
                let number = prefix * 10 + w;
                let mut try_x = x;
                let mut try_z = z;
                execute_optimized(&abcd, w, &mut try_x, &mut try_z);
                next.entry((try_x, try_z))
                    .and_modify(|e| {
                        *e = min(*e, number);
                    })
                    .or_insert(number);
            }
        }
        best = next;
    }
    println!("{}", best.len());
    println!(
        "{}",
        best.iter()
            .filter(|((_, kz), _)| *kz == 0)
            .map(|(_, v)| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
