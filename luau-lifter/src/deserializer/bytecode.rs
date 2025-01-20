use nom::{bytes::complete::take, number::complete::{le_u8, le_u16, le_u32}, IResult};

use super::chunk::Chunk;
use super::functions::Function;

#[derive(Debug)]
pub enum Bytecode {
    Error(String),
    Chunk(Chunk),
}

impl Bytecode {
    pub fn parse(input: &[u8], encode_key: u8) -> IResult<&[u8], Bytecode> {
        let (input, status_code) = le_u8(input)?;
        match status_code {
            0 => {
                let (input, error_msg) = take(input.len())(input)?;
                Ok((
                    input,
                    Bytecode::Error(String::from_utf8_lossy(error_msg).to_string()),
                ))
            }
            0x54 => { // Lua 5.4 specific magic byte
                let (input, chunk) = Chunk::parse(input, encode_key, status_code)?;

                // Track variable replacements with counters
                let mut object_var_count = 1;
                let mut param_var_count = 1;
                let mut local_var_count = 1;

                // Replace variables for better readability
                let modified_chunk = chunk
                    .lines()
                    .map(|line| {
                        let mut line = line;

                        // Replace all "v_u_" variables with short names like "oX"
                        while line.contains("v_u_") {
                            line = line.replacen("v_u_", &format!("o{}", object_var_count), 1);
                            object_var_count += 1;
                        }

                        // Replace function parameters "p_u_" with short names like "pX"
                        while line.contains("p_u_") {
                            line = line.replacen("p_u_", &format!("p{}", param_var_count), 1);
                            param_var_count += 1;
                        }

                        // Replace standalone "vX" variables with short names like "lX"
                        while let Some(start) = line.find("v") {
                            if start + 1 < line.len() && line[start + 1..].chars().next().unwrap().is_numeric() {
                                line = line.replacen("v", &format!("l{}", local_var_count), 1);
                                local_var_count += 1;
                            } else {
                                break;
                            }
                        }

                        // Remove or refine upvalue comments
                        if line.contains("-- upvalues:") {
                            line = line.replace("-- upvalues:", "-- Locals used:");
                        }

                        line
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                // Add the header explicitly
                let final_chunk = format!("-- Decompiled by Nova Decompiler\n\n{}", modified_chunk);

                // Sanitize the chunk for JSON compatibility
                let sanitized_chunk = final_chunk.replace("\0", "\\0");
                Ok((input, Bytecode::Chunk(sanitized_chunk)))
            }
            _ => panic!("Unsupported bytecode version: {}", status_code),
        }
    }
}
