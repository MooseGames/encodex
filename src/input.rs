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

pub struct Input {
    files: Vec<path::PathBuf>,
    data_vector: Vec<String>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            files: Vec::new(),
            data_vector: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_path: path::PathBuf) { self.files.push(file_path); }

    pub fn add_data(&mut self, string: String) { self.data_vector.push(string); }

    pub fn get_next_file(&mut self) -> Option<path::PathBuf> { self.files.pop() }

    pub fn get_next_data(&mut self) -> Option<String> { self.data_vector.pop() }
}

