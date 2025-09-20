// an_enigma_of_a_site/engima - A German WW2 Enigma machine.
// Copyright (C) 2025 Robert R. Russell
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3.
//
// This program is distributed in the hope that it will be usefull, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

use crate::characters::Characters;

use std::collections::HashMap;
use std::collections::HashSet;
use std::result::Result;

/// The "Steckerboard" or "Plugboard" on the front of the Enigma machine.
pub struct SteckerBoard {
    board: HashMap<Characters, Characters>,
}

impl SteckerBoard {
    pub fn new() -> Self {
        let mut board: HashMap<Characters, Characters> =
            HashMap::with_capacity(26);
        for c in Characters::A.iter() {
            board.insert(c, c);
        }
        Self{ board }
    }

    pub fn add_jumper_wires(&mut self, pairs: Pairs) {
        for pair in pairs.data {
            self.board.insert(pair.from, pair.to);
            self.board.insert(pair.to, pair.from);
        }
    }

    pub fn encipher(&self, input: Characters) -> Characters {
        *(self.board.get(&input).unwrap())
    }
}

/// A single swapping to add to the "Steckerboard".
/// You cannot swap from and to yourself.
pub(crate) struct Pairing {
    pub(crate) from: Characters,
    pub(crate) to: Characters,
}

/// The swappings that we want to plug into the Steckerboard.
pub struct Pairs {
    pub(crate) data: [Pairing; 10],
}

impl Pairs {
    // This is necessary because the compiler is being dumb.
    #[allow(unused_assignments)]
    pub fn try_new(input: &str) -> Result<Pairs, &'static str> {
        if input.len() > 29 {
            return Err("You only have ten wires for the Steckerboard.");
        }
        let mut char_set: HashSet<Characters> = HashSet::with_capacity(20);
        let mut pairings: usize = 0;
        let mut data: [Pairing; 10] = [
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
            Pairing{from: Characters::A, to: Characters::A},
        ];

        let mut temp_from: Option<Characters> = None;
        let mut temp_to: Option<Characters> = None;

        for c in input.chars() {
            let temp: Option<Characters>;
            match c {
                'A' | 'a' => {temp = Some(Characters::A);}
                'B' | 'b' => {temp = Some(Characters::B);}
                'C' | 'c' => {temp = Some(Characters::C);}
                'D' | 'd' => {temp = Some(Characters::D);}
                'E' | 'e' => {temp = Some(Characters::E);}
                'F' | 'f' => {temp=Some(Characters::F);}
                'G' | 'g' => {temp=Some(Characters::G);}
                'H' | 'h' => {temp=Some(Characters::H);}
                'I' | 'i' => {temp=Some(Characters::I);}
                'J' | 'j' => {temp=Some(Characters::J);}
                'K' | 'k' => {temp=Some(Characters::K);}
                'L' | 'l' => {temp=Some(Characters::L);}
                'M' | 'm' => {temp=Some(Characters::M);}
                'N' | 'n' => {temp=Some(Characters::N);}
                'O' | 'o' => {temp=Some(Characters::O);}
                'P' | 'p' => {temp=Some(Characters::P);}
                'Q' | 'q' => {temp=Some(Characters::Q);}
                'R' | 'r' => {temp=Some(Characters::R);}
                'S' | 's' => {temp=Some(Characters::S);}
                'T' | 't' => {temp=Some(Characters::T);}
                'U' | 'u' => {temp=Some(Characters::U);}
                'V' | 'v' => {temp=Some(Characters::V);}
                'W' | 'w' => {temp=Some(Characters::W);}
                'X' | 'x' => {temp=Some(Characters::X);}
                'Y' | 'y' => {temp=Some(Characters::Y);}
                'Z' | 'z' => {temp=Some(Characters::Z);}
                ':' => {
                    temp = None;
                    if temp_to.is_none() || temp_from.is_none() {
                        return Err("There were not exactly two characters \
between the last seen ':' and this one.");
                    }
                    if pairings > 9 {
                        return Err("There are more than ten pairings inside of \
the list of pairings.");
                    }
                    data[pairings] = Pairing{from: temp_from.unwrap(),
                        to: temp_to.unwrap()};
                    pairings += 1;
                    temp_from = None;
                    temp_to = None;
                }
                _ => {
                    temp = None;
                    return Err("Invalid character found in a Steckerboard \
pairing.");
                }
            }

            if temp.is_some() {
                if char_set.insert(temp.unwrap()) {
                    return Err("We cannot reuse a letter that we have already \
used.");
                } else {
                    if temp_from.is_none() {
                        temp_from = temp;
                        break;
                    }
                    if temp_to.is_none() {
                        temp_to = temp;
                        break;
                    }
                    return Err("There were not exactly two characters between \
the last seen ':' and this one.");
                }
            }
        }

        if pairings < 9 {
            return Err("There are 10 wires for the Steckerboard. You must use \
all of them.");
        }

        return Ok(Pairs{data});
    }
}

