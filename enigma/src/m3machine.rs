// an_engima_of_a_site/enigma/src/m3machine.rs - A German Enigma Machine.
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

use crate::Characters;
use crate::Pairs;
use crate::Reflector;
use crate::Reflectors;
use crate::Rotor;
use crate::Rotors;
use crate::SteckerBoard;

#[derive(Debug)]
pub struct Indicators {
    slow: Characters,
    medium: Characters,
    fast: Characters,
}

impl std::str::FromStr for Indicators {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 { return Err("Must be three characters long."); }
        if !s.is_ascii() {
            return Err("The argument isn't entirely ASCII characters.");
        }
        let mut temp = s.chars();
        let slow = temp.next().unwrap();
        if !slow.is_alphabetic() { return Err("Must be an alphabetic \
character.");}
        let slow = Characters::try_from(slow).unwrap();
        let medium = temp.next().unwrap();
        if !medium.is_alphabetic() { return Err("Must be an alphabetic \
character.");}
        let medium = Characters::try_from(medium).unwrap();
        let fast = temp.next().unwrap();
        if !fast.is_alphabetic() {return Err("Must be an alphabetic \
character.");}
        let fast = Characters::try_from(fast).unwrap();
        Ok(Indicators{slow, medium, fast})
    }
}

impl std::ops::Index<RotorPosition> for Indicators {
    type Output = Characters;

    fn index(&self, position: RotorPosition) -> &Self::Output {
        match position {
            RotorPosition::Fast => &self.fast,
            RotorPosition::Medium => &self.medium,
            RotorPosition::Slow => &self.slow,
        }
    }
}

/// The German M3 enigma machine.
/// This is the version used by the Wermacht aka the German Army.
/// This is the completely prepared version ready for encipherment.
pub struct M3Machine {
    fast_rotor: Rotor,
    medium_rotor: Rotor,
    slow_rotor: Rotor,
    reflector: Reflector,
    stecker_board: SteckerBoard,
    step_slow_rotor: bool,
    step_medium_rotor: bool,
}

impl M3Machine {
    /// Configure a rotor for use inside the machine.
    /// The ringstellung is the letter from the Rotor's contact ring that
    /// is paired with "A" or "01" on the Rotor's indicator ring.
    pub fn configure_rotor(chosen: Rotors, ringstellung: Characters) -> Rotor {
        Rotor::new(chosen, ringstellung)
    }

    /// Perform the encipherment of a single character.
    ///
    /// The encipherment is symmetric meaning deciphering is the same as
    /// encipherment.
    pub fn encipher(&mut self, input: Characters) -> Characters {
        self.step_rotors();
        let mut output: Characters;
        output = self.stecker_board.encipher(input);
        output = self.fast_rotor.encipher_front_to_back(output);
        output = self.medium_rotor.encipher_front_to_back(output);
        output = self.slow_rotor.encipher_front_to_back(output);
        output = self.reflector.encipher(output);
        output = self.slow_rotor.encipher_back_to_front(output);
        output = self.medium_rotor.encipher_back_to_front(output);
        output = self.fast_rotor.encipher_back_to_front(output);
        self.stecker_board.encipher(output)
    }

    /// Get the indicator of a particular rotor.
    pub fn indicator(&self, position: RotorPosition) -> Characters {
        match position {
            RotorPosition::Fast => self.fast_rotor.indicator(),
            RotorPosition::Medium => self.medium_rotor.indicator(),
            RotorPosition::Slow => self.slow_rotor.indicator(),
        }
    }

    /// Get all the indicators.
    pub fn indicators(&self) -> Indicators {
        Indicators {
            slow: self.slow_rotor.indicator(),
            medium: self.medium_rotor.indicator(),
            fast: self.fast_rotor.indicator(),
        }
    }

    /// Allows you to change the indicator position for any specific rotor.
    pub fn set_indicator(&mut self, pos: RotorPosition, to: Characters) {
        match pos {
            RotorPosition::Fast => {
                self.step_medium_rotor = self.fast_rotor.set_indicator(to);
            }
            RotorPosition::Medium => {
                self.step_slow_rotor = self.medium_rotor.set_indicator(to);
            }
            RotorPosition::Slow => {
                // The M3Machine doesn't have a steppable reflector so we can
                // ignore the stepping transfer point for the slowest rotor.
                _ = self.slow_rotor.set_indicator(to);
            }
        }
    }

    /// Set all indicator positions.
    pub fn set_indicators(&mut self, indicators: Indicators) {
        self.step_medium_rotor = self
            .fast_rotor
            .set_indicator(indicators[RotorPosition::Fast]);
        self.step_slow_rotor = self
            .medium_rotor
            .set_indicator(indicators[RotorPosition::Medium]);
        _ = self
            .slow_rotor
            .set_indicator(indicators[RotorPosition::Slow]);
    }

    /// Step the fast rotor by one position forward.
    /// If any other rotor also needs to be stepped forward also step them
    /// forward.
    // None of the official rotors have transfer positions within two or three
    // positions of each other. As such we can assume that the flags will be
    // properly set to false as we move from left to right.
    fn step_rotors(&mut self) {
        if self.step_slow_rotor {
            _ = self.slow_rotor.step();
            _ = self.medium_rotor.step();
        }
        if self.step_medium_rotor {
            self.step_slow_rotor = self.medium_rotor.step();
        }
        self.step_medium_rotor = self.fast_rotor.step();
    }

    /// You start by unpacking the M3Machine and then configuring it.
    pub const fn unpack() -> M3MachineUnprepared {
        return M3MachineUnprepared {
            fast_rotor: None,
            medium_rotor: None,
            slow_rotor: None,
            reflector: None,
            stecker_board: None,
        };
    }
}

/// The German M3 enigma machine.
/// This is the version used by the Wermacht aka the German Army.
/// This is the machine being prepared for encipherment.
pub struct M3MachineUnprepared {
    fast_rotor: Option<Rotor>,
    medium_rotor: Option<Rotor>,
    slow_rotor: Option<Rotor>,
    reflector: Option<Reflector>,
    stecker_board: Option<SteckerBoard>,
}

impl M3MachineUnprepared {
    /// If setup is not complete then you will will get None.
    ///
    /// The indicators still need to be set appropriately.
    pub fn complete_setup(self) -> Option<M3Machine> {
        match (
            self.fast_rotor,
            self.medium_rotor,
            self.slow_rotor,
            self.reflector,
            self.stecker_board,
        ) {
            (Some(fr), Some(mr), Some(sr), Some(r), Some(sb)) => {
                return Some(M3Machine {
                    fast_rotor: fr,
                    medium_rotor: mr,
                    slow_rotor: sr,
                    reflector: r,
                    stecker_board: sb,
                    step_slow_rotor: false,
                    step_medium_rotor: false,
                });
            }
            _ => {
                return None;
            }
        }
    }

    /// "Place" the chosen Reflector into the machine.
    pub fn load_reflector(&mut self, chosen: Reflectors) {
        self.reflector = Some(Reflector::new(chosen));
    }

    /// "Place" the configured Rotor into the machine.
    /// Use `configure_rotor` to prepare a Rotor for loading.
    ///
    /// Returns false if unsuccessfull.
    ///
    /// Attempting to "place" the same Rotor in two different positions will
    /// fail.
    /// "Placing" the same Rotor in the same position is treated as
    /// reconfiguring the existing rotor.
    /// "Placing" a different Rotor in a filled position simply replaces the
    /// existing rotor.
    pub fn load_rotor(
        &mut self,
        position: RotorPosition,
        rotor: Rotor,
    ) -> bool {
        let rid = rotor.id();
        match position {
            RotorPosition::Fast => {
                if let Some(medium) = &self.medium_rotor {
                    if medium.id() == rid {
                        return false;
                    }
                }
                if let Some(slow) = &self.slow_rotor {
                    if slow.id() == rid {
                        return false;
                    }
                }
                self.fast_rotor = Some(rotor);
                return true;
            }
            RotorPosition::Medium => {
                if let Some(fast) = &self.fast_rotor {
                    if fast.id() == rid {
                        return false;
                    }
                }
                if let Some(slow) = &self.slow_rotor {
                    if slow.id() == rid {
                        return false;
                    }
                }
                self.medium_rotor = Some(rotor);
                return true;
            }
            RotorPosition::Slow => {
                if let Some(fast) = &self.fast_rotor {
                    if fast.id() == rid {
                        return false;
                    }
                }
                if let Some(medium) = &self.medium_rotor {
                    if medium.id() == rid {
                        return false;
                    }
                }
                self.slow_rotor = Some(rotor);
                return true;
            }
        }
    }

    ///
    pub fn wire_stecker_board(&mut self, sb: SteckerBoard) {
        self.stecker_board = Some(sb);
    }
}

/// Which Rotor position are we trying to insert.
pub enum RotorPosition {
    Fast,
    Medium,
    Slow,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    /// The encrypted messages were taken from
    /// http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Messages
    /// The decryptions were taken from
    /// http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Decrypts
    #[test]
    fn test_message_one() {
        let message_key_part_one: &str = "BLA";
        let message_key_part_two: &str = "LSD";
        let reflector: Reflectors = Reflectors::WideB;
        let slow_wheel: Rotors = Rotors::II;
        let medium_wheel: Rotors = Rotors::IV;
        let fast_wheel: Rotors = Rotors::V;
        let slow_wheel_ringstellung: Characters =
            Characters::try_from(01 as u8).unwrap();
        let medium_wheel_ringstellung: Characters =
            Characters::try_from(20 as u8).unwrap();
        let fast_wheel_ringstellung: Characters =
            Characters::try_from(11 as u8).unwrap();
        let steckerboard_sequence: &str = "AV:BS:CG:DL:FU:HZ:IN:KM:OW:RX";
        let encrypted_message_part_one: [&str; 4] = [
            "EDPUD NRGYS ZRCXN UYTPO MRMBO FKTBZ REZKM LXLVE FGUEY SIOZV EQMIK",
            "UBPMM YLKLT TDEIS MDICA GYKUA CTCDO MOHWX MUUIA UBSTS LRNBZ SZWNR",
            "FXWFY SSXJZ VIJHI DISHP RKLKA YUPAD TXQSP INQMA TLPIF SVKDA SCTAC",
            "DPBOP VHJK",
        ];
        let encrypted_message_part_two: [&str; 3] = [
            "SFBWD NJUSE GQOBH KRTAR EEZMW KPPRB XOHDR OEQGB BGTQV PGVKB VVGBI",
            "MHUSZ YDAJQ IROAX SSSNR EHYGG RPISE ZBOVM QIEMM ZCYSG QDGRE RVBIL",
            "EKXYQ IRGIR QNRDN VRXCY YTNJR",
        ];
        let decrypted_message_part_one: [&str; 4] = [
            "AUFKL XABTE ILUNG XVONX KURTI NOWAX KURTI NOWAX NORDW ESTLX SEBEZ",
            "XSEBE ZXUAF FLIEG ERSTR ASZER IQTUN GXDUB ROWKI XDUBR OWKIX OPOTS",
            "CHKAX OPOTS CHKAX UMXEI NSAQT DREIN ULLXU HRANG ETRET ENXAN GRIFF",
            "XINFX RGTX",
        ];
        let decrypted_message_part_two: [&str; 3] = [
            "DREIG EHTLA NGSAM ABERS IQERV ORWAE RTSXE INSSI EBENN ULLSE QSXUH",
            "RXROE MXEIN SXINF RGTXD REIXA UFFLI EGERS TRASZ EMITA NFANG XEINS",
            "SEQSX KMXKM XOSTW XKAME NECXK",
        ];

        // This is an example for how to use the M3Machine.
        let mut machine = M3Machine::unpack();
        machine.load_reflector(reflector);
        machine.load_rotor(RotorPosition::Slow,
            M3Machine::configure_rotor(slow_wheel, slow_wheel_ringstellung));
        machine.load_rotor(RotorPosition::Medium,
            M3Machine::configure_rotor(medium_wheel,
                medium_wheel_ringstellung));
        machine.load_rotor(RotorPosition::Fast,
            M3Machine::configure_rotor(fast_wheel, fast_wheel_ringstellung));
        let mut stecker_board: SteckerBoard = SteckerBoard::new();
        match Pairs::try_new(steckerboard_sequence) {
            Err(e) => {
                eprintln!("Opps {e:?} ocurred.");
                panic!();
            }
            Ok(pairs) => {
                stecker_board.add_jumper_wires(pairs);
                machine.wire_stecker_board(stecker_board);
            }
        }
        let indicators = Indicators::from_str(message_key_part_one).unwrap();
        let mut machine = machine.complete_setup().unwrap();
        machine.set_indicators(indicators);
        eprintln!("The indicators are {:?}", machine.indicators());
        let first_ciphertext = Characters::E;
        let first_plaintext = Characters::A;
        assert_eq!(machine.encipher(first_ciphertext), first_plaintext);
        eprintln!("The indicators are {:?}", machine.indicators());
        let first_ciphertext = Characters::D;
        let first_plaintext = Characters::U;
        assert_eq!(machine.encipher(first_ciphertext), first_plaintext);
    }

    // The encrypted messages were taken from
    // http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Messages
    // The decryptions were taken from
    // http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Decrypts
    // #[test]
    // fn test_message_two() {
    //     let message_key_part_three: &str = "UZV";
    //     let reflector: Reflectors = Reflectors::WideB;
    //     let slow_rotor: Rotors = Rotors::III;
    //     let medium_rotor: Rotors = Rotors::VI;
    //     let fast_rotor: Rotors = Rotors::VIII;
    //     let slow_wheel_ringstellung: Characters =
    //         Characters::try_from(00 as u8).unwrap();
    //     let medium_rotor_ringstellung: Characters =
    //         Characters::try_from(07 as u8).unwrap();
    //     let fast_rotor_ringstellung: Characters =
    //         Characters::try_from(12 as u8).unwrap();
    //     let steckerboard_sequence: &str = "AN:EZ:HK:IJ:LR:MQ:OT:PV:SW:UX";
    //     let encrypted_message_part_three: [&str; 2] = [
    //         "YKAEN ZAPMS CHZBF OCUVM RMDPY COFHA DZIZM EFXTH FLOLP ZLFGG BOTGO",
    //         "XGRET DWTJI QHLMX VJWKZ UASTR"];
    //     let decrypted_message_part_three: [&str; 2] = [
    //         "STEUE REJTA NAFJO RDJAN STAND ORTQU AAACC CVIER NEUNN EUNZW OFAHR",
    //         "TZWON ULSMX XSCHA RNHOR STHCO"];
    //     todo!();
    // }
}
