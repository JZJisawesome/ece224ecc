/*
 * File:    lib.rs
 * Brief:   ECC homework helper program for ECE 224
 *
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

//TODO seperate this into multiple files

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use std::fmt;
use std::ops;
use std::str;
use std::convert;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

//Actually we'll play this simpler
#[derive(Debug, Clone)]
pub struct BitVec {
    data: Vec<bool>,//lsb to msb
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl BitVec {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    //Private helpers
    /*
    fn new() -> Self {
        BitVec {
            data: Vec::new()
        }
    }
    */

    fn with_capacity(capacity: usize) -> Self {
        BitVec {
            data: Vec::with_capacity(capacity)
        }
    }

    fn num_data_check_bits(&self) -> (usize, usize) {
        let total_bits = self.len();
        let mut num_check_bits = 0;
        while (1 << num_check_bits) < (total_bits + 1) {
            num_check_bits += 1;
        }
        let num_data_bits = total_bits - num_check_bits;
        (num_data_bits, num_check_bits)
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

pub trait DataBitVec {
    //Assuming just data in the BitVec
    fn num_data_bits(&self) -> usize;//The number present
    fn num_check_bits(&self) -> usize;//The number needed
    fn get_codeword(&self) -> BitVec;
    fn get_data_bits(&self) -> BitVec;
    fn get_check_bits(&self) -> BitVec;
}

pub trait CheckBitVec {
    //Impossible to determine data bits
    fn num_check_bits(&self) -> usize;//The number present
    fn get_check_bits(&self) -> BitVec;
}

pub trait SyndromeBitVec {
    //Impossible to determine data bits
    fn num_check_bits(&self) -> usize;//The number needed
    fn get_syndrome_bits(&self) -> BitVec;
}

pub trait CodewordBitVec {
    fn num_data_bits(&self) -> usize;//The number present
    fn num_check_bits(&self) -> usize;//The number present
    fn get_data_bits(&self) -> BitVec;
    fn get_check_bits(&self) -> BitVec;
    fn get_syndrome_bits(&self) -> BitVec;
    fn get_expected_check_bits(&self) -> BitVec;
    fn get_corrected_codeword(&self) -> Result<BitVec, ()>;
    fn print_table(&self);
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl DataBitVec for BitVec {
    fn num_data_bits(&self) -> usize {
        self.data.len()
    }
    fn num_check_bits(&self) -> usize {//The number needed
        let num_data_bits = DataBitVec::num_data_bits(self);
        let mut num_check_bits = 0;
        while (1 << num_check_bits) < (num_data_bits + num_check_bits + 1) {
            num_check_bits += 1;
        }
        num_check_bits
    }

    fn get_codeword(&self) -> BitVec {
        todo!()
    }
    fn get_data_bits(&self) -> BitVec {
        self.clone()
    }
    fn get_check_bits(&self) -> BitVec {
        todo!();
    }
}

impl CheckBitVec for BitVec {
    fn num_check_bits(&self) -> usize {
        self.len()
    }
    fn get_check_bits(&self) -> BitVec {
        self.clone()
    }
}

impl SyndromeBitVec for BitVec {
    fn num_check_bits(&self) -> usize {
        self.len()
    }

    fn get_syndrome_bits(&self) -> BitVec {
        self.clone()
    }
}

impl CodewordBitVec for BitVec {
    fn num_data_bits(&self) -> usize {
        let (num_data_bits, _) = self.num_data_check_bits();
        num_data_bits
    }

    fn num_check_bits(&self) -> usize {
        let (_, num_check_bits) = self.num_data_check_bits();
        num_check_bits
    }

    fn get_data_bits(&self) -> BitVec {
        self.iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| !((i + 1).is_power_of_two()))
            .map(|(_, bit)| bit)
            .collect()
    }

    fn get_check_bits(&self) -> BitVec {
        self.iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| (i + 1).is_power_of_two())
            .map(|(_, bit)| bit)
            .collect()
    }

    fn get_syndrome_bits(&self) -> BitVec {
        let check_bits = CodewordBitVec::get_check_bits(self);
        let expected_check_bits = self.get_expected_check_bits();
        std::iter::zip(check_bits.iter(), expected_check_bits.iter())
            .map(|(a, b)| a ^ b)
            .collect()
    }

    fn get_expected_check_bits(&self) -> BitVec {
        let mut expected_check_bits = BitVec::with_capacity(self.len());

        let (_, num_check_bits) = self.num_data_check_bits();

        for i in 0..num_check_bits {//Iterate over all check bits
            let check_bit_pos = 1 << i;//The actual check bit position
            expected_check_bits.data.push(false);

            for j in 0..self.len() {//Iterate over all data bits
                let data_bit_pos = j + 1;//The actual data bit position
                if data_bit_pos.is_power_of_two() {
                    continue;//Skip check bits
                }

                //We only include the data bit in the xor if
                //the relevant bit of the position is a 1
                if (data_bit_pos & check_bit_pos) != 0 {
                    expected_check_bits.data[i] ^= self[j];
                }
            }
        }
        expected_check_bits
    }

    fn get_corrected_codeword(&self) -> Result<BitVec, ()> {
        //TODO avoid conversion to usize
        let syndrome: usize = CodewordBitVec::get_syndrome_bits(self).try_into().unwrap();
        let mut corrected_codeword = self.clone();
        if syndrome != 0 {
            let bad_index = syndrome - 1;
            if bad_index >= self.len() {
                return Err(());
            }

            //Correct the invalid bit
            corrected_codeword.data[bad_index] ^= true;
        }
        Ok(corrected_codeword)
        /*
        self.iter().enumerate()
            .map(|(i, &bit)| {
                let pos = i + 1;
                if pos == syndrome {
                    !bit
                } else {
                    bit
                }
            })
            .collect()
        */
    }

    fn print_table(&self) {
        let (num_data_bits, num_check_bits) = self.num_data_check_bits();
        let total_bits = self.len();

        let expected_check_bits = self.get_expected_check_bits();

        let pwidth = usize::try_from(total_bits.ilog2() + 1).unwrap();
        let vwidth = usize::try_from(std::cmp::max(num_data_bits, num_check_bits).ilog10() + 1).unwrap();

        println!(
            "Position{} | Value{} | Expected ",
            " ".repeat(pwidth.saturating_sub(8)),
            " ".repeat(vwidth)
        );
        println!(
            "---------{}+-------{}+----------",
            "-".repeat(pwidth.saturating_sub(8)),
            "-".repeat(vwidth)
        );
        println!(
            "{}{} | Unused{} | N/A",
            " ".repeat(8usize.saturating_sub(pwidth)),
            "0".repeat(pwidth),
            " ".repeat(vwidth.saturating_sub(1))
        );

        let mut data_bit_counter = 0;
        let mut check_bit_counter = 0;
        for i in 1..=total_bits {
            println!(
                "{}{:0pwidth$b} | {}={} | {}",
                " ".repeat(8usize.saturating_sub(pwidth)),
                i,
                if i.is_power_of_two() {
                    format!(
                        "C[{:0vwidth$}]",
                        check_bit_counter,
                        vwidth = vwidth
                    )
                } else {
                    format!(
                        "D[{:0vwidth$}]",
                        data_bit_counter,
                        vwidth = vwidth
                    )
                },
                if self[i - 1] {1} else {0},
                if i.is_power_of_two() {
                    format!(
                        "{:b}",
                        if expected_check_bits[check_bit_counter] {1} else {0}
                    )
                } else {
                    "N/A".to_string()
                }
            );

            if i.is_power_of_two() {
                check_bit_counter += 1;
            } else {
                data_bit_counter += 1;
            }
        }
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        BitVec { data: Vec::from_iter(iter) }
    }
}

impl From<Vec<bool>> for BitVec {
    fn from(data: Vec<bool>) -> Self {
        BitVec {
            data,
        }
    }
}

impl convert::AsRef<Vec<bool>> for BitVec {
    fn as_ref(&self) -> &Vec<bool> {
        &self.data
    }
}

/*
impl convert::AsMut<Vec<bool>> for BitVec {
    fn as_mut(&mut self) -> &mut Vec<bool> {
        &mut self.data
    }
}
*/

impl convert::AsRef<[bool]> for BitVec {
    fn as_ref(&self) -> &[bool] {
        self.data.as_ref()
    }
}

/*
//Comment this out since we really don't want main modifying the BitVec itself anyways
impl convert::AsMut<[bool]> for BitVec {
    fn as_mut(&mut self) -> &mut [bool] {
        self.data.as_mut()
    }
}
*/

impl From<BitVec> for Vec<bool> {
    fn from(bitvec: BitVec) -> Self {
        bitvec.data
    }
}

impl From<usize> for BitVec {
    fn from(mut num: usize) -> Self {
        let mut bitvec = BitVec::with_capacity(usize::BITS as usize);
        while num != 0 {
            bitvec.data.push((num & 1) == 1);
            num >>= 1;
        }
        bitvec
    }
}

impl TryFrom<BitVec> for usize {
    type Error = ();

    fn try_from(bitvec: BitVec) -> Result<Self, Self::Error> {
        bitvec.data.iter()
            .enumerate()
            .fold(Ok(0), |wrapped_val, (i, &bit)|
                if i >= (usize::BITS as usize) {
                    Err(())
                } else {
                    match wrapped_val {
                        Ok(val) => Ok(val | if bit {1 << i} else {0}),
                        Err(()) => Err(())
                    }
                }
            )
    }
}

impl ops::Deref for BitVec {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/*
//Comment this out since we really don't want main modifying the BitVec itself anyways
impl ops::DerefMut for BitVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
*/

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bitstring: String = self.data.iter()
            .rev()//So we print msb -> lsb
            .map(|b| match b {
                true  => '1',
                false => '0',
            })
            .collect();
        write!(f, "{}", bitstring)
    }
}

impl str::FromStr for BitVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_bitvec: Result<Vec<bool>, ()> = s.chars()
            .rev()//So the vec is lsb -> msb
            .map(|c| match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(()),
            })
            .collect();
        Ok(BitVec { data: parsed_bitvec? })
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
