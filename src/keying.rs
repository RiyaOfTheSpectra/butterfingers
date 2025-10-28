use std::vec::Vec;
use std::collections::HashMap;

use cpal::Data;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Debug,Clone,Copy)]
enum Code {
    Dit,
    Dah,
    Char,
    Word,
    Sentence,
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

    code_dict.insert(' ', vec![Code::Char]);

    code_dict
}

pub fn gen_lev_chars(level: u8, sequence: &mut Vec<char>) {
    match level {
        1  => *sequence = vec!['k', 'm'],
        2  => sequence.push('u'),
        3  => sequence.push('r'),
        4  => sequence.push('e'),
        5  => sequence.push('s'),
        6  => sequence.push('n'),
        7  => sequence.push('a'),
        8  => sequence.push('p'),
        9  => sequence.push('t'),
        10 => sequence.push('l'),
        11 => sequence.push('w'),
        12 => sequence.push('i'),
        13 => sequence.push('.'),
        14 => sequence.push('j'),
        15 => sequence.push('z'),
        16 => sequence.push('='),
        17 => sequence.push('f'),
        18 => sequence.push('o'),
        19 => sequence.push('y'),
        20 => sequence.push(','),
        21 => sequence.push('v'),
        22 => sequence.push('g'),
        23 => sequence.push('5'),
        24 => sequence.push('1'),
        25 => sequence.push('q'),
        26 => sequence.push('9'),
        27 => sequence.push('2'),
        28 => sequence.push('h'),
        29 => sequence.push('3'),
        30 => sequence.push('8'),
        31 => sequence.push('b'),
        32 => sequence.push('?'),
        33 => sequence.push('4'),
        34 => sequence.push('7'),
        35 => sequence.push('c'),
        36 => sequence.push('1'),
        37 => sequence.push('d'),
        38 => sequence.push('6'),
        39 => sequence.push('0'),
        40 => sequence.push('x'),
        _ => {}
    }
}
