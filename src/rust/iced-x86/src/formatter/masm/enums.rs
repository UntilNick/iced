/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::fmt;

// GENERATOR-BEGIN: CtorKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum CtorKind {
	Previous,
	Normal_1,
	Normal_2,
	AamAad,
	AX,
	AY,
	bnd_1,
	bnd_2,
	DeclareData,
	DX,
	fword,
	Ib,
	imul,
	invlpga,
	jcc,
	maskmovq,
	memsize,
	mmxmem_1,
	mmxmem_2,
	monitor,
	mwait,
	mwaitx,
	nop,
	OpSize_1,
	OpSize_2,
	OpSize2,
	OpSize2_bnd,
	pblendvb,
	pclmulqdq,
	pops_2,
	pops_3,
	pushm,
	reg,
	Reg16,
	reverse2,
	ST_STi,
	STi_ST,
	STi_ST2,
	STIG1_1,
	STIG1_2,
	XLAT,
	XY,
	YA,
	YD,
	YX,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CTOR_KIND: [&str; 45] = [
	"Previous",
	"Normal_1",
	"Normal_2",
	"AamAad",
	"AX",
	"AY",
	"bnd_1",
	"bnd_2",
	"DeclareData",
	"DX",
	"fword",
	"Ib",
	"imul",
	"invlpga",
	"jcc",
	"maskmovq",
	"memsize",
	"mmxmem_1",
	"mmxmem_2",
	"monitor",
	"mwait",
	"mwaitx",
	"nop",
	"OpSize_1",
	"OpSize_2",
	"OpSize2",
	"OpSize2_bnd",
	"pblendvb",
	"pclmulqdq",
	"pops_2",
	"pops_3",
	"pushm",
	"reg",
	"Reg16",
	"reverse2",
	"ST_STi",
	"STi_ST",
	"STi_ST2",
	"STIG1_1",
	"STIG1_2",
	"XLAT",
	"XY",
	"YA",
	"YD",
	"YX",
];
impl fmt::Debug for CtorKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CTOR_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for CtorKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CtorKind::Previous
	}
}
// GENERATOR-END: CtorKind

// GENERATOR-BEGIN: InstrOpInfoFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct InstrOpInfoFlags;
impl InstrOpInfoFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const MEM_SIZE_MASK: u32 = 0x0000_0007;
	pub(crate) const MEM_SIZE_SSE: u32 = 0x0000_0000;
	pub(crate) const MEM_SIZE_MMX: u32 = 0x0000_0001;
	pub(crate) const MEM_SIZE_NORMAL: u32 = 0x0000_0002;
	pub(crate) const MEM_SIZE_NOTHING: u32 = 0x0000_0003;
	pub(crate) const MEM_SIZE_XMMWORD_PTR: u32 = 0x0000_0004;
	pub(crate) const MEM_SIZE_DWORD_OR_QWORD: u32 = 0x0000_0005;
	pub(crate) const SHOW_NO_MEM_SIZE_FORCE_SIZE: u32 = 0x0000_0008;
	pub(crate) const SHOW_MIN_MEM_SIZE_FORCE_SIZE: u32 = 0x0000_0010;
	pub(crate) const JCC_NOT_TAKEN: u32 = 0x0000_0020;
	pub(crate) const JCC_TAKEN: u32 = 0x0000_0040;
	pub(crate) const BND_PREFIX: u32 = 0x0000_0080;
	pub(crate) const IGNORE_INDEX_REG: u32 = 0x0000_0100;
	pub(crate) const MNEMONIC_IS_DIRECTIVE: u32 = 0x0000_0200;
}
// GENERATOR-END: InstrOpInfoFlags

// GENERATOR-BEGIN: InstrOpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum InstrOpKind {
	Register,
	NearBranch16,
	NearBranch32,
	NearBranch64,
	FarBranch16,
	FarBranch32,
	Immediate8,
	Immediate8_2nd,
	Immediate16,
	Immediate32,
	Immediate64,
	Immediate8to16,
	Immediate8to32,
	Immediate8to64,
	Immediate32to64,
	MemorySegSI,
	MemorySegESI,
	MemorySegRSI,
	MemorySegDI,
	MemorySegEDI,
	MemorySegRDI,
	MemoryESDI,
	MemoryESEDI,
	MemoryESRDI,
	Memory64,
	Memory,
	ExtraImmediate8_Value3,
	DeclareByte,
	DeclareWord,
	DeclareDword,
	DeclareQword,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_INSTR_OP_KIND: [&str; 31] = [
	"Register",
	"NearBranch16",
	"NearBranch32",
	"NearBranch64",
	"FarBranch16",
	"FarBranch32",
	"Immediate8",
	"Immediate8_2nd",
	"Immediate16",
	"Immediate32",
	"Immediate64",
	"Immediate8to16",
	"Immediate8to32",
	"Immediate8to64",
	"Immediate32to64",
	"MemorySegSI",
	"MemorySegESI",
	"MemorySegRSI",
	"MemorySegDI",
	"MemorySegEDI",
	"MemorySegRDI",
	"MemoryESDI",
	"MemoryESEDI",
	"MemoryESRDI",
	"Memory64",
	"Memory",
	"ExtraImmediate8_Value3",
	"DeclareByte",
	"DeclareWord",
	"DeclareDword",
	"DeclareQword",
];
impl fmt::Debug for InstrOpKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_INSTR_OP_KIND[*self as usize])?;
		Ok(())
	}
}
impl Default for InstrOpKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		InstrOpKind::Register
	}
}
// GENERATOR-END: InstrOpKind