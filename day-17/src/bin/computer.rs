#[derive(Debug, PartialEq)]
enum OpCode {
    DivA = 0,
    XorB = 1,
    ModB = 2,
    JumpIfNotZero = 3,
    XorBWithC = 4,
    Output = 5,
    DivB = 6,
    DivC = 7,
    Unknown,
}

impl From<u64> for OpCode {
    fn from(value: u64) -> Self {
        match value {
            0 => OpCode::DivA,
            1 => OpCode::XorB,
            2 => OpCode::ModB,
            3 => OpCode::JumpIfNotZero,
            4 => OpCode::XorBWithC,
            5 => OpCode::Output,
            6 => OpCode::DivB,
            7 => OpCode::DivC,
            _ => OpCode::Unknown,
        }
    }
}

pub struct Computer<'a> {
    pub program: &'a [u64],
    pub ip: usize,
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub outputs: Vec<u64>,
}

impl<'a> Computer<'a> {
    pub fn new(program: &'a [u64], a: Option<u64>, b: Option<u64>, c: Option<u64>) -> Self {
        Self {
            program,
            ip: 0,
            a: a.unwrap_or(0),
            b: b.unwrap_or(0),
            c: c.unwrap_or(0),
            outputs: Vec::new(),
        }
    }

    pub fn run_once(&mut self) -> u64 {
        while self.outputs.is_empty() {
            let opcode = OpCode::from(self.program[self.ip]);
            let operand = self.program.get(self.ip + 1).copied().unwrap_or(0);
            self.execute(opcode, operand);
        }
        if let Some(&first) = self.outputs.first() {
            return first;
        }
        unreachable!();
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            let opcode = OpCode::from(self.program[self.ip]);
            let operand = self.program.get(self.ip + 1).copied().unwrap_or(0);
            self.execute(opcode, operand);
        }
    }

    fn execute(&mut self, opcode: OpCode, operand: u64) {
        use OpCode::*;
        match opcode {
            DivA => self.div_a(operand),
            XorB => self.xor_b(operand),
            ModB => self.mod_b(operand),
            JumpIfNotZero => self.jump_if_not_zero(operand),
            XorBWithC => self.xor_b_with_c(),
            Output => self.output(operand),
            DivB => self.div_b(operand),
            DivC => self.div_c(operand),
            Unknown => self.handle_unknown(),
        }
    }

    fn div_a(&mut self, operand: u64) {
        self.a /= 2u64.pow(self.resolve_operand(operand, true) as u32);
        self.advance_ip();
    }

    fn xor_b(&mut self, operand: u64) {
        self.b ^= self.resolve_operand(operand, false);
        self.advance_ip();
    }

    fn mod_b(&mut self, operand: u64) {
        self.b = self.resolve_operand(operand, true) % 8;
        self.advance_ip();
    }

    fn jump_if_not_zero(&mut self, operand: u64) {
        if self.a != 0 {
            self.ip = self.resolve_operand(operand, false) as usize;
        } else {
            self.advance_ip();
        }
    }

    fn xor_b_with_c(&mut self) {
        self.b ^= self.c;
        self.advance_ip();
    }

    fn output(&mut self, operand: u64) {
        self.outputs.push(self.resolve_operand(operand, true) % 8);
        self.advance_ip();
    }

    fn div_b(&mut self, operand: u64) {
        self.b = self.a / 2u64.pow(self.resolve_operand(operand, true) as u32);
        self.advance_ip();
    }

    fn div_c(&mut self, operand: u64) {
        self.c = self.a / 2u64.pow(self.resolve_operand(operand, true) as u32);
        self.advance_ip();
    }

    fn handle_unknown(&self) {
        println!("Unknown opcode encountered at position {}", self.ip);
    }

    fn resolve_operand(&self, operand: u64, is_combo: bool) -> u64 {
        if !is_combo {
            operand
        } else {
            match operand {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => operand,
            }
        }
    }

    fn advance_ip(&mut self) {
        self.ip += 2;
    }
}
