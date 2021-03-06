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

#if !NO_MASM_FORMATTER && !NO_FORMATTER
namespace Iced.Intel.MasmFormatterInternal {
	enum CtorKind {
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
}
#endif
