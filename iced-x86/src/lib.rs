// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/icedland/iced/master/logo.png")]
#![allow(unknown_lints)]
#![warn(clippy::pedantic)]
#![warn(clippy::expect_used, clippy::unwrap_used)]
#![doc(test(attr(deny(warnings))))]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "encoder", feature = "block_encoder"))]
mod block_enc;
mod code;
#[cfg(feature = "code_asm")]
pub mod code_asm;
#[cfg(any(feature = "decoder", feature = "encoder"))]
mod constant_offsets;
#[cfg(any(feature = "decoder", feature = "gas", feature = "intel", feature = "masm", feature = "nasm", feature = "fast_fmt"))]
mod data_reader;
#[cfg(feature = "decoder")]
mod decoder;
#[cfg(feature = "encoder")]
mod encoder;
mod enums;
#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm", feature = "fast_fmt"))]
mod formatter;
pub(crate) mod iced_constants;
mod iced_features;
#[cfg(feature = "instr_info")]
mod info;
mod instruction;
#[cfg(feature = "encoder")]
mod instruction_create;
mod instruction_internal;
mod instruction_memory_sizes;
mod instruction_op_counts;
mod memory_size;
mod mnemonic;
mod mnemonics;
#[cfg(feature = "mvex")]
mod mvex;
mod register;
#[cfg(test)]
pub(crate) mod test;
#[cfg(test)]
pub(crate) mod test_utils;
#[cfg(any(feature = "decoder", feature = "encoder"))]
mod tuple_type_tbl;

#[cfg(all(feature = "encoder", feature = "block_encoder"))]
pub use crate::block_enc::*;
pub use crate::code::*;
#[cfg(any(feature = "decoder", feature = "encoder"))]
pub use crate::constant_offsets::*;
#[cfg(feature = "decoder")]
pub use crate::decoder::*;
#[cfg(feature = "encoder")]
pub use crate::encoder::*;
pub use crate::enums::*;
#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm", feature = "fast_fmt"))]
pub use crate::formatter::*;
pub use crate::iced_error::*;
pub use crate::iced_features::*;
#[cfg(feature = "instr_info")]
pub use crate::info::*;
pub use crate::instruction::*;
pub use crate::memory_size::*;
pub use crate::mnemonic::*;
pub use crate::register::*;
