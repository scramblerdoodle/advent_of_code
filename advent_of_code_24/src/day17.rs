use crate::utils::{get_test_file, FileNotFound, ACTUAL, EXAMPLE};
use std::fs::read_to_string;

/*
* 0123
* means Operation 0 applied to Operand 1
* Operation 2 applied to Operand 3
*
* COMBO Operand (cop) Translation
* 0 => 0
* 1 => 1
* 2 => 2
* 3 => 3
* 4 => Register A
* 5 => Register B
* 6 => Register C
* 7 => Panic
*
* Operation Translation
* 0 => `adv`, Reg A <- floor(division of Reg A / 2^cop)
* 1 => `bxl`, Reg B <- XOR of Reg B and lop
* 2 => `bst`, Reg B <- cop mod 8 (three last bits)
* 3 => `jnz`, if Reg A == 0,
*               nothing
*             else jump to operation pointer "lop"
*                  no +2 to instruction pointer
* 4 => `bxc`, Reg B <- XOR of Reg B and Reg C
* 5 => `out`, print(cop mod 8), each output comma-separated
* 6 => `bdv`, exactly like `adv`(0) but store to Reg B
* 7 => `cdv`, exactly like `adv`(0) but store to Reg C
*
*/

struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    instructions: Vec<u8>,
    pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn new(reg_a: u32, reg_b: u32, reg_c: u32, instructions: Vec<u8>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            instructions,
            pointer:0,
            output:vec![],
        }
    }

    fn print_state(&self) {
        println!("Reg A: {}", self.reg_a);
        println!("Reg B: {}", self.reg_b);
        println!("Reg C: {}", self.reg_c);
        println!("opcode: {}", self.get_instruction_at_pointer());
        println!("operand: {}", self.instructions[self.pointer+1]);
    }

    fn translate_combo_operand(&self, n: u8) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("7 is not a valid COp"),
            _ => panic!("Unknown instruction"),
        }
    }

    fn get_instruction(&mut self) -> u8 {
        let instr = self.get_instruction_at_pointer();
        self.pointer += 1;

        instr
    }

    fn get_instruction_at_pointer(&self) -> u8 {
        self.instructions[self.pointer]
    }

    fn div_pow_2(&self) -> u32 {
        let cop = self.translate_combo_operand(self.get_instruction_at_pointer());
        self.reg_a / 2_u32.pow(cop)
    }

    fn adv(&mut self) {
        self.reg_a = self.div_pow_2();
        self.pointer+=1;
    }

    fn bxl(&mut self) {
        let lop = self.get_instruction_at_pointer();
        self.reg_b ^= lop as u32;
        self.pointer+=1;
    }

    fn bst(&mut self) {
        let cop = self.translate_combo_operand(self.get_instruction_at_pointer());
        self.reg_b = cop % 8;
        self.pointer+=1;
    }

    fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.pointer+=1;
        }
        else {
            let lop = self.get_instruction_at_pointer();
            self.pointer = lop as usize;
        }
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.pointer+=1;
    }

    fn out(&mut self) {
        let cop = self.translate_combo_operand(self.get_instruction_at_pointer());
        self.output.push((cop % 8) as u8);
        self.pointer+=1;
    }

    fn bdv(&mut self) {
        self.reg_b = self.div_pow_2();
        self.pointer+=1;
    }

    fn cdv(&mut self) {
        self.reg_c = self.div_pow_2();
        self.pointer+=1;
    }

    fn process(&mut self) {
        println!("{:?}", self.instructions);

        while self.pointer < self.instructions.len() {
            self.print_state();
            println!();
            match self.get_instruction() {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => panic!("Unknown instruction"),
            }
        }
    }

    pub fn get_output(&self) -> String {
        self.output.iter().map(|n| format!("{n}")).collect::<Vec<String>>().join(",")
    }
}

fn day17(mut program: Computer) -> String {
    program.process();
    program.get_output()
}

fn day17_v2(mut program: Computer) -> String {
    program.process();
    program.get_output()
}

fn parse_input(filepath: &str) -> Computer {
    let mut result: Vec<String> = vec![];

    read_to_string(filepath).unwrap().lines().for_each(|l| {
        result.push(l.to_string());
    });

    let reg_a = result[0].split(" ").last().unwrap().parse::<u32>().expect("Unexpected input");
    let reg_b = result[1].split(" ").last().unwrap().parse::<u32>().expect("Unexpected input");
    let reg_c = result[2].split(" ").last().unwrap().parse::<u32>().expect("Unexpected input");

    let program = result[4].split(" ").last().unwrap().split(",").map(|n| n.parse::<u8>().expect("Unexpected input")).collect();

    Computer::new(reg_a, reg_b, reg_c, program)
}

pub fn main(s: &str) -> Result<String, FileNotFound> {
    match s {
        "example" => match get_test_file(EXAMPLE, "17") {
            Err(err) => Err(err),
            Ok(file) => Ok(day17(parse_input(&file))),
        },
        "actual" => match get_test_file(ACTUAL, "17") {
            Err(err) => Err(err),
            Ok(file) => Ok(day17(parse_input(&file))),
        },
        "example_v2" => match get_test_file(EXAMPLE, "17") {
            Err(err) => Err(err),
            Ok(file) => Ok(day17_v2(parse_input(&file))),
        },
        "actual_v2" => match get_test_file(ACTUAL, "17") {
            Err(err) => Err(err),
            Ok(file) => Ok(day17_v2(parse_input(&file))),
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(main("example").unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(main("example_v2").unwrap(), "");
    }
}

/*
Example:

init:
  A: 729
  B: 0
  C: 0

after 01:
  A: floor(729/2^1) = 364
  B: 0
  C: 0

after 54:
  print (364 mod 8 = 4)
  print 4

after 30:
  jump instruction 0

01:
  A: floor(364/2) = 182
  B: 0
  C: 0

54:
  print (182 mod 8 = 6)
  print 6

30:
  goto 0

01:
  A: floor(182/2) = 91

54:
  print 3 (91 mod 8)

01:
  A: floor(91/2) = 45

54:
  print 5 (45 mod 8)

01: A: 45/2 = 22
54: print 6

01: A: 11
54: print 3

01: A: 5
54: print 5

01: A: 2
54: print 2

01: A: 1
54: print 1

01: A: 0
54: print 0
30: stop

output: 4,6,3,5,6,3,5,2,1,0
*/
