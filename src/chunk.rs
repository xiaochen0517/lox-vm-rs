use crate::value::ValueVec;

pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueVec,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueVec::new(),
        }
    }

    pub fn push_code(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn push_constant(&mut self, value: f64) -> usize {
        self.constants.values.push(value);
        self.constants.values.len() - 1
    }

    pub fn disassemble(&self) -> String {
        let mut output = String::new();
        for (i, op) in self.code.iter().enumerate() {
            let code_str = match op {
                OpCode::OpConstant(index) => {
                    format!("{:04} OpConstant {:?}", i, &self.constants.get(*index))
                }
                OpCode::OpReturn => format!("{:04} OpReturn", i),
            };
            output.push_str(&code_str);
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_push_and_disassemble() {
        let mut chunk = Chunk::new();
        chunk.push_code(OpCode::OpReturn);
        chunk.push_code(OpCode::OpReturn);
        let const_index = chunk.push_constant(24.0);
        chunk.push_code(OpCode::OpConstant(const_index));
        let output_str = chunk.disassemble();
        let expected_output = "0000 OpReturn\n0001 OpReturn\n0002 OpConstant Some(24.0)\n";
        assert_eq!(output_str, expected_output);
    }
}
