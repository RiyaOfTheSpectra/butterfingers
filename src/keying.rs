use std::vec::Vec;
use std::collections::HashMap;
use deranged::RangedU8;

//use cpal::Data;
//use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

enum Code {
    Dit,
    Dah,
}

type Seq = Vec<Code>;

type CodeDict = HashMap<char, Seq>;

pub fn create_morse_characters() -> CodeDict {
    let mut code_dict: CodeDict = HashMap::new();

    code_dict.insert('a', vec![Code::Dit,Code::Dah]);
    code_dict.insert('b', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('c', vec![Code::Dah,Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert('d', vec![Code::Dah,Code::Dit,Code::Dit]);
    code_dict.insert('e', vec![Code::Dit]);
    code_dict.insert('f', vec![Code::Dit,Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert('g', vec![Code::Dah,Code::Dah,Code::Dit]);
    code_dict.insert('h', vec![Code::Dit,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('i', vec![Code::Dit,Code::Dit]);
    code_dict.insert('j', vec![Code::Dit,Code::Dah,Code::Dah,Code::Dah]);
    code_dict.insert('k', vec![Code::Dah,Code::Dit,Code::Dah]);
    code_dict.insert('l', vec![Code::Dit,Code::Dah,Code::Dit,Code::Dit]);
    code_dict.insert('m', vec![Code::Dah,Code::Dah]);
    code_dict.insert('n', vec![Code::Dah,Code::Dit]);
    code_dict.insert('o', vec![Code::Dah,Code::Dah,Code::Dah]);
    code_dict.insert('p', vec![Code::Dit,Code::Dah,Code::Dah,Code::Dit]);
    code_dict.insert('q', vec![Code::Dah,Code::Dah,Code::Dit,Code::Dah]);
    code_dict.insert('r', vec![Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert('s', vec![Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('t', vec![Code::Dah]);
    code_dict.insert('u', vec![Code::Dit,Code::Dit,Code::Dah]);
    code_dict.insert('v', vec![Code::Dit,Code::Dit,Code::Dit,Code::Dah]);
    code_dict.insert('w', vec![Code::Dit,Code::Dah,Code::Dah]);
    code_dict.insert('x', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dah]);
    code_dict.insert('y', vec![Code::Dah,Code::Dit,Code::Dah,Code::Dah]);
    code_dict.insert('z', vec![Code::Dah,Code::Dah,Code::Dit,Code::Dit]);

    code_dict.insert('1', vec![Code::Dit,Code::Dah,Code::Dah,Code::Dah,Code::Dah]);
    code_dict.insert('2', vec![Code::Dit,Code::Dit,Code::Dah,Code::Dah,Code::Dah]);
    code_dict.insert('3', vec![Code::Dit,Code::Dit,Code::Dit,Code::Dah,Code::Dah]);
    code_dict.insert('4', vec![Code::Dit,Code::Dit,Code::Dit,Code::Dit,Code::Dah]);
    code_dict.insert('5', vec![Code::Dit,Code::Dit,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('6', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('7', vec![Code::Dah,Code::Dah,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('8', vec![Code::Dah,Code::Dah,Code::Dah,Code::Dit,Code::Dit]);
    code_dict.insert('9', vec![Code::Dah,Code::Dah,Code::Dah,Code::Dah,Code::Dit]);
    code_dict.insert('0', vec![Code::Dah,Code::Dah,Code::Dah,Code::Dah,Code::Dah]);

    code_dict.insert('?', vec![Code::Dit,Code::Dit,Code::Dah,Code::Dah,Code::Dit,Code::Dit]);
    code_dict.insert('!', vec![Code::Dah,Code::Dit,Code::Dah,Code::Dit,Code::Dah,Code::Dah]);
    code_dict.insert('.', vec![Code::Dit,Code::Dah,Code::Dit,Code::Dah,Code::Dit,Code::Dah]);
    code_dict.insert(',', vec![Code::Dah,Code::Dah,Code::Dit,Code::Dit,Code::Dah,Code::Dah]);
    code_dict.insert(';', vec![Code::Dah,Code::Dit,Code::Dah,Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert(':', vec![Code::Dah,Code::Dah,Code::Dah,Code::Dit,Code::Dit,Code::Dit]);
    code_dict.insert('+', vec![Code::Dit,Code::Dah,Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert('-', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dit,Code::Dit,Code::Dah]);
    code_dict.insert('/', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dah,Code::Dit]);
    code_dict.insert('=', vec![Code::Dah,Code::Dit,Code::Dit,Code::Dit,Code::Dah]);

    code_dict
}

pub fn generate_level_characters(level: u8, mut sequence: Vec<char>) -> Option<Vec<char>> {
    match level {
        1  => vec!['k', 'm'].into(),
        2  => {
            sequence.push('u');
            Some(sequence)
        }
        3  => {
            sequence.push('r');
            Some(sequence)
        }
        4  => {
            sequence.push('e');
            Some(sequence)
        }
        5  => {
            sequence.push('s');
            Some(sequence)
        }
        6  => {
            sequence.push('n');
            Some(sequence)
        }
        7  => {
            sequence.push('a');
            Some(sequence)
        }
        8  => {
            sequence.push('p');
            Some(sequence)
        }
        9  => {
            sequence.push('t');
            Some(sequence)
        }
        10 => {
            sequence.push('l');
            Some(sequence)
        }
        11 => {
            sequence.push('w');
            Some(sequence)
        }
        12 => {
            sequence.push('i');
            Some(sequence)
        }
        13 => {
            sequence.push('.');
            Some(sequence)
        }
        14 => {
            sequence.push('j');
            Some(sequence)
        }
        15 => {
            sequence.push('z');
            Some(sequence)
        }
        16 => {
            sequence.push('=');
            Some(sequence)
        }
        17 => {
            sequence.push('f');
            Some(sequence)
        }
        18 => {
            sequence.push('o');
            Some(sequence)
        }
        19 => {
            sequence.push('y');
            Some(sequence)
        }
        20 => {
            sequence.push(',');
            Some(sequence)
        }
        21 => {
            sequence.push('v');
            Some(sequence)
        }
        22 => {
            sequence.push('g');
            Some(sequence)
        }
        23 => {
            sequence.push('5');
            Some(sequence)
        }
        24 => {
            sequence.push('1');
            Some(sequence)
        }
        25 => {
            sequence.push('q');
            Some(sequence)
        }
        26 => {
            sequence.push('9');
            Some(sequence)
        }
        27 => {
            sequence.push('2');
            Some(sequence)
        }
        28 => {
            sequence.push('h');
            Some(sequence)
        }
        29 => {
            sequence.push('3');
            Some(sequence)
        }
        30 => {
            sequence.push('8');
            Some(sequence)
        }
        31 => {
            sequence.push('b');
            Some(sequence)
        }
        32 => {
            sequence.push('?');
            Some(sequence)
        }
        33 => {
            sequence.push('4');
            Some(sequence)
        }
        34 => {
            sequence.push('7');
            Some(sequence)
        }
        35 => {
            sequence.push('c');
            Some(sequence)
        }
        36 => {
            sequence.push('1');
            Some(sequence)
        }
        37 => {
            sequence.push('d');
            Some(sequence)
        }
        38 => {
            sequence.push('6');
            Some(sequence)
        }
        39 => {
            sequence.push('0');
            Some(sequence)
        }
        40 => {
            sequence.push('x');
            Some(sequence)
        }
        _ => None
    }
}
