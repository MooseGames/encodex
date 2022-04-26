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

#[derive(Clone, Copy)]
pub enum Base {
    Base64,
    Base64url,
    Base32,
    Base32hex,
    Base16,
    Guess,
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
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            base: Base::Guess,
            encode_mode: EncodeMode::Encode,
            read_mode: ReadMode::FileName,
        }
    }

    pub fn base(&self) -> Base { self.base }

    pub fn encode_mode(&self) -> EncodeMode { self.encode_mode }

    pub fn read_mode(&self) -> ReadMode { self.read_mode }

    pub fn set_base(&mut self, base: Base) { self.base = base; }

    pub fn set_encode_mode(&mut self, mode: EncodeMode) { self.encode_mode = mode; }

    pub fn set_read_mode(&mut self, mode: ReadMode) { self.read_mode = mode; }
}
 
