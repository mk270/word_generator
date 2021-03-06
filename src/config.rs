extern crate rustc_serialize;

use rustc_serialize::*;
use std::io::prelude::*;
use std::fs::File;


#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct OutputSettings {
    pub show_word_rewrites : bool,
    pub show_syllable_strings : bool,
    pub only_mark_rejects : bool,
    pub output_file : String,
    pub word_count : usize,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Grapheme {
    pub string : String,
    pub weight : usize
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct GraphemeGroup {
    pub name : String,
    pub graphemes : Vec<Grapheme>,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Syllable {
    pub string : String,
    pub weight : usize,
    pub only_first_syllable : bool,
    pub only_last_syllable : bool
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Rewrite {
    pub pattern : String,
    pub replace : String
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct RewriteGroup {
    pub syllable_rewrites : Vec<Rewrite>,
    pub grapheme_rewrites : Vec<Rewrite>
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct RejectGroup {
    pub syllable_rejects : Vec<String>,
    pub grapheme_rejects : Vec<String>
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct GenerateSettings {
    pub syllable_decay_rate : f32,
    pub max_syllables : usize,
    pub rewrites_before_rejects : bool,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct WordGeneratorConfig {
    pub output_settings : OutputSettings,
    pub generate_settings : GenerateSettings,
    pub graphemes : Vec<GraphemeGroup>,
    pub syllables : Vec<Syllable>,
    pub rewrites : RewriteGroup,
    pub rejects : RejectGroup
}

#[allow(dead_code)]
pub fn generate_test_config() -> WordGeneratorConfig {
    WordGeneratorConfig {
        generate_settings: GenerateSettings {
            syllable_decay_rate: 0.2,
            max_syllables: 10,
            rewrites_before_rejects: false,
        },
        output_settings: OutputSettings {
            output_file: "test".to_string(),
            word_count: 10,
            show_word_rewrites: false,
            show_syllable_strings: false,
            only_mark_rejects: false,
        },
        graphemes: vec![
            GraphemeGroup {
                name:"c".to_string(),
                graphemes:vec![
                    Grapheme {
                        string: "e".to_string(),
                        weight: 10,
                    }
                ],
            },
        ],
        syllables: vec![
            Syllable{
                string: "CV".to_string(),
                weight: 20,
                only_first_syllable: false,
                only_last_syllable: false,
            }
        ],
        rewrites: RewriteGroup {
            syllable_rewrites: vec![
                Rewrite{
                    pattern:"abc".to_string(),
                    replace:"def".to_string()
                }
            ],
            grapheme_rewrites: vec![
                Rewrite{
                    pattern:"abc".to_string(),
                    replace:"def".to_string()
                }
            ],
        },
        rejects: RejectGroup {
            syllable_rejects: vec![
                "abc".to_string(),
            ],
            grapheme_rejects: vec![
                "abc".to_string(),
            ],

        }
    }
}

pub fn load_config(config_name :&str) -> WordGeneratorConfig {
    let mut config_file = File::open(config_name).unwrap();

    let mut file_buffer : Vec<u8> = Vec::new();

    match config_file.read_to_end(&mut file_buffer) {
        Ok(_) => (),
        Err(error) => panic!("Error reading config: {}", error),
    };

    let config_encoded: String = match String::from_utf8(file_buffer) {
        Ok(result) => result,
        Err(error) => panic!("Error converting config u8 buffer to a String: {}", error),
    };

    match json::decode(&config_encoded) {
        Ok(result) => result,
        Err(error) => panic!("Error converting encoded config JSON to struct: {}", error),
    }
}
