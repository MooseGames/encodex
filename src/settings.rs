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

//! Handles the configuration of [`TranslationUnit`](crate::TranslationUnit)s.

/// Describes all available Base encodings.
///
#[derive(Clone, Copy)]
pub enum Base {
    /// Alphabet:
    ///
    /// | Value | Encoding | Value | Encoding | Value | Encoding | Value | Encoding | Value | Encoding |
    /// |------:|:---------|------:|:---------|------:|:---------|------:|:---------|------:|:---------|
    /// |     0 | `A`      |    13 | `N`      |    26 | `a`      |    39 | `n`      |    52 | `0`      |
    /// |     1 | `B`      |    14 | `O`      |    27 | `b`      |    40 | `o`      |    53 | `1`      |
    /// |     2 | `C`      |    15 | `P`      |    28 | `c`      |    41 | `p`      |    54 | `2`      |
    /// |     3 | `D`      |    16 | `Q`      |    29 | `d`      |    42 | `q`      |    55 | `3`      |
    /// |     4 | `E`      |    17 | `R`      |    30 | `e`      |    43 | `r`      |    56 | `4`      |
    /// |     5 | `F`      |    18 | `S`      |    31 | `f`      |    44 | `s`      |    57 | `5`      |
    /// |     6 | `G`      |    19 | `T`      |    32 | `g`      |    45 | `t`      |    58 | `6`      |
    /// |     7 | `H`      |    20 | `U`      |    33 | `h`      |    46 | `u`      |    59 | `7`      |
    /// |     8 | `I`      |    21 | `V`      |    34 | `i`      |    47 | `v`      |    60 | `8`      |
    /// |     9 | `J`      |    22 | `W`      |    35 | `j`      |    48 | `w`      |    61 | `9`      |
    /// |    10 | `K`      |    23 | `X`      |    36 | `k`      |    49 | `x`      |    62 | `+`      |
    /// |    11 | `L`      |    24 | `Y`      |    37 | `l`      |    50 | `y`      |    63 | `/`      |
    /// |    12 | `M`      |    25 | `Z`      |    38 | `m`      |    51 | `z`      | (pad) | `=`
    Base64,
    /// The `Base64url` encoding has the same alphabet as the `Base64` alphabet. Only `+` and `/`
    /// are replaced by `-` (minus) and `_` (underscore) respectively.
    Base64url,
    /// todo
    Base32,
    /// todo
    Base32hex,
    /// todo
    Base16,
    /// todo
    Guess,
}

/// The encode mode that is used.
///
/// Default is [`Encode`](EncodeMode::Encode).
#[derive(Clone, Copy)]
pub enum EncodeMode {
    /// Decode the given input. Translate a Base encoded String into a byte vector.
    Decode,
    /// Encode the given input. Translate an arbitrary byte vector into a [`Base`](crate::Base)
    /// encoding.
    Encode,
}

/// Describes how a [`TranslationUnit`](crate::TranslationUnit) handles its input.
#[derive(Clone, Copy)]
pub struct Settings {
    base: Base,
    encode_mode: EncodeMode,
}

impl Settings {
    /// Creates a new configuration for a [`TranslationUnit`](crate::TranslationUnit).
    ///
    /// Default configurations are [`Guess`](Base::Guess) and [`Encode`](EncodeMode::Encode).
    pub fn new() -> Settings {
        Settings {
            base: Base::Guess,
            encode_mode: EncodeMode::Encode,
        }
    }

    /// Returns the [`Base`](Base) of this configuration.
    pub fn base(&self) -> Base { self.base }

    /// Returns the [encode mode](EncodeMode) of this configuration.
    pub fn encode_mode(&self) -> EncodeMode { self.encode_mode }

    /// Set a new [`Base`](Base) value for this configuration.
    pub fn set_base(&mut self, base: Base) { self.base = base; }

    /// Set a new [encode mode](EncodeMode) for this configuration.
    pub fn set_encode_mode(&mut self, mode: EncodeMode) { self.encode_mode = mode; }
}
 
