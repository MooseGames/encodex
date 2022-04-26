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

use std::path;

#[derive(Clone, Copy)]
pub enum ReadMode {
    FileName,
    StdIn,
}

pub struct Input {
    files: Vec<path::PathBuf>,
    strings: Vec<String>,
    read_mode: ReadMode,
}

impl Input {
    pub fn new() -> Input {
        Input {
            files: Vec::new(),
            strings: Vec::new(),
            read_mode: ReadMode::FileName,
        }
    }

    pub fn add_file(&mut self, file_path: path::PathBuf) { self.files.push(file_path); }

    pub fn add_string(&mut self, string: String) { self.strings.push(string); }

    pub fn read_mode(&self) -> ReadMode { self.read_mode }

    pub fn switch_read_mode(&mut self) {
        match self.read_mode {
            ReadMode::FileName => { self.read_mode = ReadMode::StdIn; }
            ReadMode::StdIn => { self.read_mode = ReadMode::FileName; }
        }
    }
}

