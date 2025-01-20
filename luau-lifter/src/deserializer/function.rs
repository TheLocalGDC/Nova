use nom::{
    bytes::complete::take,
    number::complete::{le_u32, le_u8},
    IResult,
};
use nom_leb128::leb128_usize;

#[derive(Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub upvalues: Vec<String>,
    pub body: Vec<String>,
}

impl Function {
    pub fn parse_function(input: &[u8]) -> IResult<&[u8], Function> {
        println!("Parsing function from input: {:?}", input);

        let mut param_count = 1;
        let mut upvalue_count = 1;
        let mut body_count = 1;

        // Parse function parameters
        let (input, param_length) = le_u8(input).map_err(|e| {
            eprintln!("Failed to parse parameter length: {:?}", e);
            e
        })?;
        println!("Parameter length: {}", param_length);

        let (input, params) = take(param_length as usize)(input).map(|(remaining, data)| {
            let params = (0..param_length)
                .map(|_| {
                    let name = format!("param_{}", param_count);
                    param_count += 1;
                    name
                })
                .collect::<Vec<_>>();
            (remaining, params)
        }).map_err(|e| {
            eprintln!("Failed to parse function parameters: {:?}", e);
            e
        })?;

        // Parse upvalues
        let (input, upvalue_length) = le_u8(input).map_err(|e| {
            eprintln!("Failed to parse upvalue length: {:?}", e);
            e
        })?;
        println!("Upvalue length: {}", upvalue_length);

        let (input, upvalues) = take(upvalue_length as usize)(input).map(|(remaining, _)| {
            (0..upvalue_length)
                .map(|_| {
                    let name = format!("upvalue_{}", upvalue_count);
                    upvalue_count += 1;
                    name
                })
                .collect::<Vec<_>>()
        }).map_err(|e| {
            eprintln!("Failed to parse function upvalues: {:?}", e);
            e
        })?;

        // Parse function body (placeholder logic for now)
        let (input, body) = take(10usize)(input).map(|(remaining, _)| {
            (0..body_count)
                .map(|_| {
                    let name = format!("body_{}", body_count);
                    body_count += 1;
                    name
                })
                .collect::<Vec<_>>()
        }).map_err(|e| {
            eprintln!("Failed to parse function body: {:?}", e);
            e
        })?;

        println!("Parsed function successfully: params={:?}, upvalues={:?}, body={:?}", params, upvalues, body);

        Ok((
            input,
            Function {
                params,
                upvalues,
                body,
            },
        ))
    }
}
