use super::common::CodeBlock;
use super::util;
use crate::error::Result;
use crate::varint::decode_u32;

pub fn decode_code_section<R: crate::Reader>(reader: &mut R) -> Result<CodeBlock> {
    let body_size = decode_u32(reader)?.0;

    let v = util::decode_vec(reader, |x| {
        Ok((decode_u32(x)?, util::decode_kind(x)?))
    })?;

    let mut locals = Vec::new();
    let mut read = 0;
    for ((count, consumed), kind) in v {
        for _ in 0..count {
            locals.push(kind);
        }
        read += consumed as u32;
    }

    let instruction_size = body_size - read;

    let mut instructions = Vec::with_capacity(instruction_size as usize);

    reader.read_exact(&mut instructions)?;

    Ok(CodeBlock {
        locals: locals.into_boxed_slice(),
        instructions,
    })
}
