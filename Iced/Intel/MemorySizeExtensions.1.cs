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

#if !NO_INSTR_INFO
using System.Diagnostics;

namespace Iced.Intel {
	/// <summary>
	/// <see cref="MemorySize"/> extension methods
	/// </summary>
	public static partial class MemorySizeExtensions {
		internal static readonly MemorySizeInfo[] MemorySizeInfos = GetMemorySizeInfos();
		enum SizeKind {
			S0,
			S1,
			S2,
			S4,
			S6,
			S8,
			S10,
			S14,
			S16,
			S28,
			S32,
			S64,
			S94,
			S108,
			S512,
		}
		static MemorySizeInfo[] GetMemorySizeInfos() {
			var data = new byte[DecoderConstants.NumberOfMemorySizes * 3] {
				(byte)MemorySize.Unknown, (byte)((uint)SizeKind.S0 | ((uint)SizeKind.S0 << 4)), 0,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S1 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt128, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S16 << 4)), 0,
				(byte)MemorySize.UInt256, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S32 << 4)), 0,
				(byte)MemorySize.UInt512, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S64 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S1 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Int128, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S16 << 4)), 1,
				(byte)MemorySize.Int256, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S32 << 4)), 1,
				(byte)MemorySize.Int512, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S64 << 4)), 1,
				(byte)MemorySize.SegPtr16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.SegPtr32, (byte)((uint)SizeKind.S6 | ((uint)SizeKind.S6 << 4)), 0,
				(byte)MemorySize.SegPtr64, (byte)((uint)SizeKind.S10 | ((uint)SizeKind.S10 << 4)), 0,
				(byte)MemorySize.WordOffset, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.DwordOffset, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.QwordOffset, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Bound16_WordWord, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Bound32_DwordDword, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Bnd32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Bnd64, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S16 << 4)), 0,
				(byte)MemorySize.Fword6, (byte)((uint)SizeKind.S6 | ((uint)SizeKind.S6 << 4)), 0,
				(byte)MemorySize.Fword10, (byte)((uint)SizeKind.S10 | ((uint)SizeKind.S10 << 4)), 0,
				(byte)MemorySize.Float16, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float80, (byte)((uint)SizeKind.S10 | ((uint)SizeKind.S10 << 4)), 1,
				(byte)MemorySize.Float128, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S16 << 4)), 1,
				(byte)MemorySize.BFloat16, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.FpuEnv14, (byte)((uint)SizeKind.S14 | ((uint)SizeKind.S14 << 4)), 0,
				(byte)MemorySize.FpuEnv28, (byte)((uint)SizeKind.S28 | ((uint)SizeKind.S28 << 4)), 0,
				(byte)MemorySize.FpuState94, (byte)((uint)SizeKind.S94 | ((uint)SizeKind.S94 << 4)), 0,
				(byte)MemorySize.FpuState108, (byte)((uint)SizeKind.S108 | ((uint)SizeKind.S108 << 4)), 0,
				(byte)MemorySize.Fxsave_512Byte, (byte)((uint)SizeKind.S512 | ((uint)SizeKind.S512 << 4)), 0,
				(byte)MemorySize.Fxsave64_512Byte, (byte)((uint)SizeKind.S512 | ((uint)SizeKind.S512 << 4)), 0,
				(byte)MemorySize.Xsave, (byte)((uint)SizeKind.S0 | ((uint)SizeKind.S0 << 4)), 0,
				(byte)MemorySize.Xsave64, (byte)((uint)SizeKind.S0 | ((uint)SizeKind.S0 << 4)), 0,
				(byte)MemorySize.Bcd, (byte)((uint)SizeKind.S10 | ((uint)SizeKind.S10 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S2 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.BFloat16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float16, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float16, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Packed32_BFloat16, (byte)((uint)SizeKind.S16 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.UInt128, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S16 << 4)), 0,
				(byte)MemorySize.Int128, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S16 << 4)), 1,
				(byte)MemorySize.Float16, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float128, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S16 << 4)), 1,
				(byte)MemorySize.Packed32_BFloat16, (byte)((uint)SizeKind.S32 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt8, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S1 << 4)), 0,
				(byte)MemorySize.Int8, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S1 << 4)), 1,
				(byte)MemorySize.UInt16, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S2 << 4)), 0,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.UInt128, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S16 << 4)), 0,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Packed32_BFloat16, (byte)((uint)SizeKind.S64 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.UInt52, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.UInt64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 0,
				(byte)MemorySize.Int64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Float32, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Float64, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S8 << 4)), 1,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.Int16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.UInt32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 0,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.Int32, (byte)((uint)SizeKind.S8 | ((uint)SizeKind.S4 << 4)), 1,
				(byte)MemorySize.BFloat16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.BFloat16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
				(byte)MemorySize.BFloat16, (byte)((uint)SizeKind.S4 | ((uint)SizeKind.S2 << 4)), 1,
			};

			var sizes = new ushort[] {
				0,
				1,
				2,
				4,
				6,
				8,
				10,
				14,
				16,
				28,
				32,
				64,
				94,
				108,
				512,
			};

			var infos = new MemorySizeInfo[DecoderConstants.NumberOfMemorySizes];
			for (int i = 0, j = 0; i < infos.Length; i++, j += 3) {
				var elementType = (MemorySize)data[j];
				var b = data[j + 1];
				var size = sizes[b & 0xF];
				var elementSize = sizes[b >> 4];
				infos[i] = new MemorySizeInfo((MemorySize)i, size, elementSize, elementType, data[j + 2] != 0, i >= (int)MemorySize.Broadcast64_UInt32);
			}
			return infos;
		}

		/// <summary>
		/// Gets the memory size info
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static MemorySizeInfo GetInfo(this MemorySize memorySize) {
			var infos = MemorySizeInfos;
			if ((uint)memorySize >= (uint)infos.Length)
				ThrowHelper.ThrowArgumentOutOfRangeException_memorySize();
			return infos[(int)memorySize];
		}

		/// <summary>
		/// Gets the size in bytes of the memory location or 0 if it's not accessed by the instruction or unknown or variable sized
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static int GetSize(this MemorySize memorySize) => memorySize.GetInfo().Size;

		/// <summary>
		/// Gets the size in bytes of the packed element. If it's not a packed data type, it's equal to <see cref="GetSize(MemorySize)"/>.
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static int GetElementSize(this MemorySize memorySize) => memorySize.GetInfo().ElementSize;

		/// <summary>
		/// Gets the element type if it's packed data or <paramref name="memorySize"/> if it's not packed data
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static MemorySize GetElementType(this MemorySize memorySize) => memorySize.GetInfo().ElementType;

		/// <summary>
		/// true if it's signed data (signed integer or a floating point value)
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static bool IsSigned(this MemorySize memorySize) => memorySize.GetInfo().IsSigned;

		/// <summary>
		/// true if this is a packed data type, eg. <see cref="MemorySize.Packed128_Float32"/>
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static bool IsPacked(this MemorySize memorySize) => memorySize.GetInfo().IsPacked;

		/// <summary>
		/// Gets the number of elements in the packed data type or 1 if it's not packed data (<see cref="IsPacked"/>)
		/// </summary>
		/// <param name="memorySize">Memory size</param>
		/// <returns></returns>
		public static int GetElementCount(this MemorySize memorySize) => memorySize.GetInfo().ElementCount;
	}

	/// <summary>
	/// <see cref="Intel.MemorySize"/> information
	/// </summary>
	public readonly struct MemorySizeInfo {
		// 8 bytes in size
		readonly ushort size;
		readonly ushort elementSize;
		readonly byte memorySize;
		readonly byte elementType;
		// Use flags if more booleans are needed
		readonly bool isSigned;
		readonly bool isBroadcast;

		/// <summary>
		/// Gets the <see cref="Intel.MemorySize"/> value
		/// </summary>
		public MemorySize MemorySize => (MemorySize)memorySize;

		/// <summary>
		/// Size in bytes of the memory location or 0 if it's not accessed or unknown
		/// </summary>
		public int Size => size;

		/// <summary>
		/// Size in bytes of the packed element. If it's not a packed data type, it's equal to <see cref="Size"/>.
		/// </summary>
		public int ElementSize => elementSize;

		/// <summary>
		/// Element type if it's packed data or the type itself if it's not packed data
		/// </summary>
		public MemorySize ElementType => (MemorySize)elementType;

		/// <summary>
		/// true if it's signed data (signed integer or a floating point value)
		/// </summary>
		public bool IsSigned => isSigned;

		/// <summary>
		/// true if it's a broadcast memory type
		/// </summary>
		public bool IsBroadcast => isBroadcast;

		/// <summary>
		/// true if this is a packed data type, eg. <see cref="MemorySize.Packed128_Float32"/>. See also <see cref="ElementCount"/>
		/// </summary>
		public bool IsPacked => elementSize < size;

		/// <summary>
		/// Gets the number of elements in the packed data type or 1 if it's not packed data (<see cref="IsPacked"/>)
		/// </summary>
		public int ElementCount => elementSize == size ? 1 : size / elementSize;// ElementSize can be 0 so we don't divide by it if es == s

		/// <summary>
		/// Constructor
		/// </summary>
		/// <param name="memorySize">Memory size value</param>
		/// <param name="size">Size of location</param>
		/// <param name="elementSize">Size of the packed element, or <paramref name="size"/> if it's not packed data</param>
		/// <param name="elementType">Element type if it's packed data or <paramref name="memorySize"/> if it's not packed data</param>
		/// <param name="isSigned">true if signed data</param>
		/// <param name="isBroadcast">true if broadcast</param>
		public MemorySizeInfo(MemorySize memorySize, int size, int elementSize, MemorySize elementType, bool isSigned, bool isBroadcast) {
			if (size < 0)
				ThrowHelper.ThrowArgumentOutOfRangeException_size();
			if (elementSize < 0)
				ThrowHelper.ThrowArgumentOutOfRangeException_elementSize();
			if (elementSize > size)
				ThrowHelper.ThrowArgumentOutOfRangeException_elementSize();
			Debug.Assert(DecoderConstants.NumberOfMemorySizes <= byte.MaxValue + 1);
			this.memorySize = (byte)memorySize;
			Debug.Assert(size <= ushort.MaxValue);
			this.size = (ushort)size;
			Debug.Assert(elementSize <= ushort.MaxValue);
			this.elementSize = (ushort)elementSize;
			Debug.Assert(DecoderConstants.NumberOfMemorySizes <= byte.MaxValue + 1);
			this.elementType = (byte)elementType;
			this.isSigned = isSigned;
			this.isBroadcast = isBroadcast;
		}
	}
}
#endif
