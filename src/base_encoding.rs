/* Copyright (C) 2022  Fabian Moos
 * This file is part of encodex.
 *
 * encodex is free software: you can redistribute it and/or modify it under the terms of the GNU
 * General Public License as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * encodex is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with encodex. If not,
 * see <https://www.gnu.org/licenses/>.
 */

//! Functions for en-/decoding of different [base](crate::Base) types.

use std::collections::HashMap;

use crate::settings::{Base, EncodeMode, Settings};

/// Creates a [HashMap](std::collections::HashMap).
/// 
/// # Usage Example
///
/// The map is created from a list of n-tuples. The first element of the tuple is the key, the
/// second element is the value. Handing a 1-tuple to the macro is an error and will deny
/// compilation. If more than two elements are supplied as a tuple, every tuple-element with an
/// index greater than 1 will be ignored.
/// ```
/// use std::collections::HashMap;
/// use encodex::map;
///
/// let map = map![("first", 3), ("second", 1), ("third", 0)];
///
/// assert_eq!(map.get("first"), Some(&3));
/// assert_eq!(map.get("second"), Some(&1));
/// assert_eq!(map.get("third"), Some(&0));
/// ```
#[macro_export]
macro_rules! map {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x.0, $x.1);
            )*
            temp_map
        }
    };
}

/// A unit for en- or decoding a byte vector.
pub struct TranslationUnit {
    decoded_data: Option<Vec<u8>>,
    encoded_data: Option<Vec<u8>>,
    config: Settings,
}

impl TranslationUnit {
    /// Returns the [`Base`](crate::Base) the [`TranslationUnit`] used for de-/encoding.
    pub fn base(&self) -> Base { self.config.base() }

    /// Returns the [`EncodeMode`](crate::EncodeMode) of the [`TranslationUnit`].
    pub fn encode_mode(&self) -> EncodeMode { self.config.encode_mode() }

    /// Returns the decoded byte vector.
    pub fn get_decoded_data(&self) -> &Option<Vec<u8>> { &self.decoded_data }

    /// Returns the encoded byte vector.
    ///
    /// Every byte in the returned [`Vec`](std::vec::Vec) corresponds to a char of the
    /// alphabet as defined by the [`Base`](crate::Base) of this translation unit's
    /// [config field](crate::Settings).
    pub fn get_encoded_data(&self) -> &Option<Vec<u8>> { &self.encoded_data }

    /// Creates a new [`TranslationUnit`].
    ///
    /// The [configuration](crate::Settings) and data of a translation unit can't be changed after
    /// its creation. The way the data is interpreted depends on the config that has been used to
    /// create the unit. If it is created for encoding, the data is interpreted as an arbitrary byte
    /// vector. If it is created for decoding, the data is interpreted as a [`Base`](crate::Base)
    /// encoded string.
    pub fn new(data: Vec<u8>, config: Settings) -> TranslationUnit {
        match config.encode_mode() {
            EncodeMode::Decode => {
                TranslationUnit {
                    decoded_data: None,
                    encoded_data: Some(data),
                    config,
                }
            }
            EncodeMode::Encode => {
                TranslationUnit {
                    decoded_data: Some(data),
                    encoded_data: None,
                    config,
                }
            }
        }
    }

    /// Translates the data of the [`TranslationUnit`].
    ///
    /// This function translates the data when it is called for the first time. When called a more
    /// than once it does nothing.
    pub fn translate(&mut self) -> Result<(), String> {
        match self.config.encode_mode() {
            EncodeMode::Decode => {
                if let None = self.decoded_data { self.decode_dispatch() }
                else { Ok(()) }
            }
            EncodeMode::Encode => {
                if let None = self.encoded_data { self.encode_dispatch() }
                else { Ok(()) }
            }
        }
    }

    /// Dispatches the decoding process to the correct decode function. The decode function that is
    /// used depends on the [`Base`](crate::Base) value of the [config](crate::Settings) field.
    fn decode_dispatch(&mut self) -> Result<(), String> {
        match self.config.base() {
            Base::Guess => { todo!("Guess Base decoding is not yet implemented!"); }
            Base::Base64 | Base::Base64url => { self.from_base64() }
            Base::Base32 => { todo!("Base32 decoding is not yet implemented!"); }
            Base::Base32hex => { todo!("Base32hex decoding is not yet implemented!"); }
            Base::Base16 => { todo!("Base16 decoding is not yet implemented!"); }
        }
    }

    /// Dispatches the decoding process to the correct encode function. The encode function that is
    /// used depends on the [`Base`](crate::Base) value of the [config](crate::Settings) field.
    fn encode_dispatch(&mut self) -> Result<(), String> {
        match self.config.base() {
            Base::Guess => { todo!("Guess Base encoding is not yet implemented!"); }
            Base::Base64 | Base::Base64url => { self.to_base64() }
            Base::Base32 => { todo!("Base 32 encoding is not yet implemented!"); }
            Base::Base32hex => { todo!("Base32hex encoding is not yet implemented!"); }
            Base::Base16 => { todo!("Base16 encoding is not yet implemented!"); }
        }
    }

    /// Decodes a [`String`](std::string::String) that is encoded as [`Base64`](crate::Base::Base64)
    /// or [`Base64url`](crate::Base::Base64url).
    fn from_base64(&mut self) -> Result<(), String> {
        let alphabet: HashMap<char, u32> = match self.config.base() {
            Base::Base64 => {
                map![('A', 0), ('B', 1), ('C', 2), ('D', 3), ('E', 4), ('F', 5), ('G', 6), ('H', 7),
                     ('I', 8), ('J', 9), ('K', 10), ('L', 11), ('M', 12), ('N', 13), ('O', 14),
                     ('P', 15), ('Q', 16), ('R', 17), ('S', 18), ('T', 19), ('U', 20), ('V', 21),
                     ('W', 22), ('X', 23), ('Y', 24), ('Z', 25), ('a', 26), ('b', 27), ('c', 28),
                     ('d', 29), ('e', 30), ('f', 31), ('g', 32), ('h', 33), ('i', 34), ('j', 35),
                     ('k', 36), ('l', 37), ('m', 38), ('n', 39), ('o', 40), ('p', 41), ('q', 42),
                     ('r', 43), ('s', 44), ('t', 45), ('u', 46), ('v', 47), ('w', 48), ('x', 49),
                     ('y', 50), ('z', 51), ('0', 52), ('1', 53), ('2', 54), ('3', 55), ('4', 56),
                     ('5', 57), ('6', 58), ('7', 59), ('8', 60), ('9', 61), ('+', 62), ('/', 63),
                     ('=', 64)]
            }
            Base::Base64url => {
                map![('A', 0), ('B', 1), ('C', 2), ('D', 3), ('E', 4), ('F', 5), ('G', 6), ('H', 7),
                     ('I', 8), ('J', 9), ('K', 10), ('L', 11), ('M', 12), ('N', 13), ('O', 14),
                     ('P', 15), ('Q', 16), ('R', 17), ('S', 18), ('T', 19), ('U', 20), ('V', 21),
                     ('W', 22), ('X', 23), ('Y', 24), ('Z', 25), ('a', 26), ('b', 27), ('c', 28),
                     ('d', 29), ('e', 30), ('f', 31), ('g', 32), ('h', 33), ('i', 34), ('j', 35),
                     ('k', 36), ('l', 37), ('m', 38), ('n', 39), ('o', 40), ('p', 41), ('q', 42),
                     ('r', 43), ('s', 44), ('t', 45), ('u', 46), ('v', 47), ('w', 48), ('x', 49),
                     ('y', 50), ('z', 51), ('0', 52), ('1', 53), ('2', 54), ('3', 55), ('4', 56),
                     ('5', 57), ('6', 58), ('7', 59), ('8', 60), ('9', 61), ('-', 62), ('_', 63),
                     ('=', 64)]
            }
            _ => { return Err(String::from("Wrong encoding! This should not have happened!")); }
        };
        let encoded_data = self.encoded_data.as_ref().unwrap();
        if encoded_data.len() % 4 != 0 {
            return Err(String::from("Number of bytes for Base64 is not a multiple of 4!"));
        }
        let mut decoded_data = Vec::new();
        let mut iter = encoded_data.iter();
        let mut byte = iter.next();
        while byte != None {
            let mut block: u32 = 0;

            // Get first character of block.
            let mut character: char = char::from(byte.unwrap().clone());
            let num = alphabet.get(&character);
            let num = if let None = num {
                return Err(String::from("Non base64-alphabet character encountered!"));
            } else {
                num.unwrap()
            };
            block = block | (num << 18);

            // Get second character of block.
            byte = iter.next();
            character = char::from(byte.unwrap().clone());
            let num = alphabet.get(&character);
            let num = if let None = num {
                return Err(String::from("Non base64-alphabet character encountered!"));
            } else {
                num.unwrap()
            };
            block = block | (num << 12);

            // Get third character of block.
            byte = iter.next();
            character = char::from(byte.unwrap().clone());
            let num = alphabet.get(&character);
            let num = if let None = num {
                return Err(String::from("Non base64-alphabet character encountered!"));
            } else {
                num.unwrap()
            };
            let third_is_padding;
            if *num != 64 {
                block = block | (num << 6);
                third_is_padding = false;
            } else {
                third_is_padding = true;
            }

            // Get fourth character of block.
            byte = iter.next();
            character = char::from(byte.unwrap().clone());
            let num = alphabet.get(&character);
            let num = if let None = num {
                return Err(String::from("Non base64-alphabet character encountered!"));
            } else {
                num.unwrap()
            };
            let fourth_is_padding;
            if *num != 64 {
                block = block | num;
                fourth_is_padding = false;
            } else {
                fourth_is_padding = true;
            }

            decoded_data.push((block >> 16) as u8);
            if !third_is_padding { decoded_data.push((block >> 8) as u8); }
            if !fourth_is_padding { decoded_data.push(block as u8); }
            byte = iter.next();
        }
        self.decoded_data = Some(decoded_data);
        Ok(())
    }

    /// Encodes an arbitrary byte vector as [`Base64`](crate::Base::Base64) or
    /// [`Base64url`](crate::Base::Base64url) [`String`](std::string::String).
    fn to_base64(&mut self) -> Result<(), String> {
        let alphabet: Vec<char> = match self.config.base() {
            Base::Base64 => {
                vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                     'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                     'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                     'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/']
            }
            Base::Base64url => {
                vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                     'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                     'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                     'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_']
            }
            _ => { return Err(String::from("Wrong encoding! This should not have happened!")); }
        };
        let decoded_data = self.decoded_data.as_ref().unwrap();

        let mut encoded_data: Vec<u8> = Vec::new();
        let mut iter = decoded_data.iter();
        let mut byte = iter.next();
        while byte != None {
            let mut block: u32 = 0;
            let mut missing_bytes = 0;

            // Get bytes for next character.
            for count in 1..=3 {
                if let None = byte {
                    missing_bytes += 1;
                } else {
                    let value = byte.unwrap().clone();
                    match count {
                        1 => { block = block | ((value as u32) << 16); }
                        2 => { block = block | ((value as u32) << 8); }
                        3 => { block = block | (value as u32); }
                        _ => { }
                    }
                }
                byte = iter.next();
            }

            // Create first encoded character.
            let character = alphabet[(block >> 18) as usize];
            encoded_data.push(character as u8);

            // Create second encoded character.
            let character = alphabet[((block >> 12) & 0b111111) as usize];
            encoded_data.push(character as u8);

            // Create third encoded character.
            if missing_bytes == 2 {
                encoded_data.push('=' as u8);
            } else {
                let character = alphabet[((block >> 6) & 0b111111) as usize];
                encoded_data.push(character as u8);
            }

            // Create fourth encoded character.
            if missing_bytes >= 1 {
                encoded_data.push('=' as u8);
            } else {
                let character = alphabet[(block & 0b111111) as usize];
                encoded_data.push(character as u8);
            }
        }
        self.encoded_data = Some(encoded_data);
        Ok(())
    }
}

/// Test vectors for different encodings.
#[cfg(any(test, feature = "doc_tests"))]
mod tests {
    use super::*;

    fn setup_config_for_decode_base64() -> Settings {
        let mut config = Settings::new();
        config.set_base(Base::Base64);
        config.set_encode_mode(EncodeMode::Decode);
        config
    }

    fn setup_config_for_encode_base64() -> Settings {
        let mut config = Settings::new();
        config.set_base(Base::Base64);
        config.set_encode_mode(EncodeMode::Encode);
        config
    }

    fn setup_config_for_decode_base64url() -> Settings {
        let mut config = Settings::new();
        config.set_base(Base::Base64url);
        config.set_encode_mode(EncodeMode::Decode);
        config
    }

    fn setup_config_for_encode_base64url() -> Settings {
        let mut config = Settings::new();
        config.set_base(Base::Base64url);
        config.set_encode_mode(EncodeMode::Encode);
        config
    }

/**************************************************************************************************\
|********** Base64 Decode Tests *******************************************************************|
\**************************************************************************************************/

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64() {
        let mut t_unit = TranslationUnit::new(String::from("").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_f() {
        let mut t_unit = TranslationUnit::new(String::from("Zg==").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "f");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_fo() {
        let mut t_unit = TranslationUnit::new(String::from("Zm8=").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "fo");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_foo() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9v").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foo");
    }
    
    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_foob() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYg==").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foob");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_fooba() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYmE=").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "fooba");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_foobar() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYmFy").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foobar");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64_foobar_mima() {
        let mut t_unit = TranslationUnit::new(String::from("44G/44G+").into_bytes(),
                                              setup_config_for_decode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "みま");
    }

/**************************************************************************************************\
|********** Base64 Encode Tests *******************************************************************|
\**************************************************************************************************/

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64() {
        let mut t_unit = TranslationUnit::new(String::from("").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_f() {
        let mut t_unit = TranslationUnit::new(String::from("f").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zg==");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_fo() {
        let mut t_unit = TranslationUnit::new(String::from("fo").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm8=");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_foo() {
        let mut t_unit = TranslationUnit::new(String::from("foo").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9v");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_foob() {
        let mut t_unit = TranslationUnit::new(String::from("foob").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYg==");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_fooba() {
        let mut t_unit = TranslationUnit::new(String::from("fooba").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYmE=");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_foobar() {
        let mut t_unit = TranslationUnit::new(String::from("foobar").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYmFy");
    }
    
    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64_foobar_mima() {
        let mut t_unit = TranslationUnit::new(String::from("みま").into_bytes(),
                                              setup_config_for_encode_base64());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "44G/44G+");
    }

/**************************************************************************************************\
|********** Base64 Decode Tests *******************************************************************|
\**************************************************************************************************/

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url() {
        let mut t_unit = TranslationUnit::new(String::from("").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_f() {
        let mut t_unit = TranslationUnit::new(String::from("Zg==").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "f");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_fo() {
        let mut t_unit = TranslationUnit::new(String::from("Zm8=").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "fo");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_foo() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9v").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foo");
    }
    
    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_foob() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYg==").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foob");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_fooba() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYmE=").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "fooba");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_foobar() {
        let mut t_unit = TranslationUnit::new(String::from("Zm9vYmFy").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "foobar");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_decode_base64url_foobar_mima() {
        let mut t_unit = TranslationUnit::new(String::from("44G_44G-").into_bytes(),
                                              setup_config_for_decode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_decoded_data().as_ref().unwrap()).unwrap(),
                   "みま");
    }

/**************************************************************************************************\
|********** Base64url Encode Tests ****************************************************************|
\**************************************************************************************************/

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url() {
        let mut t_unit = TranslationUnit::new(String::from("").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_f() {
        let mut t_unit = TranslationUnit::new(String::from("f").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zg==");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_fo() {
        let mut t_unit = TranslationUnit::new(String::from("fo").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm8=");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_foo() {
        let mut t_unit = TranslationUnit::new(String::from("foo").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9v");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_foob() {
        let mut t_unit = TranslationUnit::new(String::from("foob").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYg==");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_fooba() {
        let mut t_unit = TranslationUnit::new(String::from("fooba").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYmE=");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_foobar() {
        let mut t_unit = TranslationUnit::new(String::from("foobar").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "Zm9vYmFy");
    }

    #[cfg_attr(not(feature = "doc_tests"), test)]
    fn test_translation_unit_encode_base64url_foobar_mima() {
        let mut t_unit = TranslationUnit::new(String::from("みま").into_bytes(),
                                              setup_config_for_encode_base64url());
        let result = t_unit.translate();
        assert_eq!(result, Ok(()));
        assert_eq!(std::str::from_utf8(&t_unit.get_encoded_data().as_ref().unwrap()).unwrap(),
                   "44G_44G-");
    }
}


