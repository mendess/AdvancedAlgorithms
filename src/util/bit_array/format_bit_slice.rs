use super::WORD_SIZE;
use itertools::Itertools;
use std::fmt;

pub fn format_slice(
    slice: &[u8],
    r_size: usize,
    count: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let slice_len = ((r_size * count) + (WORD_SIZE - 1)) / WORD_SIZE;
    writeln!(
        f,
        "[{}]",
        slice[..slice_len]
            .iter()
            .format_with("|", |b, f| f(&format_args!("{:08b}", b)))
    )?;
    let mut line = 0;
    let mut byte_bound = 8;
    write!(f, " ")?;
    let mut wchar = |line: usize, c: char| -> Result<usize, fmt::Error> {
        if line == byte_bound {
            write!(f, " ")?;
            byte_bound += 8;
        }
        write!(f, "{}", c)?;
        Ok(line + 1)
    };
    for _ in 0..count {
        line = wchar(line, '^')?;
        if r_size > 1 {
            for _ in 0..r_size - 2 {
                line = wchar(line, '-')?;
            }
            line = wchar(line, '^')?;
        }
    }
    writeln!(f)?;
    write!(f, " ")?;
    let mut span = 0;
    for i in 0..count {
        span += r_size;
        let w = if span >= 8 {
            span %= 8;
            r_size + 1
        } else {
            r_size
        };
        write!(f, "{:<width$}", i, width = w)?;
    }
    Ok(())
}
