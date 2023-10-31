/*
 * File:    ecc.rs
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

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

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

//TODO

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

trait Bin {
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

//TODO

impl Bin for &[bool] {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        bad_usage();
        return;
    }

    let binary: Vec<bool> = args[2]
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid code character: {}", c),
        })
        .rev()//So [0] is the lsb, [1] is the next bit, etc.
        .collect();

    //TODO add commands to create practice questions
    match args[1].as_str() {
        "e" => encode_subcmd(&binary),
        "d" => decode_subcmd(&binary),
        _ => bad_usage(),
    }
}

fn bad_usage() {
    println!("Usage: ecc {{e, d}} code");
    println!("e: encode");
    println!("d: decode (and correct if necessary)");
}

fn encode_subcmd(data: &[bool]) {
    let num_data_bits = data.len();
    let num_check_bits = data.check_bits_needed();
    let total_bits = num_data_bits + num_check_bits;

    println!("Encoding data              : \"{}\" (msb -> lsb)", data.bin_string());
    println!("Number of data bits        : {}", num_data_bits);
    println!("Number of check bits needed: {}", num_check_bits);

    todo!()
}

fn decode_subcmd(codeword: &[bool]) {
    let (num_data_bits, num_check_bits) = codeword.num_data_and_check_bits();
    let total_bits = num_data_bits + num_check_bits;

    println!("Decoding codeword   : \"{}\" (msb -> lsb)", codeword.bin_string());
    println!("Number of data bits : {}", num_data_bits);
    println!("Number of check bits: {}", num_check_bits);
    codeword.print_codeword_table();

    let data_bits = codeword.get_data_bits();
    let check_bits = codeword.get_check_bits();
    let expected_check_bits = codeword.get_expected_check_bits();
    let syndrome = codeword.get_syndrome();
    println!("Data bits          : \"{}\" (msb -> lsb)", (&data_bits[..]).bin_string());
    println!("Check bits         : \"{}\" (msb -> lsb)", (&check_bits[..]).bin_string());
    println!("Expected check bits: \"{}\" (msb -> lsb)", (&expected_check_bits[..]).bin_string());
    println!("Syndrome           : \"{}\" (msb -> lsb)", (&syndrome[..]).bin_string());

    let corrected = codeword.correct();
    let new_data_bits = (&corrected[..]).get_data_bits();
    let new_check_bits = (&corrected[..]).get_check_bits();
    let new_expected_check_bits = (&corrected[..]).get_expected_check_bits();
    let new_syndrome = (&corrected[..]).get_syndrome();
    println!("Corrected codeword : \"{}\" (msb -> lsb)", (&corrected[..]).bin_string());
    println!("New data bits      : \"{}\" (msb -> lsb)", (&new_data_bits[..]).bin_string());
    println!("New check bits     : \"{}\" (msb -> lsb)", (&new_check_bits[..]).bin_string());
    println!("New expected check : \"{}\" (msb -> lsb)", (&new_expected_check_bits[..]).bin_string());
    println!("New syndrome       : \"{}\" (msb -> lsb)", (&new_syndrome[..]).bin_string());
    (&corrected[..]).print_codeword_table();
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
