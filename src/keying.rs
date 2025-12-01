use std::vec::Vec;
use std::f32::consts::TAU;
use std::thread::sleep;
use std::time::Duration;

use phf::map::Map;
use phf::phf_map;
use tinyaudio::{
    OutputDeviceParameters,
    run_output_device
};

use crate::config::Config;

#[derive(Debug,Clone,Copy)]
enum Code {
    Dit,
    Dah,
    Char,
    Word,
    Sent,
    Inter,
}

impl Code {
    fn to_ms(&self, conf: &Config) -> u32 {
        match self {
            Code::Dit   => conf.char_sp_ms as u32,
            Code::Dah   => conf.char_sp_ms as u32 * 3,
            Code::Char  => conf.char_sp_ms as u32 * 3,
            Code::Word  => conf.char_sp_ms as u32 * 5 + conf.ex_wd_sp_ms as u32,
            Code::Sent  => conf.char_sp_ms as u32 * 7 + conf.ex_wd_sp_ms as u32,
            Code::Inter => conf.char_sp_ms as u32,
        }
    }
}

type Seq = Vec<Code>;

type Str = Vec<char>;

const CODE_DICT: Map<char, &'static [Code]> = phf_map! {
    'a' | 'A' => &[Code::Dit,Code::Inter,Code::Dah],
    'b' | 'B' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    'c' | 'C' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    'd' | 'D' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    'e' | 'E' => &[Code::Dit],
    'f' | 'F' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    'g' | 'G' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    'h' | 'H' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    'i' | 'I' => &[Code::Dit,Code::Inter,Code::Dit],
    'j' | 'J' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    'k' | 'K' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    'l' | 'L' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    'm' | 'M' => &[Code::Dah,Code::Inter,Code::Dah],
    'n' | 'N' => &[Code::Dah,Code::Inter,Code::Dit],
    'o' | 'O' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    'p' | 'P' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    'q' | 'Q' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    'r' | 'R' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    's' | 'S' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    't' | 'T' => &[Code::Dah],
    'u' | 'U' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    'v' | 'V' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    'w' | 'W' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    'x' | 'X' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    'y' | 'Y' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    'z' | 'Z' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit],

    '1' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    '2' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    '3' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    '4' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    '5' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '6' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '7' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '8' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '9' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    '0' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah],

    '?' => &[Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '!' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    '.' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    ',' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dah],
    ':' => &[Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit],
    '+' => &[Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    '-' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],
    '/' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah,Code::Inter,Code::Dit],
    '=' => &[Code::Dah,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dit,Code::Inter,Code::Dah],

    ' ' => &[Code::Word],
};

fn calc_len(seq: &Seq, conf: &Config) -> u32 {
    let ms: u32 = seq.into_iter()
        .map(|&chr| chr.to_ms(conf))
        .fold(0, |acc, x| acc + x);

    ms
}

pub fn gen_lev_chars(level: u8, sequence: &mut Str) {
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

pub fn play_chars(params: OutputDeviceParameters, conf: &Config, seq: Seq) {
    let len = calc_len(&seq, &conf) as usize;
    let mut mask = vec![false; 44 * len];

    let mut i: usize = 0;
    for code in seq.into_iter() {
        let ms = 44 * code.to_ms(&conf) as usize;
        match code {
            Code::Dit | Code::Dah => {
                mask[i..i+ms].copy_from_slice(vec![true; ms].as_slice());
            },
            _ => (),
        }
        i += ms;
    }

    let _device = run_output_device(
        params,
        {
            let mut clock = 0.0;
            let mut mask = mask.into_iter();
            move |data| {
                for samples in data.chunks_mut(params.channels_count) {
                    clock = (clock + 1.0) % params.sample_rate as f32;
                    let value = (clock * 440.0 * TAU / params.sample_rate as f32).sin();
                    for sample in samples {
                        if mask.next().unwrap_or(false) {
                            *sample = value;
                        }
                        else {
                            *sample = 0.0;
                        }
                    }
                }
            }
        }
    ).unwrap();

    println!("{:?}", len);
    sleep( Duration::from_millis(len as u64 / 2) );
}

fn string_to_morse(string: String) -> Seq {
    let chars = string.as_str()
        .chars();

    let mut code: Seq = Vec::new();

    for i in chars.into_iter() {
        let char_code = CODE_DICT.get(&i).unwrap();
        code.extend_from_slice(char_code);
        if i != ' ' {
            code.push(Code::Word);
        }
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_sound() {
        let params = OutputDeviceParameters {
            channels_count          : 2,
            sample_rate             : 44_100,
            channel_sample_count    : 441,
        };

        let paris: Seq = string_to_morse("paris codex PaRiS".to_string());

        println!("{:?}", paris);

        play_chars(
            params,
            &Config::init(),
            paris
        );
    }
    */

    #[test]
    fn test_sum() {
        let paris: Seq = string_to_morse("paris codex PaRiS".to_string());
        println!("{:?}", calc_len(&paris, &Config::init()));
    }
}
