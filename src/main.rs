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

use ece224ecc::*;

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
    let num_data_bits = data.len();
    let num_check_bits = DataBitVec::num_check_bits(data);
    //let total_bits = num_data_bits + num_check_bits;

    println!("Encoding data              : \"{}\" (msb -> lsb)", data);
    println!("Number of data bits        : {}", num_data_bits);
    println!("Number of check bits needed: {}", num_check_bits);

    let codeword = DataBitVec::get_codeword(data);
    let check_bits = CodewordBitVec::get_check_bits(&codeword);
    println!("Codeword                   : \"{}\" (msb -> lsb)", codeword);
    println!(
        "Check bits                 : \"{}\" (msb -> lsb)",
        check_bits
    );
    codeword.print_table();
}

fn decode_subcmd(codeword: &BitVec) {
    let num_data_bits = CodewordBitVec::num_data_bits(codeword);
    let num_check_bits = CodewordBitVec::num_check_bits(codeword);

    println!("Decoding codeword   : \"{}\" (msb -> lsb)", codeword);
    println!("Number of data bits : {}", num_data_bits);
    println!("Number of check bits: {}", num_check_bits);
    codeword.print_table();

    let data_bits = CodewordBitVec::get_data_bits(codeword);
    let check_bits = CodewordBitVec::get_check_bits(codeword);
    let expected_check_bits = codeword.get_expected_check_bits();
    let syndrome = CodewordBitVec::get_syndrome_bits(codeword);
    println!("Data bits          : \"{}\" (msb -> lsb)", data_bits);
    println!("Check bits         : \"{}\" (msb -> lsb)", check_bits);
    println!(
        "Expected check bits: \"{}\" (msb -> lsb)",
        expected_check_bits
    );
    println!("Syndrome           : \"{}\" (msb -> lsb)", syndrome);

    let corrected = codeword.get_corrected_codeword().unwrap(); //TODO handle properly
    let new_data_bits = CodewordBitVec::get_data_bits(&corrected);
    let new_check_bits = CodewordBitVec::get_check_bits(&corrected);
    let new_expected_check_bits = CodewordBitVec::get_expected_check_bits(&corrected);
    let new_syndrome = CodewordBitVec::get_syndrome_bits(&corrected);
    println!("Corrected codeword : \"{}\" (msb -> lsb)", corrected);
    println!("New data bits      : \"{}\" (msb -> lsb)", new_data_bits);
    println!("New check bits     : \"{}\" (msb -> lsb)", new_check_bits);
    println!(
        "New expected check : \"{}\" (msb -> lsb)",
        new_expected_check_bits
    );
    println!("New syndrome       : \"{}\" (msb -> lsb)", new_syndrome);
    corrected.print_table();
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
