use super::{Plank, BinaryInsert};

use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Board {
    logs: [[bool; 5]; 7],
    planks: Vec<Plank>,
}

impl Board {
    pub const fn new() -> Self {
        Board {
            logs: [[false; 5]; 7],
            planks: Vec::new(),
        }
    }
    pub fn add_log(&mut self, x: u8, y: u8) {
        self.logs[y as usize][x as usize] = true;
    }
    pub fn set_plank(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        if self.logs[y1 as usize][x1 as usize] && self.logs[y2 as usize][x2 as usize] {
            let dx = x2.sub_abs(x1);
            let dy = y2.sub_abs(y1);

            let pointing_down = dx == 0;
            let pointing_right = dy == 0;

            let (x, y, length) = if pointing_down && pointing_right {
                return false;
            } else if pointing_down && 1 <= dy && dy <= 3 {
                (x1, y1.min(y2), dy)
            } else if pointing_right && 1 <= dx && dx <= 3 {
                (x1.min(x2), y1, dx)
            } else {
                return false;
            };

            let plank = Plank {
                x,
                y,
                pointing_down,
                length
            };

            self.planks.binary_insert(plank);

            true
        } else {
            false
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut planks = self.planks.iter().peekable();
        // [(x, remaining_length)]
        let mut vertical_plank_lengths = [0; 5];

        let ly = self.logs.len() - 1;
        let lx = self.logs[0].len() - 1;

        writeln!(f, "┌─────────┐")?;
        for (y, ls) in self.logs.iter().enumerate() {
            let mut has_plank = [false; 5];

            while planks.peek().map(|i| i.y == y as u8).unwrap_or(false) {
                let &Plank { x, pointing_down, length, y: _ } = planks.next().unwrap();
                if pointing_down {
                    vertical_plank_lengths[x as usize] += length;
                } else {
                    has_plank[x as usize..(x+length) as usize].iter_mut().for_each(|p| *p = true);
                }
            }

            write!(f, "│")?;
            for (x, &has_log) in ls.iter().enumerate() {
                let has_plank = has_plank[x];
                let first_char = if has_log {
                    "#"
                } else if vertical_plank_lengths[x] > 0 {
                    "║"
                } else if has_plank {
                    "═"
                } else {
                    " "
                };
                let second_char = if x == lx {
                    ""
                } else if has_plank {
                    "═"
                } else {
                    " "
                };
                write!(f, "{}{}", first_char, second_char)?;
            }
            writeln!(f, "│")?;
            if y != ly {
                write!(f, "│")?;
                for (x, length) in vertical_plank_lengths.iter_mut().enumerate() {
                    let plank_s = if *length > 0 {
                        *length -= 1;
                        "║"
                    } else {
                        " "
                    };
                    write!(f, "{}{}", plank_s, if x != lx { " " } else { "" })?;
                }
                writeln!(f, "│")?;
            }
        }
        write!(f, "└─────────┘")?;
        Ok(())
    }
}