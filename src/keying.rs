use std::vec::Vec;
use std::f32::consts::TAU;
use std::thread::sleep;
use std::time::Duration;

use rand::{
    rng,
    Rng,
    seq::IndexedRandom,
};

use phf::map::Map;
use phf::phf_map;
use tinyaudio::{
    OutputDeviceParameters,
    run_output_device
};

use crate::config::{Config,Len};

#[derive(Debug,Clone,Copy)]
enum Code {
    Dit,
    Dah,
    Char,
    Word,
    Inter,
}

impl Code {
    fn to_duration(&self, conf: &Config) -> Duration {
        let char_len        = Duration::from_millis((1200 / conf.chr_wpm).into());
        let del_unit_len    = Duration::from_millis(
            (((60_000 / conf.trx_wpm) - (37_200 / conf.chr_wpm)) / 19).into()
            );
        match self {
            Code::Dit   => char_len,
            Code::Dah   => char_len.saturating_mul(3),
            Code::Inter => char_len,
            Code::Char  => del_unit_len.saturating_mul(3),
            Code::Word  => del_unit_len.saturating_mul(7),
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
// TODO: There has to be a better way.

fn calc_len(seq: &Seq, conf: &Config) -> Duration {
    let duration = seq.into_iter()
        .map(|&chr| chr.to_duration(conf))
        .fold(Duration::from_millis(0), |acc, x| acc.saturating_add(x));

    duration
}

fn string_to_morse(string: &String) -> Seq {
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

fn const_words(conf: &Config, len: u8, word_num: u16) -> String {
    let mut rng = rng();
    let word_num = conf.duration.as_secs() * conf.trx_wpm as u64 / 60;

    let mut phrase = String::new();
    for _i in 0..word_num {
        for _j in 0..len {
            phrase.push(conf.usables.choose(&mut rng).unwrap().clone());
        }
        phrase.push(' ');
    }

    phrase
}

fn rand_words(conf: &Config, low: u8, high: u8, word_num: u16) -> String {
    let mut rng = rng();

    let mut phrase = String::new();
    for _i in 1..word_num {
        for _j in 1..rng.random_range(low..=high) {
            phrase.push(conf.usables.choose(&mut rng).unwrap().clone());
        }
        phrase.push(' ');
    }

    phrase
}

pub fn random_string(conf: &Config) -> String {
    let low : u8;
    let high: u8;

    match conf.group_ln {
        Len::Constant(len) => {
            low  = len;
            high = len;
        },
        Len::Random(l, h)  => {
            low  = l;
            high = h;
        },
    }

    let mut rng = rng();
    let word_num = conf.duration.as_secs() * conf.trx_wpm as u64 / 60;

    let mut phrase = String::new();
    for _i in 1..word_num {
        for _j in 1..rng.random_range(low..=high) {
            phrase.push(conf.usables.choose(&mut rng).unwrap().clone());
        }
        phrase.push(' ');
    }

    phrase
}

pub fn play_chars(params: OutputDeviceParameters, conf: Config, seq: String) {
    let code = string_to_morse(&seq);
    let ch_cnt = params.channels_count;

    let len = calc_len(&code, &conf).as_millis() as usize;
    let mut mask = vec![false; 44 * ch_cnt * len];

    let mut i: usize = 0;
    for cd in code.into_iter() {
        let ms = 44 * ch_cnt * cd.to_duration(&conf).as_millis() as usize;
        match cd {
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

    sleep( Duration::from_millis(len as u64) );
}
// TODO: Remove the harsh blips.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_string() {
        let conf = Config::init();

        let params = OutputDeviceParameters {
            channels_count          : 2,
            sample_rate             : 44_100,
            channel_sample_count    : 441,
        };

        let string = random_string(&conf);

        play_chars(params, conf, string);
    }
}
