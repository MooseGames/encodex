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

mod args;
mod input;

use std::process;

use encodex::{EncodeMode, TranslationUnit};

fn main() {
    let result = crate::args::parse_terminal_args();
    let (mut input, config) = match result {
        Ok((input, config)) => { (input, config) }
        Err(error_message) => {
            eprintln!("{}", error_message);
            process::exit(1);
        }
    };

    let mut data = input.get_next_byte_stream();
    while data != None {
        let bytes = data.unwrap();
        let mut translation_unit = TranslationUnit::new(bytes, config);
        if let Err(error_message) = translation_unit.translate() {
            eprintln!("{}", error_message);
            process::exit(1);
        }
        match config.encode_mode() {
            EncodeMode::Decode => { println!("{}", std::str::from_utf8(&translation_unit
                                                   .get_decoded_data().as_ref().unwrap())
                                                   .unwrap()); }
            EncodeMode::Encode => { println!("{}", std::str::from_utf8(&translation_unit
                                                   .get_encoded_data().as_ref().unwrap())
                                                   .unwrap()); }
        }

        data = input.get_next_byte_stream();
    }
}

