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

pub fn generate_level_characters(level: u8) -> Option<Vec<char>> {
    match level {
        1  => vec!['k', 'm'].into(),
        2  => {
            let mut result = generate_level_characters( 1)?;
            result.push('u');
            Some(result)
        }
        3  => {
            let mut result = generate_level_characters( 2)?;
            result.push('r');
            Some(result)
        }
        4  => {
            let mut result = generate_level_characters( 3)?;
            result.push('e');
            Some(result)
        }
        5  => {
            let mut result = generate_level_characters( 4)?;
            result.push('s');
            Some(result)
        }
        6  => {
            let mut result = generate_level_characters( 5)?;
            result.push('n');
            Some(result)
        }
        7  => {
            let mut result = generate_level_characters( 6)?;
            result.push('a');
            Some(result)
        }
        8  => {
            let mut result = generate_level_characters( 7)?;
            result.push('p');
            Some(result)
        }
        9  => {
            let mut result = generate_level_characters( 8)?;
            result.push('t');
            Some(result)
        }
        10 => {
            let mut result = generate_level_characters( 9)?;
            result.push('l');
            Some(result)
        }
        11 => {
            let mut result = generate_level_characters(10)?;
            result.push('w');
            Some(result)
        }
        12 => {
            let mut result = generate_level_characters(11)?;
            result.push('i');
            Some(result)
        }
        13 => {
            let mut result = generate_level_characters(12)?;
            result.push('.');
            Some(result)
        }
        14 => {
            let mut result = generate_level_characters(13)?;
            result.push('j');
            Some(result)
        }
        15 => {
            let mut result = generate_level_characters(14)?;
            result.push('z');
            Some(result)
        }
        16 => {
            let mut result = generate_level_characters(15)?;
            result.push('=');
            Some(result)
        }
        17 => {
            let mut result = generate_level_characters(16)?;
            result.push('f');
            Some(result)
        }
        18 => {
            let mut result = generate_level_characters(17)?;
            result.push('o');
            Some(result)
        }
        19 => {
            let mut result = generate_level_characters(18)?;
            result.push('y');
            Some(result)
        }
        20 => {
            let mut result = generate_level_characters(19)?;
            result.push(',');
            Some(result)
        }
        21 => {
            let mut result = generate_level_characters(20)?;
            result.push('v');
            Some(result)
        }
        22 => {
            let mut result = generate_level_characters(21)?;
            result.push('g');
            Some(result)
        }
        23 => {
            let mut result = generate_level_characters(22)?;
            result.push('5');
            Some(result)
        }
        24 => {
            let mut result = generate_level_characters(23)?;
            result.push('1');
            Some(result)
        }
        25 => {
            let mut result = generate_level_characters(24)?;
            result.push('q');
            Some(result)
        }
        26 => {
            let mut result = generate_level_characters(25)?;
            result.push('9');
            Some(result)
        }
        27 => {
            let mut result = generate_level_characters(26)?;
            result.push('2');
            Some(result)
        }
        28 => {
            let mut result = generate_level_characters(27)?;
            result.push('h');
            Some(result)
        }
        29 => {
            let mut result = generate_level_characters(28)?;
            result.push('3');
            Some(result)
        }
        30 => {
            let mut result = generate_level_characters(29)?;
            result.push('8');
            Some(result)
        }
        31 => {
            let mut result = generate_level_characters(30)?;
            result.push('b');
            Some(result)
        }
        32 => {
            let mut result = generate_level_characters(31)?;
            result.push('?');
            Some(result)
        }
        33 => {
            let mut result = generate_level_characters(32)?;
            result.push('4');
            Some(result)
        }
        34 => {
            let mut result = generate_level_characters(33)?;
            result.push('7');
            Some(result)
        }
        35 => {
            let mut result = generate_level_characters(34)?;
            result.push('c');
            Some(result)
        }
        36 => {
            let mut result = generate_level_characters(35)?;
            result.push('1');
            Some(result)
        }
        37 => {
            let mut result = generate_level_characters(36)?;
            result.push('d');
            Some(result)
        }
        38 => {
            let mut result = generate_level_characters(37)?;
            result.push('6');
            Some(result)
        }
        39 => {
            let mut result = generate_level_characters(38)?;
            result.push('0');
            Some(result)
        }
        40 => {
            let mut result = generate_level_characters(39)?;
            result.push('x');
            Some(result)
        }
        _ => None
    }
}
