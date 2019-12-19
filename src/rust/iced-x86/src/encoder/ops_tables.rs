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

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#![cfg_attr(feature = "cargo-fmt", rustfmt::skip)]

use super::super::*;
use super::ops::*;

static NONE: InvalidOpHandler = InvalidOpHandler;
static OPR_DI: OprDI = OprDI;
static OP_A_2: OpA = OpA {
	size: 2
};
static OP_A_4: OpA = OpA {
	size: 4
};
static OP_HX_EAX_R15_D: OpHx = OpHx {
	reg_lo: Register::EAX,
	reg_hi: Register::R15D,
};
static OP_HX_K0_K7: OpHx = OpHx {
	reg_lo: Register::K0,
	reg_hi: Register::K7,
};
static OP_HX_RAX_R15: OpHx = OpHx {
	reg_lo: Register::RAX,
	reg_hi: Register::R15,
};
static OP_HX_XMM0_XMM15: OpHx = OpHx {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM15,
};
static OP_HX_XMM0_XMM31: OpHx = OpHx {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM31,
};
static OP_HX_YMM0_YMM15: OpHx = OpHx {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM15,
};
static OP_HX_YMM0_YMM31: OpHx = OpHx {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM31,
};
static OP_HX_ZMM0_ZMM31: OpHx = OpHx {
	reg_lo: Register::ZMM0,
	reg_hi: Register::ZMM31,
};
static OP_I2: OpI2 = OpI2;
static OP_IB11: OpIb11 = OpIb11;
static OP_IB21: OpIb21 = OpIb21;
static OP_IB_IMMEDIATE8: OpIb = OpIb {
	op_kind: OpKind::Immediate8,
};
static OP_IB_IMMEDIATE8TO16: OpIb = OpIb {
	op_kind: OpKind::Immediate8to16,
};
static OP_IB_IMMEDIATE8TO32: OpIb = OpIb {
	op_kind: OpKind::Immediate8to32,
};
static OP_IB_IMMEDIATE8TO64: OpIb = OpIb {
	op_kind: OpKind::Immediate8to64,
};
static OP_ID_IMMEDIATE32: OpId = OpId {
	op_kind: OpKind::Immediate32,
};
static OP_ID_IMMEDIATE32TO64: OpId = OpId {
	op_kind: OpKind::Immediate32to64,
};
static OP_IMM_1: OpImm = OpImm {
	value: 1
};
static OP_IQ: OpIq = OpIq;
static OP_IS4X_XMM0_XMM15: OpIs4x = OpIs4x {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM15,
};
static OP_IS4X_YMM0_YMM15: OpIs4x = OpIs4x {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM15,
};
static OP_IW: OpIw = OpIw;
static OP_JDISP_2: OpJdisp = OpJdisp {
	displ_size: 2
};
static OP_JDISP_4: OpJdisp = OpJdisp {
	displ_size: 4
};
static OP_JX_2: OpJx = OpJx {
	imm_size: 2
};
static OP_JX_4: OpJx = OpJx {
	imm_size: 4
};
static OP_J_NEAR_BRANCH16_1: OpJ = OpJ {
	op_kind: OpKind::NearBranch16,
	imm_size: 1
};
static OP_J_NEAR_BRANCH16_2: OpJ = OpJ {
	op_kind: OpKind::NearBranch16,
	imm_size: 2
};
static OP_J_NEAR_BRANCH32_1: OpJ = OpJ {
	op_kind: OpKind::NearBranch32,
	imm_size: 1
};
static OP_J_NEAR_BRANCH32_4: OpJ = OpJ {
	op_kind: OpKind::NearBranch32,
	imm_size: 4
};
static OP_J_NEAR_BRANCH64_1: OpJ = OpJ {
	op_kind: OpKind::NearBranch64,
	imm_size: 1
};
static OP_J_NEAR_BRANCH64_4: OpJ = OpJ {
	op_kind: OpKind::NearBranch64,
	imm_size: 4
};
static OP_MOD_RM_REG_AL_R15_L: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::AL,
	reg_hi: Register::R15L,
};
static OP_MOD_RM_REG_AX_R15_W: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::AX,
	reg_hi: Register::R15W,
};
static OP_MOD_RM_REG_BND0_BND3: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::BND0,
	reg_hi: Register::BND3,
};
static OP_MOD_RM_REG_CR0_CR15: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::CR0,
	reg_hi: Register::CR15,
};
static OP_MOD_RM_REG_DR0_DR15: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::DR0,
	reg_hi: Register::DR15,
};
static OP_MOD_RM_REG_EAX_R15_D: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::EAX,
	reg_hi: Register::R15D,
};
static OP_MOD_RM_REG_ES_GS: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::ES,
	reg_hi: Register::GS,
};
static OP_MOD_RM_REG_F0_CR0_CR15: OpModRM_regF0 = OpModRM_regF0 {
	reg_lo: Register::CR0,
	reg_hi: Register::CR15,
};
static OP_MOD_RM_REG_K0_K7: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::K0,
	reg_hi: Register::K7,
};
static OP_MOD_RM_REG_MM0_MM7: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::MM0,
	reg_hi: Register::MM7,
};
static OP_MOD_RM_REG_RAX_R15: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::RAX,
	reg_hi: Register::R15,
};
static OP_MOD_RM_REG_TR0_TR7: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::TR0,
	reg_hi: Register::TR7,
};
static OP_MOD_RM_REG_XMM0_XMM15: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM15,
};
static OP_MOD_RM_REG_XMM0_XMM31: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM31,
};
static OP_MOD_RM_REG_YMM0_YMM15: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM15,
};
static OP_MOD_RM_REG_YMM0_YMM31: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM31,
};
static OP_MOD_RM_REG_ZMM0_ZMM31: OpModRM_reg = OpModRM_reg {
	reg_lo: Register::ZMM0,
	reg_hi: Register::ZMM31,
};
static OP_MOD_RM_RM_AL_R15_L: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::AL,
	reg_hi: Register::R15L,
};
static OP_MOD_RM_RM_AX_R15_W: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::AX,
	reg_hi: Register::R15W,
};
static OP_MOD_RM_RM_BND0_BND3: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::BND0,
	reg_hi: Register::BND3,
};
static OP_MOD_RM_RM_EAX_R15_D: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::EAX,
	reg_hi: Register::R15D,
};
static OP_MOD_RM_RM_K0_K7: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::K0,
	reg_hi: Register::K7,
};
static OP_MOD_RM_RM_MEM_ONLY: OpModRM_rm_mem_only = OpModRM_rm_mem_only;
static OP_MOD_RM_RM_MM0_MM7: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::MM0,
	reg_hi: Register::MM7,
};
static OP_MOD_RM_RM_RAX_R15: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::RAX,
	reg_hi: Register::R15,
};
static OP_MOD_RM_RM_REG_ONLY_AX_R15_W: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::AX,
	reg_hi: Register::R15W,
};
static OP_MOD_RM_RM_REG_ONLY_EAX_R15_D: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::EAX,
	reg_hi: Register::R15D,
};
static OP_MOD_RM_RM_REG_ONLY_K0_K7: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::K0,
	reg_hi: Register::K7,
};
static OP_MOD_RM_RM_REG_ONLY_MM0_MM7: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::MM0,
	reg_hi: Register::MM7,
};
static OP_MOD_RM_RM_REG_ONLY_RAX_R15: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::RAX,
	reg_hi: Register::R15,
};
static OP_MOD_RM_RM_REG_ONLY_XMM0_XMM15: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM15,
};
static OP_MOD_RM_RM_REG_ONLY_XMM0_XMM31: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM31,
};
static OP_MOD_RM_RM_REG_ONLY_YMM0_YMM15: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM15,
};
static OP_MOD_RM_RM_REG_ONLY_YMM0_YMM31: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM31,
};
static OP_MOD_RM_RM_REG_ONLY_ZMM0_ZMM31: OpModRM_rm_reg_only = OpModRM_rm_reg_only {
	reg_lo: Register::ZMM0,
	reg_hi: Register::ZMM31,
};
static OP_MOD_RM_RM_XMM0_XMM15: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM15,
};
static OP_MOD_RM_RM_XMM0_XMM31: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::XMM0,
	reg_hi: Register::XMM31,
};
static OP_MOD_RM_RM_YMM0_YMM15: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM15,
};
static OP_MOD_RM_RM_YMM0_YMM31: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::YMM0,
	reg_hi: Register::YMM31,
};
static OP_MOD_RM_RM_ZMM0_ZMM31: OpModRM_rm = OpModRM_rm {
	reg_lo: Register::ZMM0,
	reg_hi: Register::ZMM31,
};
static OP_MRBX: OpMRBX = OpMRBX;
static OP_O: OpO = OpO;
static OP_REG_AL: OpReg = OpReg {
	register: Register::AL,
};
static OP_REG_AX: OpReg = OpReg {
	register: Register::AX,
};
static OP_REG_CL: OpReg = OpReg {
	register: Register::CL,
};
static OP_REG_CS: OpReg = OpReg {
	register: Register::CS,
};
static OP_REG_DS: OpReg = OpReg {
	register: Register::DS,
};
static OP_REG_DX: OpReg = OpReg {
	register: Register::DX,
};
static OP_REG_EAX: OpReg = OpReg {
	register: Register::EAX,
};
static OP_REG_EMBED8_AL_R15_L: OpRegEmbed8 = OpRegEmbed8 {
	reg_lo: Register::AL,
	reg_hi: Register::R15L,
};
static OP_REG_EMBED8_AX_R15_W: OpRegEmbed8 = OpRegEmbed8 {
	reg_lo: Register::AX,
	reg_hi: Register::R15W,
};
static OP_REG_EMBED8_EAX_R15_D: OpRegEmbed8 = OpRegEmbed8 {
	reg_lo: Register::EAX,
	reg_hi: Register::R15D,
};
static OP_REG_EMBED8_RAX_R15: OpRegEmbed8 = OpRegEmbed8 {
	reg_lo: Register::RAX,
	reg_hi: Register::R15,
};
static OP_REG_ES: OpReg = OpReg {
	register: Register::ES,
};
static OP_REG_FS: OpReg = OpReg {
	register: Register::FS,
};
static OP_REG_GS: OpReg = OpReg {
	register: Register::GS,
};
static OP_REG_RAX: OpReg = OpReg {
	register: Register::RAX,
};
static OP_REG_SS: OpReg = OpReg {
	register: Register::SS,
};
static OP_REG_ST0: OpReg = OpReg {
	register: Register::ST0,
};
static OP_REG_STI: OpRegSTi = OpRegSTi;
static OP_VMX_XMM0_XMM15: OpVMx = OpVMx {
	vsib_index_reg_lo: Register::XMM0,
	vsib_index_reg_hi: Register::XMM15,
};
static OP_VMX_XMM0_XMM31: OpVMx = OpVMx {
	vsib_index_reg_lo: Register::XMM0,
	vsib_index_reg_hi: Register::XMM31,
};
static OP_VMX_YMM0_YMM15: OpVMx = OpVMx {
	vsib_index_reg_lo: Register::YMM0,
	vsib_index_reg_hi: Register::YMM15,
};
static OP_VMX_YMM0_YMM31: OpVMx = OpVMx {
	vsib_index_reg_lo: Register::YMM0,
	vsib_index_reg_hi: Register::YMM31,
};
static OP_VMX_ZMM0_ZMM31: OpVMx = OpVMx {
	vsib_index_reg_lo: Register::ZMM0,
	vsib_index_reg_hi: Register::ZMM31,
};
static OP_X: OpX = OpX;
static OP_Y: OpY = OpY;

pub(crate) static LEGACY_TABLE: [&(Op + Sync); 118] = [
	&NONE,// None
	&OP_A_2,// Aww
	&OP_A_4,// Adw
	&OP_MOD_RM_RM_MEM_ONLY,// M
	&OP_MOD_RM_RM_MEM_ONLY,// Mfbcd
	&OP_MOD_RM_RM_MEM_ONLY,// Mf32
	&OP_MOD_RM_RM_MEM_ONLY,// Mf64
	&OP_MOD_RM_RM_MEM_ONLY,// Mf80
	&OP_MOD_RM_RM_MEM_ONLY,// Mfi16
	&OP_MOD_RM_RM_MEM_ONLY,// Mfi32
	&OP_MOD_RM_RM_MEM_ONLY,// Mfi64
	&OP_MOD_RM_RM_MEM_ONLY,// M14
	&OP_MOD_RM_RM_MEM_ONLY,// M28
	&OP_MOD_RM_RM_MEM_ONLY,// M98
	&OP_MOD_RM_RM_MEM_ONLY,// M108
	&OP_MOD_RM_RM_MEM_ONLY,// Mp
	&OP_MOD_RM_RM_MEM_ONLY,// Ms
	&OP_MOD_RM_RM_MEM_ONLY,// Mo
	&OP_MOD_RM_RM_MEM_ONLY,// Mb
	&OP_MOD_RM_RM_MEM_ONLY,// Mw
	&OP_MOD_RM_RM_MEM_ONLY,// Md
	&OP_MOD_RM_RM_MEM_ONLY,// Md_MPX
	&OP_MOD_RM_RM_MEM_ONLY,// Mq
	&OP_MOD_RM_RM_MEM_ONLY,// Mq_MPX
	&OP_MOD_RM_RM_MEM_ONLY,// Mw2
	&OP_MOD_RM_RM_MEM_ONLY,// Md2
	&OP_MOD_RM_RM_AL_R15_L,// Eb
	&OP_MOD_RM_RM_AX_R15_W,// Ew
	&OP_MOD_RM_RM_EAX_R15_D,// Ed
	&OP_MOD_RM_RM_EAX_R15_D,// Ed_MPX
	&OP_MOD_RM_RM_EAX_R15_D,// Ew_d
	&OP_MOD_RM_RM_RAX_R15,// Ew_q
	&OP_MOD_RM_RM_RAX_R15,// Eq
	&OP_MOD_RM_RM_RAX_R15,// Eq_MPX
	&OP_MOD_RM_RM_MEM_ONLY,// Eww
	&OP_MOD_RM_RM_MEM_ONLY,// Edw
	&OP_MOD_RM_RM_MEM_ONLY,// Eqw
	&OP_MOD_RM_RM_EAX_R15_D,// RdMb
	&OP_MOD_RM_RM_RAX_R15,// RqMb
	&OP_MOD_RM_RM_EAX_R15_D,// RdMw
	&OP_MOD_RM_RM_RAX_R15,// RqMw
	&OP_MOD_RM_REG_AL_R15_L,// Gb
	&OP_MOD_RM_REG_AX_R15_W,// Gw
	&OP_MOD_RM_REG_EAX_R15_D,// Gd
	&OP_MOD_RM_REG_RAX_R15,// Gq
	&OP_MOD_RM_RM_REG_ONLY_AX_R15_W,// Rw
	&OP_MOD_RM_RM_REG_ONLY_EAX_R15_D,// Rd
	&OP_MOD_RM_RM_REG_ONLY_RAX_R15,// Rq
	&OP_MOD_RM_REG_ES_GS,// Sw
	&OP_MOD_RM_REG_F0_CR0_CR15,// Cd
	&OP_MOD_RM_REG_CR0_CR15,// Cq
	&OP_MOD_RM_REG_DR0_DR15,// Dd
	&OP_MOD_RM_REG_DR0_DR15,// Dq
	&OP_MOD_RM_REG_TR0_TR7,// Td
	&OP_IB_IMMEDIATE8,// Ib
	&OP_IB_IMMEDIATE8TO16,// Ib16
	&OP_IB_IMMEDIATE8TO32,// Ib32
	&OP_IB_IMMEDIATE8TO64,// Ib64
	&OP_IW,// Iw
	&OP_ID_IMMEDIATE32,// Id
	&OP_ID_IMMEDIATE32TO64,// Id64
	&OP_IQ,// Iq
	&OP_IB21,// Ib21
	&OP_IB11,// Ib11
	&OP_X,// Xb
	&OP_X,// Xw
	&OP_X,// Xd
	&OP_X,// Xq
	&OP_Y,// Yb
	&OP_Y,// Yw
	&OP_Y,// Yd
	&OP_Y,// Yq
	&OP_J_NEAR_BRANCH16_1,// wJb
	&OP_J_NEAR_BRANCH32_1,// dJb
	&OP_J_NEAR_BRANCH64_1,// qJb
	&OP_J_NEAR_BRANCH16_2,// Jw
	&OP_J_NEAR_BRANCH32_4,// wJd
	&OP_J_NEAR_BRANCH32_4,// dJd
	&OP_J_NEAR_BRANCH64_4,// qJd
	&OP_JX_2,// Jxw
	&OP_JX_4,// Jxd
	&OP_JDISP_2,// Jdisp16
	&OP_JDISP_4,// Jdisp32
	&OP_O,// Ob
	&OP_O,// Ow
	&OP_O,// Od
	&OP_O,// Oq
	&OP_IMM_1,// Imm1
	&OP_MOD_RM_REG_BND0_BND3,// B
	&OP_MOD_RM_RM_BND0_BND3,// BMq
	&OP_MOD_RM_RM_BND0_BND3,// BMo
	&OP_MOD_RM_RM_MEM_ONLY,// MIB
	&OP_MOD_RM_RM_REG_ONLY_MM0_MM7,// N
	&OP_MOD_RM_REG_MM0_MM7,// P
	&OP_MOD_RM_RM_MM0_MM7,// Q
	&OP_MOD_RM_RM_REG_ONLY_XMM0_XMM15,// RX
	&OP_MOD_RM_REG_XMM0_XMM15,// VX
	&OP_MOD_RM_RM_XMM0_XMM15,// WX
	&OPR_DI,// rDI
	&OP_MRBX,// MRBX
	&OP_REG_ES,// ES
	&OP_REG_CS,// CS
	&OP_REG_SS,// SS
	&OP_REG_DS,// DS
	&OP_REG_FS,// FS
	&OP_REG_GS,// GS
	&OP_REG_AL,// AL
	&OP_REG_CL,// CL
	&OP_REG_AX,// AX
	&OP_REG_DX,// DX
	&OP_REG_EAX,// EAX
	&OP_REG_RAX,// RAX
	&OP_REG_ST0,// ST
	&OP_REG_STI,// STi
	&OP_REG_EMBED8_AL_R15_L,// r8_rb
	&OP_REG_EMBED8_AX_R15_W,// r16_rw
	&OP_REG_EMBED8_EAX_R15_D,// r32_rd
	&OP_REG_EMBED8_RAX_R15,// r64_ro
];
pub(crate) static VEX_TABLE: [&(Op + Sync); 39] = [
	&NONE,// None
	&OP_MOD_RM_RM_EAX_R15_D,// Ed
	&OP_MOD_RM_RM_RAX_R15,// Eq
	&OP_MOD_RM_REG_EAX_R15_D,// Gd
	&OP_MOD_RM_REG_RAX_R15,// Gq
	&OP_MOD_RM_RM_EAX_R15_D,// RdMb
	&OP_MOD_RM_RM_RAX_R15,// RqMb
	&OP_MOD_RM_RM_EAX_R15_D,// RdMw
	&OP_MOD_RM_RM_RAX_R15,// RqMw
	&OP_MOD_RM_RM_REG_ONLY_EAX_R15_D,// Rd
	&OP_MOD_RM_RM_REG_ONLY_RAX_R15,// Rq
	&OP_HX_EAX_R15_D,// Hd
	&OP_HX_RAX_R15,// Hq
	&OP_HX_K0_K7,// HK
	&OP_HX_XMM0_XMM15,// HX
	&OP_HX_YMM0_YMM15,// HY
	&OP_IB_IMMEDIATE8,// Ib
	&OP_I2,// I2
	&OP_IS4X_XMM0_XMM15,// Is4X
	&OP_IS4X_YMM0_YMM15,// Is4Y
	&OP_IS4X_XMM0_XMM15,// Is5X
	&OP_IS4X_YMM0_YMM15,// Is5Y
	&OP_MOD_RM_RM_MEM_ONLY,// M
	&OP_MOD_RM_RM_MEM_ONLY,// Md
	&OP_MOD_RM_RM_MEM_ONLY,// MK
	&OPR_DI,// rDI
	&OP_MOD_RM_RM_REG_ONLY_K0_K7,// RK
	&OP_MOD_RM_RM_REG_ONLY_XMM0_XMM15,// RX
	&OP_MOD_RM_RM_REG_ONLY_YMM0_YMM15,// RY
	&OP_MOD_RM_REG_K0_K7,// VK
	&OP_VMX_XMM0_XMM15,// VM32X
	&OP_VMX_YMM0_YMM15,// VM32Y
	&OP_VMX_XMM0_XMM15,// VM64X
	&OP_VMX_YMM0_YMM15,// VM64Y
	&OP_MOD_RM_REG_XMM0_XMM15,// VX
	&OP_MOD_RM_REG_YMM0_YMM15,// VY
	&OP_MOD_RM_RM_K0_K7,// WK
	&OP_MOD_RM_RM_XMM0_XMM15,// WX
	&OP_MOD_RM_RM_YMM0_YMM15,// WY
];
pub(crate) static XOP_TABLE: [&(Op + Sync); 19] = [
	&NONE,// None
	&OP_MOD_RM_RM_EAX_R15_D,// Ed
	&OP_MOD_RM_RM_RAX_R15,// Eq
	&OP_MOD_RM_REG_EAX_R15_D,// Gd
	&OP_MOD_RM_REG_RAX_R15,// Gq
	&OP_MOD_RM_RM_REG_ONLY_EAX_R15_D,// Rd
	&OP_MOD_RM_RM_REG_ONLY_RAX_R15,// Rq
	&OP_HX_EAX_R15_D,// Hd
	&OP_HX_RAX_R15,// Hq
	&OP_HX_XMM0_XMM15,// HX
	&OP_HX_YMM0_YMM15,// HY
	&OP_IB_IMMEDIATE8,// Ib
	&OP_ID_IMMEDIATE32,// Id
	&OP_IS4X_XMM0_XMM15,// Is4X
	&OP_IS4X_YMM0_YMM15,// Is4Y
	&OP_MOD_RM_REG_XMM0_XMM15,// VX
	&OP_MOD_RM_REG_YMM0_YMM15,// VY
	&OP_MOD_RM_RM_XMM0_XMM15,// WX
	&OP_MOD_RM_RM_YMM0_YMM15,// WY
];
pub(crate) static EVEX_TABLE: [&(Op + Sync); 36] = [
	&NONE,// None
	&OP_MOD_RM_RM_EAX_R15_D,// Ed
	&OP_MOD_RM_RM_RAX_R15,// Eq
	&OP_MOD_RM_REG_EAX_R15_D,// Gd
	&OP_MOD_RM_REG_RAX_R15,// Gq
	&OP_MOD_RM_RM_EAX_R15_D,// RdMb
	&OP_MOD_RM_RM_RAX_R15,// RqMb
	&OP_MOD_RM_RM_EAX_R15_D,// RdMw
	&OP_MOD_RM_RM_RAX_R15,// RqMw
	&OP_HX_XMM0_XMM31,// HX
	&OP_HX_YMM0_YMM31,// HY
	&OP_HX_ZMM0_ZMM31,// HZ
	&OP_HX_XMM0_XMM31,// HXP3
	&OP_HX_ZMM0_ZMM31,// HZP3
	&OP_IB_IMMEDIATE8,// Ib
	&OP_MOD_RM_RM_MEM_ONLY,// M
	&OP_MOD_RM_RM_REG_ONLY_EAX_R15_D,// Rd
	&OP_MOD_RM_RM_REG_ONLY_RAX_R15,// Rq
	&OP_MOD_RM_RM_REG_ONLY_XMM0_XMM31,// RX
	&OP_MOD_RM_RM_REG_ONLY_YMM0_YMM31,// RY
	&OP_MOD_RM_RM_REG_ONLY_ZMM0_ZMM31,// RZ
	&OP_MOD_RM_RM_REG_ONLY_K0_K7,// RK
	&OP_VMX_XMM0_XMM31,// VM32X
	&OP_VMX_YMM0_YMM31,// VM32Y
	&OP_VMX_ZMM0_ZMM31,// VM32Z
	&OP_VMX_XMM0_XMM31,// VM64X
	&OP_VMX_YMM0_YMM31,// VM64Y
	&OP_VMX_ZMM0_ZMM31,// VM64Z
	&OP_MOD_RM_REG_K0_K7,// VK
	&OP_MOD_RM_REG_K0_K7,// VKP1
	&OP_MOD_RM_REG_XMM0_XMM31,// VX
	&OP_MOD_RM_REG_YMM0_YMM31,// VY
	&OP_MOD_RM_REG_ZMM0_ZMM31,// VZ
	&OP_MOD_RM_RM_XMM0_XMM31,// WX
	&OP_MOD_RM_RM_YMM0_YMM31,// WY
	&OP_MOD_RM_RM_ZMM0_ZMM31,// WZ
];