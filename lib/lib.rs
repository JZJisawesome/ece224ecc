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
    bitvec: Vec<bool>,//lsb to msb
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            bitvec: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.bitvec.len()
    }

    pub fn empty(&self) -> bool {
        self.bitvec.is_empty()
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

pub trait DataBitVec {
    //Assuming just data in the BitVec
    fn check_bits_needed(&self) -> usize;
    fn encode(&self) -> BitVec;
}

pub trait CheckBitVec {
}

pub trait SyndromeBitVec {
}

pub trait CodewordBitVec {
    //TODO
    //fn num_data_and_check_bits(&self) -> (usize, usize);
    //fn print_codeword_table(&self);
}

pub trait Bin {//Old
    fn bin_string(&self) -> String;
    fn into_usize(&self) -> usize;

    //Assuming just data
    fn check_bits_needed(&self) -> usize;
    fn encode(&self) -> Vec<bool>;

    //Assuming a whole codeword and in proper order
    fn get_data_bits(&self) -> Vec<bool>;
    fn get_check_bits(&self) -> Vec<bool>;
    fn get_expected_check_bits(&self) -> Vec<bool>;
    fn get_syndrome(&self) -> Vec<bool>;
    fn correct(&self) -> Vec<bool>;
    fn num_data_and_check_bits(&self) -> (usize, usize);
    fn print_codeword_table(&self);
    //TODO extract and calc check bits
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl DataBitVec for BitVec {
    fn check_bits_needed(&self) -> usize {
        let num_data_bits = self.bitvec.len();
        let mut num_check_bits = 0;
        while (1 << num_check_bits) < (num_data_bits + num_check_bits + 1) {
            num_check_bits += 1;
        }
        num_check_bits
    }

    fn encode(&self) -> BitVec {
        todo!()
    }
}

impl From<Vec<bool>> for BitVec {
    fn from(bitvec: Vec<bool>) -> Self {
        BitVec {
            bitvec,
        }
    }
}

impl convert::AsRef<Vec<bool>> for BitVec {
    fn as_ref(&self) -> &Vec<bool> {
        &self.bitvec
    }
}

impl convert::AsMut<Vec<bool>> for BitVec {
    fn as_mut(&mut self) -> &mut Vec<bool> {
        &mut self.bitvec
    }
}

impl convert::AsRef<[bool]> for BitVec {
    fn as_ref(&self) -> &[bool] {
        self.bitvec.as_ref()
    }
}

impl convert::AsMut<[bool]> for BitVec {
    fn as_mut(&mut self) -> &mut [bool] {
        self.bitvec.as_mut()
    }
}

impl From<BitVec> for Vec<bool> {
    fn from(bitvec: BitVec) -> Self {
        bitvec.bitvec
    }
}

impl From<usize> for BitVec {
    fn from(num: usize) -> Self {
        todo!()
    }
}

impl TryFrom<BitVec> for usize {
    type Error = ();

    fn try_from(bitvec: BitVec) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl ops::Deref for BitVec {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        &self.bitvec
    }
}

impl ops::DerefMut for BitVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bitvec
    }
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bitstring: String = self.bitvec.iter()
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
        Ok(BitVec { bitvec: parsed_bitvec? })
    }
}

impl Bin for &[bool] {//Old
    fn bin_string(&self) -> String {
        self.iter().rev()//So we print msb -> lsb
            .map(|b| match b {
                true  => '1',
                false => '0',
            })
            .collect()
    }
    fn into_usize(&self) -> usize {
        /*
        let mut val = 0;
        for i in 0..self.len() {
            if self[i] {
                val |= 1 << i;
            }
        }
        val
        */
        self.iter()
            .enumerate()
            .fold(0, |val, (i, &bit)| val | (if bit {1 << i} else {0}))
    }

    //Assuming just data
    fn check_bits_needed(&self) -> usize {
        let num_data_bits = self.len();
        let mut num_check_bits = 0;
        while (1 << num_check_bits) < (num_data_bits + num_check_bits + 1) {
            num_check_bits += 1;
        }
        num_check_bits
    }
    fn encode(&self) -> Vec<bool> {
        todo!()
    }

    //Assuming a whole codeword
    fn get_data_bits(&self) -> Vec<bool> {
        self.iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| !((i + 1).is_power_of_two()))
            .map(|(_, bit)| bit)
            .collect()
    }
    fn get_check_bits(&self) -> Vec<bool> {
        self.iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| (i + 1).is_power_of_two())
            .map(|(_, bit)| bit)
            .collect()
    }
    fn get_expected_check_bits(&self) -> Vec<bool> {
        let mut expected_check_bits: Vec<bool> = Vec::with_capacity(self.len());

        let (_, num_check_bits) = self.num_data_and_check_bits();

        for i in 0..num_check_bits {//Iterate over all check bits
            let check_bit_pos = 1 << i;//The actual check bit position
            expected_check_bits.push(false);

            for j in 0..self.len() {//Iterate over all data bits
                let data_bit_pos = j + 1;//The actual data bit position
                if data_bit_pos.is_power_of_two() {
                    continue;//Skip check bits
                }

                //We only include the data bit in the xor if
                //the relevant bit of the position is a 1
                if (data_bit_pos & check_bit_pos) != 0 {
                    expected_check_bits[i] ^= self[j];
                }
            }
        }
        expected_check_bits
    }
    fn get_syndrome(&self) -> Vec<bool> {
        let check_bits = self.get_check_bits();
        let expected_check_bits = self.get_expected_check_bits();
        std::iter::zip(check_bits.iter(), expected_check_bits.iter())
            .map(|(a, b)| a ^ b)
            .collect()
    }
    fn correct(&self) -> Vec<bool> {
        let syndrome = self.get_syndrome();
        let syndromeu = (&syndrome[..]).into_usize();
        self.iter().enumerate()
            .map(|(i, &bit)| {
                let pos = i + 1;
                if pos == syndromeu {
                    !bit
                } else {
                    bit
                }
            })
            .collect()
    }
    fn num_data_and_check_bits(&self) -> (usize, usize) {
        let total_bits = self.len();
        let mut num_check_bits = 0;
        while (1 << num_check_bits) < (total_bits + 1) {
            num_check_bits += 1;
        }
        let num_data_bits = total_bits - num_check_bits;
        (num_data_bits, num_check_bits)
    }

    fn print_codeword_table(&self) {
        let (num_data_bits, num_check_bits) = self.num_data_and_check_bits();
        let total_bits = num_data_bits + num_check_bits;

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
