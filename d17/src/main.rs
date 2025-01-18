use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitXor;
use std::path::Path;

use utils::get_input_path;

#[derive(Debug)]
enum OpCodes {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Clone, Default)]
struct VM {
    pos: u64,
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    output: Vec<u64>,
}

impl VM {
    pub(crate) fn run(&mut self) {
        while self.can_continue() {
            self.operate();
        }
    }

    pub(crate) fn operate(&mut self) {
        let opcode: OpCodes = self.program[self.pos as usize].into();
        match opcode {
            OpCodes::Adv => self.adv(),
            OpCodes::Bxl => self.bxl(),
            OpCodes::Bst => self.bst(),
            OpCodes::Jnz => self.jnz(),
            OpCodes::Bxc => self.bxc(),
            OpCodes::Out => self.out(),
            OpCodes::Bdv => self.bdv(),
            OpCodes::Cdv => self.cdv(),
        }
    }
    // pub(crate) fn is_halt(&self) -> bool {
    //     self.pos >= self.program.len() as u64
    // }
    pub(crate) fn can_continue(&self) -> bool {
        self.pos < self.program.len() as u64
    }
    pub(crate) fn bdv(&mut self) {
        let operand = self.get_combo();
        let result = self.a / 2_u64.pow(operand as u32);
        self.b = result;
        self.pos += 2;
    }
    pub(crate) fn cdv(&mut self) {
        let operand = self.get_combo();
        let result = self.a / 2_u64.pow(operand as u32);
        self.c = result;
        self.pos += 2;
    }
    pub(crate) fn out(&mut self) {
        let combo = self.get_combo();
        let result = combo % 8;
        self.output.push(result);

        self.pos += 2;
    }
    pub(crate) fn bxc(&mut self) {
        self.b = self.b.bitxor(self.c);

        self.pos += 2;
    }
    pub(crate) fn jnz(&mut self) {
        if self.a == 0 {
            self.pos += 2;
            return;
        }

        self.pos = self.get_literal();
    }
    pub(crate) fn bst(&mut self) {
        let combo = self.get_combo();
        self.b = combo % 8;

        self.pos += 2;
    }
    pub(crate) fn bxl(&mut self) {
        let result = self.b.bitxor(self.get_literal());
        self.b = result;

        self.pos += 2;
    }
    pub(crate) fn adv(&mut self) {
        let operand = self.get_combo();
        let result = self.a / 2_u64.pow(operand as u32);
        self.a = result;

        self.pos += 2;
    }
    pub(crate) fn get_literal(&self) -> u64 {
        self.program[self.pos as usize + 1]
    }
    pub(crate) fn get_combo(&self) -> u64 {
        match self.program[self.pos as usize + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("This is not a valid combo value. The value is 7"),
            _ => panic!(
                "This is not a valid combo value. The value is {}",
                &self.program[self.pos as usize]
            ),
        }
    }
    pub(crate) fn create_result(&self) -> String {
        let mut rtn = String::with_capacity(self.output.len() * 2);

        let mut is_not_first = false;

        for i in &self.output {
            if is_not_first {
                rtn.push(',');
            }
            is_not_first = true;

            let s = i % 8;
            rtn.push_str(s.to_string().as_str());
        }

        rtn
    }
}

impl From<u64> for OpCodes {
    fn from(value: u64) -> Self {
        match value {
            0 => OpCodes::Adv,
            1 => OpCodes::Bxl,
            2 => OpCodes::Bst,
            3 => OpCodes::Jnz,
            4 => OpCodes::Bxc,
            5 => OpCodes::Out,
            6 => OpCodes::Bdv,
            7 => OpCodes::Cdv,
            _ => panic!("Should not be here!"),
        }
    }
}

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut vm = VM {
        pos: 0,
        a: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        b: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        c: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        program: vec![],
        output: vec![],
    };

    let _ = lines.next().unwrap();

    lines.next().unwrap().unwrap()[9..]
        .split(",")
        .map(|i| i.parse::<u64>().unwrap())
        .for_each(|i| vm.program.push(i));

    vm.run();

    // Result
    println!("Result of part 1 is {}", vm.create_result());
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut vm = VM {
        pos: 0,
        a: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        b: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        c: lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        program: vec![],
        output: vec![],
    };

    let _ = lines.next().unwrap();

    lines.next().unwrap().unwrap()[9..]
        .split(",")
        .map(|i| i.parse::<u64>().unwrap())
        .for_each(|i| vm.program.push(i));

    // Solve
    let mut a = 0;
    for i in (0..vm.program.len()).rev() {
        a <<= 3;
        loop {
            let mut current_vm = vm.clone();
            current_vm.a = a;
            current_vm.run();
            if current_vm.output == vm.program[i..] {
                break;
            }
            a += 1;
        }
    }

    // Result
    println!("Result of part 2 is {}", a);
}

fn main() {
    let input_file = get_input_path(env!("CARGO_MANIFEST_DIR"));

    println!("Running {}", env!("CARGO_PKG_NAME"));
    println!("InputFile: {}", input_file.display());

    run(input_file.as_path());
    run2(input_file.as_path());
}

#[cfg(test)]
mod main_test {
    use utils::{get_test_input_2_path, get_test_input_path};

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        run(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn test_input_part_2() {
        run2(&get_test_input_2_path(env!("CARGO_MANIFEST_DIR")));
    }
}
