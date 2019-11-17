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

using System.Linq;

namespace Generator.Enums {
	static class EvexOpCodeHandlerKindEnum {
		const string? documentation = null;

		public enum Enum : byte {
			Invalid,
			Invalid2,
			Dup,
			HandlerReference,
			ArrayReference,
			RM,
			Group,
			W,
			MandatoryPrefix2,
			VectorLength,
			VectorLength_er,
			Ed_V_Ib,
			Ev_VX,
			Ev_VX_Ib,
			Gv_W_er,
			GvM_VX_Ib,
			HkWIb_3,
			HkWIb_3b,
			HWIb,
			KkHW_3,
			KkHW_3b,
			KkHWIb_sae_3,
			KkHWIb_sae_3b,
			KkHWIb_3,
			KkHWIb_3b,
			KkWIb_3,
			KkWIb_3b,
			KP1HW,
			KR,
			MV,
			V_H_Ev_er,
			V_H_Ev_Ib,
			VHM,
			VHW_3,
			VHW_4,
			VHWIb,
			VK,
			Vk_VSIB,
			VkEv_REXW_2,
			VkEv_REXW_3,
			VkHM,
			VkHW_3,
			VkHW_3b,
			VkHW_5,
			VkHW_er_4,
			VkHW_er_4b,
			VkHWIb_3,
			VkHWIb_3b,
			VkHWIb_5,
			VkHWIb_er_4,
			VkHWIb_er_4b,
			VkM,
			VkW_3,
			VkW_3b,
			VkW_4,
			VkW_4b,
			VkW_er_4,
			VkW_er_5,
			VkW_er_6,
			VkWIb_3,
			VkWIb_3b,
			VkWIb_er,
			VM,
			VSIB_k1,
			VSIB_k1_VX,
			VW,
			VW_er,
			VX_Ev,
			WkHV,
			WkV_3,
			WkV_4a,
			WkV_4b,
			WkVIb,
			WkVIb_er,
			WV,
		}

		static EnumValue[] GetValues() =>
			typeof(Enum).GetFields().Where(a => a.IsLiteral).Select(a => new EnumValue((uint)(Enum)a.GetValue(null)!, a.Name)).ToArray();

		public static readonly EnumType Instance = new EnumType(TypeIds.EvexOpCodeHandlerKind, documentation, GetValues(), EnumTypeFlags.None);
	}
}