// an_enigma_of_a_site/enigma/src/reflectors.rs - A German WW2 Enigma machine.
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
use crate::static_data::REFLECTOR_WIDEB;
use crate::static_data::REFLECTOR_WIDEC;
use crate::wiring::Wiring;

pub struct Reflector {
    wiring: Wiring,
}

impl Reflector {
    pub fn new(reflector: Reflectors) -> Self {
        match reflector {
            Reflectors::WideB => {
                return Reflector {
                    wiring: Wiring::new(REFLECTOR_WIDEB),
                };
            }
            Reflectors::WideC => {
                return Reflector {
                    wiring: Wiring::new(REFLECTOR_WIDEC),
                };
            }
        }
    }

    pub fn encipher(&self, input: Characters) -> Characters {
        self.wiring[input]
    }
}

pub enum Reflectors {
    WideB,
    WideC,
}
