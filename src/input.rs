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

use std::{fs, io, path};

#[derive(Clone, Copy)]
pub enum ReadMode {
    FileName,
    StdIn,
}

pub struct Input {
    byte_streams: Vec<Vec<u8>>,
    read_mode: ReadMode,
}

impl Input {
    pub fn new() -> Input {
        Input {
            byte_streams: Vec::new(),
            read_mode: ReadMode::FileName,
        }
    }

    pub fn add_file(&mut self, file_path: path::PathBuf) {
        match fs::read(file_path.clone()) {
            Ok(bytes) => { self.byte_streams.push(bytes); }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::NotFound => {
                        eprintln!("Could not open file '{}' Not Found!",
                                  file_path.to_str().unwrap());
                    }
                    io::ErrorKind::PermissionDenied => {
                        eprintln!("Could not open file '{}' Permission denied!",
                                 file_path.to_str().unwrap());
                    }
                    _ => {
                        eprintln!("Could not open file '{}'!", file_path.to_str().unwrap());
                    }
                }
            }
        }
    }

    pub fn add_string_as_byte_stream(&mut self, string: String) {
        self.byte_streams.push(string.into_bytes());
    }

    pub fn read_mode(&self) -> ReadMode { self.read_mode }

    pub fn switch_read_mode(&mut self) {
        match self.read_mode {
            ReadMode::FileName => { self.read_mode = ReadMode::StdIn; }
            ReadMode::StdIn => { self.read_mode = ReadMode::FileName; }
        }
    }

    pub fn get_next_byte_stream(&mut self) -> Option<Vec<u8>> { self.byte_streams.pop() }
}

