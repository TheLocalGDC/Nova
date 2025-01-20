use nom::{bytes::complete::take, number::complete::{le_u8, le_u32}, IResult};

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub data: Vec<u8>,
}

impl Constant {
    pub fn parse_constant(input: &[u8], sequence: usize) -> IResult<&[u8], Constant> {
        // Attempt to extract the length of the constant data
        let (input, length) = le_u32(input).map_err(|e| {
            eprintln!("Failed to parse constant length at sequence {}: {:?}", sequence, e);
            e
        })?;

        // Validate the length does not exceed the remaining input size
        if input.len() < length as usize {
            eprintln!(
                "Invalid constant length: {} exceeds remaining input size: {}",
                length, input.len()
            );
            return Err(nom::Err::Error((input, nom::error::ErrorKind::LengthValue)));
        }

        // Attempt to extract the actual constant data
        let (input, data) = take(length as usize)(input).map_err(|e| {
            eprintln!(
                "Failed to extract constant data for const_{} (length: {}): {:?}",
                sequence, length, e
            );
            e
        })?;

        // Assign a name to the constant based on its sequence number
        let constant_name = format!("const_{}", sequence);

        println!("Successfully parsed constant: {}, Data: {:?}", constant_name, data);

        Ok((
            input,
            Constant {
                name: constant_name,
                data: data.to_vec(),
            },
        ))
    }
}
