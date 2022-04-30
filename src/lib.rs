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

//! En-/Decoder for [`Base`](crate::Base) encodings as defined by **RFC 4648**.
//!
//! Currently it is assumed that the encoded byte vector is in the correct format to be processed by
//! the translate function. Nothing is assumed for a decoded byte vector. Every vector of arbitrary
//! bytes can be encoded.
//!
//! # Usage Example
//!
//! ```
//! let mut config = encodex::Settings::new();
//! config.set_base(encodex::Base::Base64);
//! config.set_encode_mode(encodex::EncodeMode::Decode);
//!
//! let mut unit = encodex::TranslationUnit::new(String::from("d2FpZnU=").into_bytes(), config);
//! let result = unit.translate();
//!
//! assert_eq!(result, Ok(()));
//! assert_eq!(std::str::from_utf8(&unit.get_decoded_data().as_ref().unwrap()).unwrap(), "waifu");
//! ```

mod base_encoding;
mod settings;

pub use base_encoding::TranslationUnit;
pub use settings::{Base, EncodeMode, Settings};

