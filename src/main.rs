/*
 * File:    main.rs
 * Brief:   ECC homework helper program for ECE 224 (frontend)
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
 * Uses
 * --------------------------------------------------------------------------------------------- */

use ece224ecc::BitVec;
use ece224ecc::Bin;

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        bad_usage();
        return;
    }

    let bitvec: BitVec = match args[2].parse() {
        Ok(bitvec) => bitvec,
        Err(_) => {
            bad_usage();
            return;
        }
    };

    //TODO add commands to create practice questions
    match args[1].as_str() {
        "e" => encode_subcmd(&bitvec),
        "d" => decode_subcmd(&bitvec),
        "p" => todo!(),
        _ => bad_usage(),
    }
}

fn bad_usage() {
    println!("Usage: ecc {{e, d}} code");
    println!("e: encode");
    println!("d: decode (and correct if necessary)");
    println!("p: generate practice questions");
}

fn encode_subcmd(data: &BitVec) {
    use ece224ecc::DataBitVec;
    let num_data_bits = data.len();
    let num_check_bits = data.check_bits_needed();
    let total_bits = num_data_bits + num_check_bits;

    println!("Encoding data              : \"{}\" (msb -> lsb)", data);
    println!("Number of data bits        : {}", num_data_bits);
    println!("Number of check bits needed: {}", num_check_bits);

    todo!()
}

fn decode_subcmd(codeword: &[bool]) {
//fn decode_subcmd(codeword: &BitVec) {
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
