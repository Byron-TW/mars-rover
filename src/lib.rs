#[macro_use]
extern crate failure;

use failure::{err_msg, Error, ResultExt};
use std::{fmt, io::{self, BufRead, BufReader, Read}, str::FromStr};

pub fn answers(input: impl Read, mut output: impl io::Write) -> Result<(), Error> {
    let input = BufReader::new(input);
    let mut lines = input.lines();

    let first_line = lines
        .next()
        .ok_or_else(|| err_msg("Empty input"))?
        .context("Failed to obtain first line for dimensions")?;

    let (_max_x, _max_y) = parse_dimensions(first_line)?;
    loop {
        match (lines.next(), lines.next()) {
            (Some(l), Some(m)) => {
                use self::Orientation::*;
                let location = l.with_context(|_err| "Could not read line with rover location")?;
                let movement = m.with_context(|_err| "Could not read line with rover movement")?;
                let (mut lx, mut ly, mut ld) = parse_location(location)?;

                for action in movement.chars() {
                    match action {
                        'M' | 'm' => match ld {
                            North => ly += 1,
                            South => ly -= 1,
                            East => lx += 1,
                            West => lx -= 1,
                        },
                        'L' | 'l' => ld = ld.turn_left(),
                        'R' | 'r' => ld = ld.turn_right(),
                        _ => bail!("Invalid action: '{}'", action),
                    }
                }
                writeln!(output, "{} {} {}", lx, ly, ld)?;
            }
            (None, None) => return Ok(()),
            _ => bail!("Didn't obtain two lines for (1) mars rover position and (2) its movement"),
        }
    }
}

enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn turn_right(&self) -> Orientation {
        use self::Orientation::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn turn_left(&self) -> Orientation {
        use self::Orientation::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Orientation::*;
        use std::fmt::Write;
        match self {
            North => f.write_char('N'),
            East => f.write_char('E'),
            South => f.write_char('S'),
            West => f.write_char('W'),
        }
    }
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Orientation, Error> {
        use self::Orientation::*;
        Ok(match s {
            "N" | "n" => North,
            "E" | "e" => East,
            "S" | "s" => South,
            "W" | "w" => West,
            _ => bail!("'{}' is not a valid orientation", s),
        })
    }
}

fn parse_location(line: String) -> Result<(u32, u32, Orientation), Error> {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [x, y, d] => Ok((parse_u32(x)?, parse_u32(y)?, d.parse()?)),
        _ => Err(format_err!(
            "Didn't find x and y coordinate and direction in input '{}'",
            line
        )),
    }
}

fn parse_dimensions(line: String) -> Result<(u32, u32), Error> {
    let dimensions: Vec<u32> = line.split_whitespace()
        .map(|t| parse_u32(t))
        .collect::<Result<_, _>>()?;

    match dimensions.as_slice() {
        &[x, y] => Ok((x, y)),
        _ => Err(format_err!(
            "Input '{}' didn't contain exactly two numbers",
            line
        )),
    }
}

fn parse_u32(t: &str) -> Result<u32, Error> {
    t.parse::<u32>()
        .with_context(|_err| format!("Could not parse '{}' into unsigned 32bit integer", t))
        .map_err(Into::into)
}
