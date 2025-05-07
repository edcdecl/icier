// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::decoder::handlers::{OpCodeHandler, OpCodeHandlerDecodeFn};
use crate::decoder::table_de::*;
use std::sync::LazyLock;

pub(in crate::decoder) struct Tables {
	#[cfg(feature = "evex")]
	pub(in crate::decoder) invalid_map: [(OpCodeHandlerDecodeFn, &'static OpCodeHandler); 256],
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	invalid_map: (),

	pub(in crate::decoder) handlers_map0: [(OpCodeHandlerDecodeFn, &'static OpCodeHandler); 256],
	#[cfg(feature = "vex")]
	#[allow(dead_code)]
	pub(in crate::decoder) handlers_vex_map0: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "vex")]
	pub(in crate::decoder) handlers_vex_0f: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "vex")]
	pub(in crate::decoder) handlers_vex_0f38: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "vex")]
	pub(in crate::decoder) handlers_vex_0f3a: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_evex_0f: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_evex_0f38: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_evex_0f3a: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_evex_map5: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_evex_map6: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_xop_map8: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_xop_map9: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "evex")]
	pub(in crate::decoder) handlers_xop_map10: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "mvex")]
	pub(in crate::decoder) handlers_mvex_0f: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "mvex")]
	pub(in crate::decoder) handlers_mvex_0f38: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(feature = "mvex")]
	pub(in crate::decoder) handlers_mvex_0f3a: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(not(feature = "vex"))]
	#[allow(dead_code)]
	handlers_vex_map0: (),
	#[cfg(not(feature = "vex"))]
	#[allow(dead_code)]
	handlers_vex_0f: (),
	#[cfg(not(feature = "vex"))]
	#[allow(dead_code)]
	handlers_vex_0f38: (),
	#[cfg(not(feature = "vex"))]
	#[allow(dead_code)]
	handlers_vex_0f3a: (),
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	handlers_evex_0f: (),
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	handlers_evex_0f38: (),
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	handlers_evex_0f3a: (),
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	handlers_evex_map5: (),
	#[cfg(not(feature = "evex"))]
	#[allow(dead_code)]
	handlers_evex_map6: (),
	#[cfg(not(feature = "xop"))]
	#[allow(dead_code)]
	handlers_xop_map8: (),
	#[cfg(not(feature = "xop"))]
	#[allow(dead_code)]
	handlers_xop_map9: (),
	#[cfg(not(feature = "xop"))]
	#[allow(dead_code)]
	handlers_xop_map10: (),
	#[cfg(not(feature = "mvex"))]
	#[allow(dead_code)]
	handlers_mvex_0f: (),
	#[cfg(not(feature = "mvex"))]
	#[allow(dead_code)]
	handlers_mvex_0f38: (),
	#[cfg(not(feature = "mvex"))]
	#[allow(dead_code)]
	handlers_mvex_0f3a: (),
}

const fn build_tables() -> Tables {
	let handlers_map0 = read_legacy();
	#[cfg(feature = "vex")]
	let (handlers_vex_map0, handlers_vex_0f, handlers_vex_0f38, handlers_vex_0f3a) = read_vex();
	#[cfg(feature = "evex")]
	let (handlers_evex_0f, handlers_evex_0f38, handlers_evex_0f3a, handlers_evex_map5, handlers_evex_map6) = read_evex();
	#[cfg(feature = "evex")]
	let (handlers_xop_map8, handlers_xop_map9, handlers_xop_map10) = read_xop();
	#[cfg(feature = "mvex")]
	let (handlers_mvex_0f, handlers_mvex_0f38, handlers_mvex_0f3a) = read_mvex();
	#[cfg(not(feature = "vex"))]
	let (handlers_vex_map0, handlers_vex_0f, handlers_vex_0f38, handlers_vex_0f3a) = ((), (), (), ());
	#[cfg(not(feature = "evex"))]
	let (handlers_evex_0f, handlers_evex_0f38, handlers_evex_0f3a, handlers_evex_map5, handlers_evex_map6) = ((), (), (), (), ());
	#[cfg(not(feature = "xop"))]
	let (handlers_xop_map8, handlers_xop_map9, handlers_xop_map10) = ((), (), ());
	#[cfg(not(feature = "mvex"))]
	let (handlers_mvex_0f, handlers_mvex_0f38, handlers_mvex_0f3a) = ((), (), ());

	#[cfg(feature = "evex")]
	let invalid_map = [super::handlers::get_invalid_handler(); 0x100];
	#[cfg(not(feature = "evex"))]
	let invalid_map = ();
	Tables {
		invalid_map,
		handlers_map0,
		handlers_vex_map0,
		handlers_vex_0f,
		handlers_vex_0f38,
		handlers_vex_0f3a,
		handlers_evex_0f,
		handlers_evex_0f38,
		handlers_evex_0f3a,
		handlers_evex_map5,
		handlers_evex_map6,
		handlers_xop_map8,
		handlers_xop_map9,
		handlers_xop_map10,
		handlers_mvex_0f,
		handlers_mvex_0f38,
		handlers_mvex_0f3a,
	}
}

pub(in crate::decoder) const TABLES: Tables = build_tables();
