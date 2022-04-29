/* Copyright (C) 2022  Fabian Moos
 * This file is part of encodex.
 *
 * encodex is free software: you can redistribute it and/or modify it under the terms of the GNU
 * Lesser General Public License version 3 as published by the Free Software Foundation.
 *
 * encodex is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along with encodex.
 * If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env, path, process};

use crate::input::{Input, ReadMode};
use encodex::{Base, EncodeMode, Settings};

const OP_BASE: &str = "b";
const OP_BASE_LONG: &str = "base";
const OP_DECODE: &str = "d";
const OP_DECODE_LONG: &str = "decode";
const OP_HELP_LONG: &str = "help";
const OP_VERSION_LONG: &str = "version";

pub fn parse_terminal_args() -> Result<(Input, Settings), String> {
    let working_dir = match env::current_dir() {
        Ok(path) => { path }
        Err(error) => { panic!("{}", error); }
    };
    let mut settings = Settings::new();
    let mut input = Input::new();
    let mut arg_it = env::args().skip(1);
    let mut arg_opt = arg_it.next();

    while arg_opt != None {
        let arg = arg_opt.unwrap();
        let cmd_line_op;
        let current_value: &str;
        if arg.len() >= 2 && arg.is_ascii() && "--" == &arg[0..2] {
            cmd_line_op = true;
            current_value = &arg[2..];
        } else if arg.len() >= 1 && arg.is_ascii() && "-" == &arg[0..1] {
            cmd_line_op = true;
            current_value = &arg[1..];
        } else {
            cmd_line_op = false;
            current_value = &arg[..];
        }

        match current_value {
            OP_BASE_LONG | OP_BASE => {
                if let Err(error_message) = handle_base_type(&mut settings, arg_it.next()) {
                    return Err(String::from(error_message));
                }
            }
            OP_DECODE_LONG | OP_DECODE => { switch_encode_mode(&mut settings); }
            OP_HELP_LONG => { print_help(); process::exit(0); }
            OP_VERSION_LONG => { print_version(); process::exit(0); }
            "" => { input.switch_read_mode(); }
            &_ if !cmd_line_op => {
                handle_input(&mut input, current_value, &working_dir);
            }
            &_ => {
                print_help();
                let mut error_message = String::from(">>> Unrecognized option: '");
                error_message.push_str(arg.as_str());
                error_message.push_str("'");
                return Err(error_message);
            }
        }
        arg_opt = arg_it.next();
    }
    Ok((input, settings))
}

fn handle_base_type(settings: &mut Settings, base_type: Option<String>)
                    -> Result<(), &'static str> {
    match base_type {
        Some(base_type) => {
            match &base_type[..] {
                "Base64" => { settings.set_base(Base::Base64); Ok(()) }
                "Base64url" => { settings.set_base(Base::Base64url); Ok(()) }
                "Base32" => { settings.set_base(Base::Base32); Ok(()) }
                "Base32hex" => { settings.set_base(Base::Base32hex); Ok(()) }
                "Base16" => { settings.set_base(Base::Base16); Ok(()) }
                &_ => { Err(">>> Error: Unrecognized base type!") }
            }
        }
        None => { Err(">>> Error: No base type found for '--base' option!") }
    }
}

fn handle_input(input: &mut Input, value: &str, working_dir: &path::PathBuf) {
    match input.read_mode() {
        ReadMode::FileName => {
            let mut file_path = working_dir.clone();
            file_path.push(value);
            input.add_file(file_path);
        }
        ReadMode::StdIn => {
            input.add_string(String::from(value));
        }
    }
}

fn print_help() {
    println!("Usage: encodex [options] <file>... (todo)");
    println!("       encodex [options] -- <stdin>...");
    println!("  The default of the program is encoding input and printing it to stdout.");
    println!("  Every command line argument that is not prefixed with '-' or '--' and is not");
    println!("  empty will be interpreted as a file name to be encoded/decoded. '--' without any");
    println!("  suffix switches between file input and stdin.\n");
    println!("Options:");
    println!("  -{}, --{} <base>      Set encoding to: Base64, Base64url, Base32(todo),",
             OP_BASE, OP_BASE_LONG);
    println!("                         Base32hex(todo), Base16(todo). Default is 'Guess Base' (todo).");
    println!("  -{}, --{}           Decode input",
             OP_DECODE, OP_DECODE_LONG);
    println!("      --{}             Print this help and exit", OP_HELP_LONG);
    println!("      --{}          Print version and license information and exit\n",
             OP_VERSION_LONG);
    println!("The last parsed value for the -{} option determines the used base for encoding and",
             OP_BASE);
    println!(" decoding.");
}

fn print_version() {
    let mut version = String::from(env!("CARGO_PKG_VERSION_MAJOR"));
    version.push_str(".");
    version.push_str(env!("CARGO_PKG_VERSION_MINOR"));
    version.push_str(".");
    version.push_str(env!("CARGO_PKG_VERSION_PATCH"));
    let description = String::from(env!("CARGO_PKG_DESCRIPTION"));
    println!("encodex {}  {}\n\
              {}\n\
              Copyright (C) 2022  Fabian Moos\n\n\
              This program is free software: you can redistribute it and/or modify\n\
              it under the terms of the GNU Lesser General Public License as\n\
              published by the Free Software Foundation, either version 3 of the\n\
              License, or (at your option) any later version.\n\n\
              This program is distributed in the hope that it will be useful,\n\
              but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
              MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
              GNU Lesser General Public License for more details.\n\n\
              You should have received a copy of the GNU General Public License\n\
              along with this program.  If not, see <https://www.gnu.org/licenses/>.",
              version, &description[..51], &description[51..]);
}

fn switch_encode_mode(settings: &mut Settings) {
    match settings.encode_mode() {
        EncodeMode::Decode => { settings.set_encode_mode(EncodeMode::Encode); }
        EncodeMode::Encode => { settings.set_encode_mode(EncodeMode::Decode); }
    }
}

