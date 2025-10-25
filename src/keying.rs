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

pub fn generate_level_characters(level: u8, mut sequence: Vec<char>) -> Vec<char> {
    match level {
        1  => vec!['k', 'm'],
        2  => {
            sequence.push('u');
            sequence
        }
        3  => {
            sequence.push('r');
            sequence
        }
        4  => {
            sequence.push('e');
            sequence
        }
        5  => {
            sequence.push('s');
            sequence
        }
        6  => {
            sequence.push('n');
            sequence
        }
        7  => {
            sequence.push('a');
            sequence
        }
        8  => {
            sequence.push('p');
            sequence
        }
        9  => {
            sequence.push('t');
            sequence
        }
        10 => {
            sequence.push('l');
            sequence
        }
        11 => {
            sequence.push('w');
            sequence
        }
        12 => {
            sequence.push('i');
            sequence
        }
        13 => {
            sequence.push('.');
            sequence
        }
        14 => {
            sequence.push('j');
            sequence
        }
        15 => {
            sequence.push('z');
            sequence
        }
        16 => {
            sequence.push('=');
            sequence
        }
        17 => {
            sequence.push('f');
            sequence
        }
        18 => {
            sequence.push('o');
            sequence
        }
        19 => {
            sequence.push('y');
            sequence
        }
        20 => {
            sequence.push(',');
            sequence
        }
        21 => {
            sequence.push('v');
            sequence
        }
        22 => {
            sequence.push('g');
            sequence
        }
        23 => {
            sequence.push('5');
            sequence
        }
        24 => {
            sequence.push('1');
            sequence
        }
        25 => {
            sequence.push('q');
            sequence
        }
        26 => {
            sequence.push('9');
            sequence
        }
        27 => {
            sequence.push('2');
            sequence
        }
        28 => {
            sequence.push('h');
            sequence
        }
        29 => {
            sequence.push('3');
            sequence
        }
        30 => {
            sequence.push('8');
            sequence
        }
        31 => {
            sequence.push('b');
            sequence
        }
        32 => {
            sequence.push('?');
            sequence
        }
        33 => {
            sequence.push('4');
            sequence
        }
        34 => {
            sequence.push('7');
            sequence
        }
        35 => {
            sequence.push('c');
            sequence
        }
        36 => {
            sequence.push('1');
            sequence
        }
        37 => {
            sequence.push('d');
            sequence
        }
        38 => {
            sequence.push('6');
            sequence
        }
        39 => {
            sequence.push('0');
            sequence
        }
        40 => {
            sequence.push('x');
            sequence
        }
        _ => sequence
    }
}
