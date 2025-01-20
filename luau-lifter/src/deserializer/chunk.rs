use nom::{bytes::complete::take, IResult};

#[derive(Debug)]
pub struct Chunk {
    pub content: String,
}

impl Chunk {
    pub fn parse(input: &[u8], encode_key: u8, _status_code: u8) -> IResult<&[u8], Chunk> {
        let (input, data) = take(input.len())(input)?;

        // Replace placeholder variables for readability
        let mut variable_count = 1;
        let mut param_count = 1;

        let content = String::from_utf8_lossy(data)
            .lines()
            .map(|line| {
                let mut modified_line = line.to_string();

                // Replace "v_u_" variables with short names
                while modified_line.contains("v_u_") {
                    modified_line = modified_line.replacen("v_u_", &format!("var{}", variable_count), 1);
                    variable_count += 1;
                }

                // Replace "p_u_" parameters with short names
                while modified_line.contains("p_u_") {
                    modified_line = modified_line.replacen("p_u_", &format!("param{}", param_count), 1);
                    param_count += 1;
                }

                modified_line
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok((
            input,
            Chunk {
                content,
            },
        ))
    }
}
