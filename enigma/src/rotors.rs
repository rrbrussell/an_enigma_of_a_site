// an_enigma_of_a_site/enigma - A German WW2 Enigma machine.
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
use crate::static_data;
use crate::wiring::Wiring;

/// This is the list of available Rotors.
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Rotors {
    I,
    II,
    III,
    IV,
    V,
}

pub struct Rotor {
    forward_wiring_map: Wiring,
    backward_wiring_map: Wiring,
    transfer_position: Characters,
    offset: Characters,
    indicator: Characters,
}

impl Rotor {
    /// Create a new Rotor.
    /// You can pick a rotor from the Rotors enumeration.
    /// The offset sets which positon A on the Rotor's internal ring maps to
    /// on the Rotor's indicators.
    pub fn new(which: Rotors, indicator: Characters, offset: Characters) -> Rotor {
        let forward_wiring_map: Wiring;
        let backward_wiring_map: Wiring;
        let transfer_position: Characters;

        match which {
            Rotors::I => {
                transfer_position = static_data::ROTOR_I_TRANSFER_POSITION;
                forward_wiring_map = Wiring::new(static_data::ROTOR_I_FORWARD);
                backward_wiring_map = Wiring::new(static_data::ROTOR_I_BACKWARD);
            },
            Rotors::II => {
                transfer_position = static_data::ROTOR_II_TRANSFER_POSITION;
                forward_wiring_map = Wiring::new(static_data::ROTOR_II_FORWARD);
                backward_wiring_map = Wiring::new(static_data::ROTOR_II_BACKWARD);
            },
            Rotors::III => {
                transfer_position = static_data::ROTOR_I_TRANSFER_POSITION;
                forward_wiring_map = Wiring::new(static_data::ROTOR_III_FORWARD);
                backward_wiring_map = Wiring::new(static_data::ROTOR_III_BACKWARD);
            },
            Rotors::IV => {
                transfer_position = static_data::ROTOR_IV_TRANSFER_POSITION;
                forward_wiring_map = Wiring::new(static_data::ROTOR_IV_FORWARD);
                backward_wiring_map = Wiring::new(static_data::ROTOR_IV_BACKWARD);
            },
            Rotors::V => {
                transfer_position = static_data::ROTOR_V_TRANSFER_POSITION;
                forward_wiring_map = Wiring::new(static_data::ROTOR_V_FORWARD);
                backward_wiring_map = Wiring::new(static_data::ROTOR_V_BACKWARD);
            }
        }
        
        Rotor{ forward_wiring_map, backward_wiring_map, transfer_position,
            offset, indicator}
    }

    /// Moves the Rotor on the spindle.
    /// A true return value means that the next Rotor needs an additional
    /// stepping.
    pub fn step(&mut self) -> bool {
        let old_indicator: Characters = self.indicator;
        self.indicator = self.indicator + Characters::A;
        if old_indicator == self.transfer_position {
            return true;
        }
        false
    }

    pub fn encipher_front_to_back(&self, input: Characters) -> Characters {
        // Account for how far the rotor has rotated around the spindle.
        let mut input: Characters = input + self.indicator;
        // Account for any initial rotation between the inner wiring and the
        // outer indicator ring of the rotor.
        input = input + self.offset;
        let mut output: Characters = self.forward_wiring_map[input];
        // Undo the prior adjustments to the input
        output = output - self.offset;
        return output - self.indicator;
    }

    pub fn encipher_back_to_front(&self, input: Characters) -> Characters {
                // Account for how far the rotor has rotated around the spindle.
        let mut input: Characters = input + self.indicator;
        // Account for any initial rotation between the inner wiring and the
        // outer indicator ring of the rotor.
        input = input + self.offset;
        let mut output: Characters = self.backward_wiring_map[input];
        // Undo the prior adjustments to the input
        output = output - self.offset;
        return output - self.indicator;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_encipherment() {
        let mut test_rotor = Rotor::new(Rotors::I,
            Characters::A,
            Characters::A);
        assert_eq!(test_rotor.encipher_front_to_back(Characters::A),
            Characters::E);
        test_rotor.step();
        assert_eq!(test_rotor.encipher_front_to_back(Characters::A),
            Characters::E);
    }

    #[test]
    fn test_backward_encipherment() {
        let mut test_rotor = Rotor::new(Rotors::I,
            Characters::A,
            Characters::A);
        assert_eq!(test_rotor.encipher_back_to_front(Characters::A),
            Characters::U);
        test_rotor.step();
        assert_eq!(test_rotor.encipher_back_to_front(Characters::A),
            Characters::U);
    }
}
