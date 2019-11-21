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

#![allow(unused_results)]

use super::super::super::MemorySize;
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_MEMORY_SIZE_HASH: HashMap<&'static str, MemorySize> = {
		// GENERATOR-BEGIN: MemorySizeHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(136);
		h.insert("Unknown", MemorySize::Unknown);
		h.insert("UInt8", MemorySize::UInt8);
		h.insert("UInt16", MemorySize::UInt16);
		h.insert("UInt32", MemorySize::UInt32);
		h.insert("UInt52", MemorySize::UInt52);
		h.insert("UInt64", MemorySize::UInt64);
		h.insert("UInt128", MemorySize::UInt128);
		h.insert("UInt256", MemorySize::UInt256);
		h.insert("UInt512", MemorySize::UInt512);
		h.insert("Int8", MemorySize::Int8);
		h.insert("Int16", MemorySize::Int16);
		h.insert("Int32", MemorySize::Int32);
		h.insert("Int64", MemorySize::Int64);
		h.insert("Int128", MemorySize::Int128);
		h.insert("Int256", MemorySize::Int256);
		h.insert("Int512", MemorySize::Int512);
		h.insert("SegPtr16", MemorySize::SegPtr16);
		h.insert("SegPtr32", MemorySize::SegPtr32);
		h.insert("SegPtr64", MemorySize::SegPtr64);
		h.insert("WordOffset", MemorySize::WordOffset);
		h.insert("DwordOffset", MemorySize::DwordOffset);
		h.insert("QwordOffset", MemorySize::QwordOffset);
		h.insert("Bound16_WordWord", MemorySize::Bound16_WordWord);
		h.insert("Bound32_DwordDword", MemorySize::Bound32_DwordDword);
		h.insert("Bnd32", MemorySize::Bnd32);
		h.insert("Bnd64", MemorySize::Bnd64);
		h.insert("Fword6", MemorySize::Fword6);
		h.insert("Fword10", MemorySize::Fword10);
		h.insert("Float16", MemorySize::Float16);
		h.insert("Float32", MemorySize::Float32);
		h.insert("Float64", MemorySize::Float64);
		h.insert("Float80", MemorySize::Float80);
		h.insert("Float128", MemorySize::Float128);
		h.insert("BFloat16", MemorySize::BFloat16);
		h.insert("FpuEnv14", MemorySize::FpuEnv14);
		h.insert("FpuEnv28", MemorySize::FpuEnv28);
		h.insert("FpuState94", MemorySize::FpuState94);
		h.insert("FpuState108", MemorySize::FpuState108);
		h.insert("Fxsave_512Byte", MemorySize::Fxsave_512Byte);
		h.insert("Fxsave64_512Byte", MemorySize::Fxsave64_512Byte);
		h.insert("Xsave", MemorySize::Xsave);
		h.insert("Xsave64", MemorySize::Xsave64);
		h.insert("Bcd", MemorySize::Bcd);
		h.insert("Packed16_UInt8", MemorySize::Packed16_UInt8);
		h.insert("Packed16_Int8", MemorySize::Packed16_Int8);
		h.insert("Packed32_UInt8", MemorySize::Packed32_UInt8);
		h.insert("Packed32_Int8", MemorySize::Packed32_Int8);
		h.insert("Packed32_UInt16", MemorySize::Packed32_UInt16);
		h.insert("Packed32_Int16", MemorySize::Packed32_Int16);
		h.insert("Packed32_BFloat16", MemorySize::Packed32_BFloat16);
		h.insert("Packed64_UInt8", MemorySize::Packed64_UInt8);
		h.insert("Packed64_Int8", MemorySize::Packed64_Int8);
		h.insert("Packed64_UInt16", MemorySize::Packed64_UInt16);
		h.insert("Packed64_Int16", MemorySize::Packed64_Int16);
		h.insert("Packed64_UInt32", MemorySize::Packed64_UInt32);
		h.insert("Packed64_Int32", MemorySize::Packed64_Int32);
		h.insert("Packed64_Float16", MemorySize::Packed64_Float16);
		h.insert("Packed64_Float32", MemorySize::Packed64_Float32);
		h.insert("Packed128_UInt8", MemorySize::Packed128_UInt8);
		h.insert("Packed128_Int8", MemorySize::Packed128_Int8);
		h.insert("Packed128_UInt16", MemorySize::Packed128_UInt16);
		h.insert("Packed128_Int16", MemorySize::Packed128_Int16);
		h.insert("Packed128_UInt32", MemorySize::Packed128_UInt32);
		h.insert("Packed128_Int32", MemorySize::Packed128_Int32);
		h.insert("Packed128_UInt52", MemorySize::Packed128_UInt52);
		h.insert("Packed128_UInt64", MemorySize::Packed128_UInt64);
		h.insert("Packed128_Int64", MemorySize::Packed128_Int64);
		h.insert("Packed128_Float16", MemorySize::Packed128_Float16);
		h.insert("Packed128_Float32", MemorySize::Packed128_Float32);
		h.insert("Packed128_Float64", MemorySize::Packed128_Float64);
		h.insert("Packed128_2xBFloat16", MemorySize::Packed128_2xBFloat16);
		h.insert("Packed256_UInt8", MemorySize::Packed256_UInt8);
		h.insert("Packed256_Int8", MemorySize::Packed256_Int8);
		h.insert("Packed256_UInt16", MemorySize::Packed256_UInt16);
		h.insert("Packed256_Int16", MemorySize::Packed256_Int16);
		h.insert("Packed256_UInt32", MemorySize::Packed256_UInt32);
		h.insert("Packed256_Int32", MemorySize::Packed256_Int32);
		h.insert("Packed256_UInt52", MemorySize::Packed256_UInt52);
		h.insert("Packed256_UInt64", MemorySize::Packed256_UInt64);
		h.insert("Packed256_Int64", MemorySize::Packed256_Int64);
		h.insert("Packed256_UInt128", MemorySize::Packed256_UInt128);
		h.insert("Packed256_Int128", MemorySize::Packed256_Int128);
		h.insert("Packed256_Float16", MemorySize::Packed256_Float16);
		h.insert("Packed256_Float32", MemorySize::Packed256_Float32);
		h.insert("Packed256_Float64", MemorySize::Packed256_Float64);
		h.insert("Packed256_Float128", MemorySize::Packed256_Float128);
		h.insert("Packed256_2xBFloat16", MemorySize::Packed256_2xBFloat16);
		h.insert("Packed512_UInt8", MemorySize::Packed512_UInt8);
		h.insert("Packed512_Int8", MemorySize::Packed512_Int8);
		h.insert("Packed512_UInt16", MemorySize::Packed512_UInt16);
		h.insert("Packed512_Int16", MemorySize::Packed512_Int16);
		h.insert("Packed512_UInt32", MemorySize::Packed512_UInt32);
		h.insert("Packed512_Int32", MemorySize::Packed512_Int32);
		h.insert("Packed512_UInt52", MemorySize::Packed512_UInt52);
		h.insert("Packed512_UInt64", MemorySize::Packed512_UInt64);
		h.insert("Packed512_Int64", MemorySize::Packed512_Int64);
		h.insert("Packed512_UInt128", MemorySize::Packed512_UInt128);
		h.insert("Packed512_Float32", MemorySize::Packed512_Float32);
		h.insert("Packed512_Float64", MemorySize::Packed512_Float64);
		h.insert("Packed512_2xBFloat16", MemorySize::Packed512_2xBFloat16);
		h.insert("Broadcast64_UInt32", MemorySize::Broadcast64_UInt32);
		h.insert("Broadcast64_Int32", MemorySize::Broadcast64_Int32);
		h.insert("Broadcast64_Float32", MemorySize::Broadcast64_Float32);
		h.insert("Broadcast128_UInt32", MemorySize::Broadcast128_UInt32);
		h.insert("Broadcast128_Int32", MemorySize::Broadcast128_Int32);
		h.insert("Broadcast128_UInt52", MemorySize::Broadcast128_UInt52);
		h.insert("Broadcast128_UInt64", MemorySize::Broadcast128_UInt64);
		h.insert("Broadcast128_Int64", MemorySize::Broadcast128_Int64);
		h.insert("Broadcast128_Float32", MemorySize::Broadcast128_Float32);
		h.insert("Broadcast128_Float64", MemorySize::Broadcast128_Float64);
		h.insert("Broadcast256_UInt32", MemorySize::Broadcast256_UInt32);
		h.insert("Broadcast256_Int32", MemorySize::Broadcast256_Int32);
		h.insert("Broadcast256_UInt52", MemorySize::Broadcast256_UInt52);
		h.insert("Broadcast256_UInt64", MemorySize::Broadcast256_UInt64);
		h.insert("Broadcast256_Int64", MemorySize::Broadcast256_Int64);
		h.insert("Broadcast256_Float32", MemorySize::Broadcast256_Float32);
		h.insert("Broadcast256_Float64", MemorySize::Broadcast256_Float64);
		h.insert("Broadcast512_UInt32", MemorySize::Broadcast512_UInt32);
		h.insert("Broadcast512_Int32", MemorySize::Broadcast512_Int32);
		h.insert("Broadcast512_UInt52", MemorySize::Broadcast512_UInt52);
		h.insert("Broadcast512_UInt64", MemorySize::Broadcast512_UInt64);
		h.insert("Broadcast512_Int64", MemorySize::Broadcast512_Int64);
		h.insert("Broadcast512_Float32", MemorySize::Broadcast512_Float32);
		h.insert("Broadcast512_Float64", MemorySize::Broadcast512_Float64);
		h.insert("Broadcast128_2xInt16", MemorySize::Broadcast128_2xInt16);
		h.insert("Broadcast256_2xInt16", MemorySize::Broadcast256_2xInt16);
		h.insert("Broadcast512_2xInt16", MemorySize::Broadcast512_2xInt16);
		h.insert("Broadcast128_2xUInt32", MemorySize::Broadcast128_2xUInt32);
		h.insert("Broadcast256_2xUInt32", MemorySize::Broadcast256_2xUInt32);
		h.insert("Broadcast512_2xUInt32", MemorySize::Broadcast512_2xUInt32);
		h.insert("Broadcast128_2xInt32", MemorySize::Broadcast128_2xInt32);
		h.insert("Broadcast256_2xInt32", MemorySize::Broadcast256_2xInt32);
		h.insert("Broadcast512_2xInt32", MemorySize::Broadcast512_2xInt32);
		h.insert("Broadcast128_2xBFloat16", MemorySize::Broadcast128_2xBFloat16);
		h.insert("Broadcast256_2xBFloat16", MemorySize::Broadcast256_2xBFloat16);
		h.insert("Broadcast512_2xBFloat16", MemorySize::Broadcast512_2xBFloat16);
		// GENERATOR-END: MemorySizeHash
		h
	};
}
