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

use std::env;
use std::path;

#[derive(Clone, Copy)]
pub enum Base {
    Base64,
    Base64url,
    Base32,
    Base32hex,
    Base16,
}

#[derive(Clone, Copy)]
pub enum EncodeMode {
    Decode,
    Encode,
}

#[derive(Clone, Copy)]
pub enum ReadMode {
    FileName,
    StdIn,
}

pub struct Settings {
    base: Base,
    encode_mode: EncodeMode,
    read_mode: ReadMode,
    working_dir: path::PathBuf,
    files: Vec<path::PathBuf>,
    stdin: Vec<String>,
}

impl Settings {
    pub fn new() -> Settings {
        let working_dir = match env::current_dir() {
            Ok(path) => { path }
            Err(error) => { panic!("{}", error); }
        };

        Settings {
            base: Base::Base64,
            encode_mode: EncodeMode::Encode,
            read_mode: ReadMode::FileName,
            working_dir,
            files: Vec::new(),
            stdin: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        let mut file_path = self.working_dir.clone();
        file_path.push(file_name);
        self.files.push(file_path);
    }

    pub fn add_string(&mut self, string: &str) { self.stdin.push(String::from(string)); }

    pub fn base(&self) -> Base { self.base }

    pub fn encode_mode(&self) -> EncodeMode { self.encode_mode }

    pub fn get_next_file(&mut self) -> Option<path::PathBuf> { self.files.pop() }

    pub fn get_next_string(&mut self) -> Option<String> { self.stdin.pop() }

    pub fn read_mode(&self) -> ReadMode { self.read_mode }

    pub fn working_dir(&self) -> path::PathBuf { self.working_dir.clone() }

    pub fn set_base(&mut self, base: Base) { self.base = base; }

    pub fn set_encode_mode(&mut self, mode: EncodeMode) { self.encode_mode = mode; }

    pub fn set_read_mode(&mut self, mode: ReadMode) { self.read_mode = mode; }
}
 
