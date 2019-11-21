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

#![allow(dead_code)] //TODO: REMOVE

// GENERATOR-BEGIN: CodeSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// The code size (16/32/64) that was used when an instruction was decoded
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CodeSize {
	/// Unknown size
	Unknown,
	/// 16-bit code
	Code16,
	/// 32-bit code
	Code32,
	/// 64-bit code
	Code64,
}
// GENERATOR-END: CodeSize

// GENERATOR-BEGIN: RoundingControl
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Rounding control
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RoundingControl {
	/// No rounding mode
	None = 0,
	/// Round to nearest (even)
	RoundToNearest = 1,
	/// Round down (toward -inf)
	RoundDown = 2,
	/// Round up (toward +inf)
	RoundUp = 3,
	/// Round toward zero (truncate)
	RoundTowardZero = 4,
}
// GENERATOR-END: RoundingControl

// GENERATOR-BEGIN: OpKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Instruction operand kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum OpKind {
	/// A register (`Register`).
	///
	/// This operand kind uses `Instruction::op0_register()`, `Instruction::op1_register()`, `Instruction::op2_register()`, `Instruction::op3_register()` or `Instruction::op4_register()` depending on operand number. See also `Instruction::op_register()`.
	Register,
	/// Near 16-bit branch. This operand kind uses `Instruction::near_branch16()`
	NearBranch16,
	/// Near 32-bit branch. This operand kind uses `Instruction::near_branch32()`
	NearBranch32,
	/// Near 64-bit branch. This operand kind uses `Instruction::near_branch64()`
	NearBranch64,
	/// Far 16-bit branch. This operand kind uses `Instruction::far_branch16()` and `Instruction::far_branch_selector()`
	FarBranch16,
	/// Far 32-bit branch. This operand kind uses `Instruction::far_branch32()` and `Instruction::far_branch_selector()`
	FarBranch32,
	/// 8-bit constant. This operand kind uses `Instruction::immediate8()`
	Immediate8,
	/// 8-bit constant used by the `enter`, `extrq`, `insertq` instructions. This operand kind uses `Instruction::immediate8_2nd()`
	Immediate8_2nd,
	/// 16-bit constant. This operand kind uses `Instruction::immediate16()`
	Immediate16,
	/// 32-bit constant. This operand kind uses `Instruction::immediate32()`
	Immediate32,
	/// 64-bit constant. This operand kind uses `Instruction::immediate64()`
	Immediate64,
	/// An 8-bit value sign extended to 16 bits. This operand kind uses `Instruction::immediate8to16()`
	Immediate8to16,
	/// An 8-bit value sign extended to 32 bits. This operand kind uses `Instruction::immediate8to32()`
	Immediate8to32,
	/// An 8-bit value sign extended to 64 bits. This operand kind uses `Instruction::immediate8to64()`
	Immediate8to64,
	/// A 32-bit value sign extended to 64 bits. This operand kind uses `Instruction::immediate32to64()`
	Immediate32to64,
	/// `seg:[si]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegSI,
	/// `seg:[esi]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegESI,
	/// `seg:[rsi]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegRSI,
	/// `seg:[di]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegDI,
	/// `seg:[edi]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegEDI,
	/// `seg:[rdi]`. This operand kind uses `Instruction::memory_size()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	MemorySegRDI,
	/// `es:[di]`. This operand kind uses `Instruction::memory_size()`
	MemoryESDI,
	/// `es:[edi]`. This operand kind uses `Instruction::memory_size()`
	MemoryESEDI,
	/// `es:[rdi]`. This operand kind uses `Instruction::memory_size()`
	MemoryESRDI,
	/// 64-bit offset `[xxxxxxxxxxxxxxxx]`. This operand kind uses `Instruction::memory_address64()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`, `Instruction::memory_size()`
	Memory64,
	/// Memory operand.
	///
	/// This operand kind uses `Instruction::memory_displ_size()`, `Instruction::memory_size()`, `Instruction::memory_index_scale()`, `Instruction::memory_displacement()`, `Instruction::memory_base()`, `Instruction::memory_index()`, `Instruction::memory_segment()`, `Instruction::segment_prefix()`
	Memory,
}
// GENERATOR-END: OpKind

// GENERATOR-BEGIN: VectorLength
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
#[cfg(any(feature = "DECODER", feature = "ENCODER"))]
pub(crate) enum VectorLength {
	L128,
	L256,
	L512,
	Unknown,
}
// GENERATOR-END: VectorLength

impl Default for VectorLength {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		VectorLength::L128
	}
}

// GENERATOR-BEGIN: MandatoryPrefixByte
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(any(feature = "DECODER", feature = "ENCODER"))]
pub(crate) enum MandatoryPrefixByte {
	None,
	P66,
	PF3,
	PF2,
}
// GENERATOR-END: MandatoryPrefixByte

impl Default for MandatoryPrefixByte {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		MandatoryPrefixByte::None
	}
}

// GENERATOR-BEGIN: EncodingKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Instruction encoding
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(any(feature = "DECODER", feature = "ENCODER", feature = "INSTR_INFO"))]
pub enum EncodingKind {
	/// Legacy encoding
	Legacy,
	/// VEX encoding
	VEX,
	/// EVEX encoding
	EVEX,
	/// XOP encoding
	XOP,
	/// 3DNow! encoding
	D3NOW,
}
// GENERATOR-END: EncodingKind

// GENERATOR-BEGIN: TupleType
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Tuple type (EVEX) which can be used to get the disp8 scale factor `N`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg(any(feature = "DECODER", feature = "ENCODER"))]
#[allow(non_camel_case_types)]
pub enum TupleType {
	/// `N = 1`
	None,
	/// `N = b ? (W ? 8 : 4) : 16`
	Full_128,
	/// `N = b ? (W ? 8 : 4) : 32`
	Full_256,
	/// `N = b ? (W ? 8 : 4) : 64`
	Full_512,
	/// `N = b ? 4 : 8`
	Half_128,
	/// `N = b ? 4 : 16`
	Half_256,
	/// `N = b ? 4 : 32`
	Half_512,
	/// `N = 16`
	Full_Mem_128,
	/// `N = 32`
	Full_Mem_256,
	/// `N = 64`
	Full_Mem_512,
	/// `N = W ? 8 : 4`
	Tuple1_Scalar,
	/// `N = 1`
	Tuple1_Scalar_1,
	/// `N = 2`
	Tuple1_Scalar_2,
	/// `N = 4`
	Tuple1_Scalar_4,
	/// `N = 8`
	Tuple1_Scalar_8,
	/// `N = 4`
	Tuple1_Fixed_4,
	/// `N = 8`
	Tuple1_Fixed_8,
	/// `N = W ? 16 : 8`
	Tuple2,
	/// `N = W ? 32 : 16`
	Tuple4,
	/// `N = W ? error : 32`
	Tuple8,
	/// `N = 16`
	Tuple1_4X,
	/// `N = 8`
	Half_Mem_128,
	/// `N = 16`
	Half_Mem_256,
	/// `N = 32`
	Half_Mem_512,
	/// `N = 4`
	Quarter_Mem_128,
	/// `N = 8`
	Quarter_Mem_256,
	/// `N = 16`
	Quarter_Mem_512,
	/// `N = 2`
	Eighth_Mem_128,
	/// `N = 4`
	Eighth_Mem_256,
	/// `N = 8`
	Eighth_Mem_512,
	/// `N = 16`
	Mem128,
	/// `N = 8`
	MOVDDUP_128,
	/// `N = 32`
	MOVDDUP_256,
	/// `N = 64`
	MOVDDUP_512,
}
// GENERATOR-END: TupleType

// GENERATOR-BEGIN: FlowControl
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Flow control
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "INSTR_INFO")]
pub enum FlowControl {
	/// The next instruction that will be executed is the next instruction in the instruction stream
	Next,
	/// It's an unconditional branch instruction: `jmp near`, `jmp far`
	UnconditionalBranch,
	/// It's an unconditional indirect branch: `jmp near reg`, `jmp near [mem]`, `jmp far [mem]`
	IndirectBranch,
	/// It's a conditional branch instruction: `jcc short`, `jcc near`, `loop`, `loopcc`, `jrcxz`
	ConditionalBranch,
	/// It's a return instruction: `ret near`, `ret far`, `iret`, `sysret`, `sysexit`, `rsm`, `vmlaunch`, `vmresume`, `vmrun`, `skinit`
	Return,
	/// It's a call instruction: `call near`, `call far`, `syscall`, `sysenter`, `vmcall`, `vmmcall`
	Call,
	/// It's an indirect call instruction: `call near reg`, `call near [mem]`, `call far [mem]`
	IndirectCall,
	/// It's an interrupt instruction: `int n`, `int3`, `int1`, `into`
	Interrupt,
	/// It's `xbegin`, `xabort` or `xend`
	XbeginXabortXend,
	/// It's an invalid instruction, eg. `Code::INVALID`, `ud0`, `ud1`, `ud2`
	Exception,
}
// GENERATOR-END: FlowControl

// GENERATOR-BEGIN: OpCodeOperandKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Operand kind
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "ENCODER")]
#[allow(non_camel_case_types)]
pub enum OpCodeOperandKind {
	/// No operand
	None,
	/// Far branch 16-bit offset, 16-bit segment/selector
	farbr2_2,
	/// Far branch 32-bit offset, 16-bit segment/selector
	farbr4_2,
	/// Memory offset without a modrm byte (eg. `mov al,[offset]`)
	mem_offs,
	/// Memory (modrm)
	mem,
	/// Memory (modrm), MPX: 16/32-bit mode: must be 32-bit addressing, 64-bit mode: 64-bit addressing is forced
	mem_mpx,
	/// Memory (modrm), MPX: 16/32-bit mode: must be 32-bit addressing, 64-bit mode: 64-bit addressing is forced and must not be RIP relative
	mem_mib,
	/// Memory (modrm), vsib32, xmm registers
	mem_vsib32x,
	/// Memory (modrm), vsib64, xmm registers
	mem_vsib64x,
	/// Memory (modrm), vsib32, ymm registers
	mem_vsib32y,
	/// Memory (modrm), vsib64, ymm registers
	mem_vsib64y,
	/// Memory (modrm), vsib32, zmm registers
	mem_vsib32z,
	/// Memory (modrm), vsib64, zmm registers
	mem_vsib64z,
	/// 8-bit GPR or memory
	r8_or_mem,
	/// 16-bit GPR or memory
	r16_or_mem,
	/// 32-bit GPR or memory
	r32_or_mem,
	/// 32-bit GPR or memory, MPX: 16/32-bit mode: must be 32-bit addressing, 64-bit mode: 64-bit addressing is forced
	r32_or_mem_mpx,
	/// 64-bit GPR or memory
	r64_or_mem,
	/// 64-bit GPR or memory, MPX: 16/32-bit mode: must be 32-bit addressing, 64-bit mode: 64-bit addressing is forced
	r64_or_mem_mpx,
	/// MM register or memory
	mm_or_mem,
	/// XMM register or memory
	xmm_or_mem,
	/// YMM register or memory
	ymm_or_mem,
	/// ZMM register or memory
	zmm_or_mem,
	/// BND register or memory, MPX: 16/32-bit mode: must be 32-bit addressing, 64-bit mode: 64-bit addressing is forced
	bnd_or_mem_mpx,
	/// K register or memory
	k_or_mem,
	/// 8-bit GPR encoded in the `reg` field of the modrm byte
	r8_reg,
	/// 8-bit GPR encoded in the low 3 bits of the opcode
	r8_opcode,
	/// 16-bit GPR encoded in the `reg` field of the modrm byte
	r16_reg,
	/// 16-bit GPR encoded in the `mod + r/m` fields of the modrm byte
	r16_rm,
	/// 16-bit GPR encoded in the low 3 bits of the opcode
	r16_opcode,
	/// 32-bit GPR encoded in the `reg` field of the modrm byte
	r32_reg,
	/// 32-bit GPR encoded in the `mod + r/m` fields of the modrm byte
	r32_rm,
	/// 32-bit GPR encoded in the low 3 bits of the opcode
	r32_opcode,
	/// 32-bit GPR encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	r32_vvvv,
	/// 64-bit GPR encoded in the `reg` field of the modrm byte
	r64_reg,
	/// 64-bit GPR encoded in the `mod + r/m` fields of the modrm byte
	r64_rm,
	/// 64-bit GPR encoded in the low 3 bits of the opcode
	r64_opcode,
	/// 64-bit GPR encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	r64_vvvv,
	/// Segment register encoded in the `reg` field of the modrm byte
	seg_reg,
	/// K register encoded in the `reg` field of the modrm byte
	k_reg,
	/// K register (+1) encoded in the `reg` field of the modrm byte
	kp1_reg,
	/// K register encoded in the `mod + r/m` fields of the modrm byte
	k_rm,
	/// K register encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	k_vvvv,
	/// MM register encoded in the `reg` field of the modrm byte
	mm_reg,
	/// MM register encoded in the `mod + r/m` fields of the modrm byte
	mm_rm,
	/// XMM register encoded in the `reg` field of the modrm byte
	xmm_reg,
	/// XMM register encoded in the `mod + r/m` fields of the modrm byte
	xmm_rm,
	/// XMM register encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	xmm_vvvv,
	/// XMM register (+3) encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	xmmp3_vvvv,
	/// XMM register encoded in the the high 4 bits of the last 8-bit immediate (VEX/XOP only so only XMM0-XMM15)
	xmm_is4,
	/// XMM register encoded in the the high 4 bits of the last 8-bit immediate (VEX/XOP only so only XMM0-XMM15)
	xmm_is5,
	/// YMM register encoded in the `reg` field of the modrm byte
	ymm_reg,
	/// YMM register encoded in the `mod + r/m` fields of the modrm byte
	ymm_rm,
	/// YMM register encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	ymm_vvvv,
	/// YMM register encoded in the the high 4 bits of the last 8-bit immediate (VEX/XOP only so only YMM0-YMM15)
	ymm_is4,
	/// YMM register encoded in the the high 4 bits of the last 8-bit immediate (VEX/XOP only so only YMM0-YMM15)
	ymm_is5,
	/// ZMM register encoded in the `reg` field of the modrm byte
	zmm_reg,
	/// ZMM register encoded in the `mod + r/m` fields of the modrm byte
	zmm_rm,
	/// ZMM register encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	zmm_vvvv,
	/// ZMM register (+3) encoded in the the `V'vvvv` field (VEX/EVEX/XOP)
	zmmp3_vvvv,
	/// CR register encoded in the `reg` field of the modrm byte
	cr_reg,
	/// DR register encoded in the `reg` field of the modrm byte
	dr_reg,
	/// TR register encoded in the `reg` field of the modrm byte
	tr_reg,
	/// BND register encoded in the `reg` field of the modrm byte
	bnd_reg,
	/// ES register
	es,
	/// CS register
	cs,
	/// SS register
	ss,
	/// DS register
	ds,
	/// FS register
	fs,
	/// GS register
	gs,
	/// AL register
	al,
	/// CL register
	cl,
	/// AX register
	ax,
	/// DX register
	dx,
	/// EAX register
	eax,
	/// RAX register
	rax,
	/// ST0 register
	st0,
	/// ST(i) register encoded in the low 3 bits of the opcode
	sti_opcode,
	/// 2-bit immediate (m2z field, low 2 bits of the /is5 immediate, eg. `vpermil2ps`)
	imm2_m2z,
	/// 8-bit immediate
	imm8,
	/// Constant 1 (8-bit immediate)
	imm8_const_1,
	/// 8-bit immediate sign extended to 16 bits
	imm8sex16,
	/// 8-bit immediate sign extended to 32 bits
	imm8sex32,
	/// 8-bit immediate sign extended to 64 bits
	imm8sex64,
	/// 16-bit immediate
	imm16,
	/// 32-bit immediate
	imm32,
	/// 32-bit immediate sign extended to 64 bits
	imm32sex64,
	/// 64-bit immediate
	imm64,
	/// `seg:[rSI]` memory operand (string instructions)
	seg_rSI,
	/// `es:[rDI]` memory operand (string instructions)
	es_rDI,
	/// `seg:[rDI]` memory operand (`(v)maskmovq` instructions)
	seg_rDI,
	/// `seg:[rBX+al]` memory operand (`xlatb` instruction)
	seg_rBX_al,
	/// 16-bit branch, 1-byte signed relative offset
	br16_1,
	/// 32-bit branch, 1-byte signed relative offset
	br32_1,
	/// 64-bit branch, 1-byte signed relative offset
	br64_1,
	/// 16-bit branch, 2-byte signed relative offset
	br16_2,
	/// 32-bit branch, 4-byte signed relative offset
	br32_4,
	/// 64-bit branch, 4-byte signed relative offset
	br64_4,
	/// `xbegin`, 2-byte signed relative offset
	xbegin_2,
	/// `xbegin`, 4-byte signed relative offset
	xbegin_4,
	/// 2-byte branch offset (`jmpe` instruction)
	brdisp_2,
	/// 4-byte branch offset (`jmpe` instruction)
	brdisp_4,
}
// GENERATOR-END: OpCodeOperandKind

// GENERATOR-BEGIN: CpuidFeature
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// CPUID feature flags
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "INSTR_INFO")]
#[allow(non_camel_case_types)]
pub enum CpuidFeature {
	/// 8086 or later
	INTEL8086,
	/// 8086 only
	INTEL8086_ONLY,
	/// 80186 or later
	INTEL186,
	/// 80286 or later
	INTEL286,
	/// 80286 only
	INTEL286_ONLY,
	/// 80386 or later
	INTEL386,
	/// 80386 only
	INTEL386_ONLY,
	/// 80386 A0-B0 stepping only (xbts, ibts instructions)
	INTEL386_A0_ONLY,
	/// Intel486 or later
	INTEL486,
	/// Intel486 A stepping only (cmpxchg)
	INTEL486_A_ONLY,
	/// 80386 and Intel486 only
	INTEL386_486_ONLY,
	/// IA-64
	IA64,
	/// CPUID.80000001H:EDX.LM[bit 29]
	X64,
	/// CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19]
	ADX,
	/// CPUID.01H:ECX.AES[bit 25]
	AES,
	/// CPUID.01H:ECX.AVX[bit 28]
	AVX,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX2[bit 5]
	AVX2,
	/// CPUID.(EAX=07H, ECX=0H):EDX.AVX512_4FMAPS[bit 3]
	AVX512_4FMAPS,
	/// CPUID.(EAX=07H, ECX=0H):EDX.AVX512_4VNNIW[bit 2]
	AVX512_4VNNIW,
	/// CPUID.(EAX=07H, ECX=1):EAX[bit 5]
	AVX512_BF16,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_BITALG[bit 12]
	AVX512_BITALG,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512_IFMA[bit 21]
	AVX512_IFMA,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VBMI[bit 1]
	AVX512_VBMI,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VBMI2[bit 6]
	AVX512_VBMI2,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VNNI[bit 11]
	AVX512_VNNI,
	/// CPUID.(EAX=07H, ECX=0):EDX[bit 08]
	AVX512_VP2INTERSECT,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VPOPCNTDQ[bit 14]
	AVX512_VPOPCNTDQ,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512BW[bit 30]
	AVX512BW,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512CD[bit 28]
	AVX512CD,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512DQ[bit 17]
	AVX512DQ,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512ER[bit 27]
	AVX512ER,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512F[bit 16]
	AVX512F,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512PF[bit 26]
	AVX512PF,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512VL[bit 31]
	AVX512VL,
	/// CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]
	BMI1,
	/// CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]
	BMI2,
	/// CPUID.(EAX=07H, ECX=0H):EDX.CET_IBT[bit 20]
	CET_IBT,
	/// CPUID.(EAX=07H, ECX=0H):ECX.CET_SS[bit 7]
	CET_SS,
	/// CL1INVMB instruction (Intel SCC = Single-Chip Computer)
	CL1INVMB,
	/// CPUID.(EAX=07H, ECX=0H):ECX.CLDEMOTE[bit 25]
	CLDEMOTE,
	/// CPUID.(EAX=07H, ECX=0H):EBX.CLFLUSHOPT[bit 23]
	CLFLUSHOPT,
	/// CPUID.01H:EDX.CLFSH[bit 19]
	CLFSH,
	/// CPUID.(EAX=07H, ECX=0H):EBX.CLWB[bit 24]
	CLWB,
	/// CPUID.80000008H:EBX.CLZERO[bit 0]
	CLZERO,
	/// CPUID.01H:EDX.CMOV[bit 15]
	CMOV,
	/// CPUID.01H:ECX.CMPXCHG16B[bit 13]
	CMPXCHG16B,
	/// RFLAGS.ID can be toggled
	CPUID,
	/// CPUID.01H:EDX.CX8[bit 8]
	CX8,
	/// CPUID.80000001H:EDX.3DNOW[bit 31]
	D3NOW,
	/// CPUID.80000001H:EDX.3DNOWEXT[bit 30]
	D3NOWEXT,
	/// CPUID.(EAX=12H, ECX=0H):EAX.OSS[bit 5]
	ENCLV,
	/// CPUID.(EAX=07H, ECX=0):ECX[bit 29]
	ENQCMD,
	/// CPUID.01H:ECX.F16C[bit 29]
	F16C,
	/// CPUID.01H:ECX.FMA[bit 12]
	FMA,
	/// CPUID.80000001H:ECX.FMA4[bit 16]
	FMA4,
	/// 8087 or later (CPUID.01H:EDX.FPU[bit 0])
	FPU,
	/// 80287 or later
	FPU287,
	/// 80287XL only
	FPU287XL_ONLY,
	/// 80387 or later
	FPU387,
	/// 80387SL only
	FPU387SL_ONLY,
	/// CPUID.(EAX=07H, ECX=0H):EBX.FSGSBASE[bit 0]
	FSGSBASE,
	/// CPUID.01H:EDX.FXSR[bit 24]
	FXSR,
	/// AMD Geode LX/GX CPU
	GEODE,
	/// CPUID.(EAX=07H, ECX=0H):ECX.GFNI[bit 8]
	GFNI,
	/// CPUID.(EAX=07H, ECX=0H):EBX.HLE[bit 4]
	HLE,
	/// `HLE` or `RTM`
	HLE_or_RTM,
	/// `VMX` and IA32_VMX_EPT_VPID_CAP[bit 20]
	INVEPT,
	/// CPUID.(EAX=07H, ECX=0H):EBX.INVPCID[bit 10]
	INVPCID,
	/// `VMX` and IA32_VMX_EPT_VPID_CAP[bit 32]
	INVVPID,
	/// CPUID.80000001H:ECX.LWP[bit 15]
	LWP,
	/// CPUID.80000001H:ECX.LZCNT[bit 5]
	LZCNT,
	/// CPUID.80000008H:EBX.MCOMMIT[bit 8]
	MCOMMIT,
	/// CPUID.01H:EDX.MMX[bit 23]
	MMX,
	/// CPUID.01H:ECX.MONITOR[bit 3]
	MONITOR,
	/// CPUID.80000001H:ECX.MONITORX[bit 29]
	MONITORX,
	/// CPUID.01H:ECX.MOVBE[bit 22]
	MOVBE,
	/// CPUID.(EAX=07H, ECX=0H):ECX.MOVDIR64B[bit 28]
	MOVDIR64B,
	/// CPUID.(EAX=07H, ECX=0H):ECX.MOVDIRI[bit 27]
	MOVDIRI,
	/// CPUID.(EAX=07H, ECX=0H):EBX.MPX[bit 14]
	MPX,
	/// CPUID.01H:EDX.MSR[bit 5]
	MSR,
	/// Multi-byte nops (0F1F /0): CPUID.01H.EAX[Bits 11:8] = 0110B or 1111B
	MULTIBYTENOP,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.ACE[Bits 7:6] = 11B ([6] = exists, [7] = enabled)
	PADLOCK_ACE,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.PHE[Bits 11:10] = 11B ([10] = exists, [11] = enabled)
	PADLOCK_PHE,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.PMM[Bits 13:12] = 11B ([12] = exists, [13] = enabled)
	PADLOCK_PMM,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.RNG[Bits 3:2] = 11B ([2] = exists, [3] = enabled)
	PADLOCK_RNG,
	/// PAUSE instruction (Pentium 4 or later)
	PAUSE,
	/// CPUID.01H:ECX.PCLMULQDQ[bit 1]
	PCLMULQDQ,
	/// CPUID.(EAX=07H, ECX=0H):EBX.PCOMMIT[bit 22]
	PCOMMIT,
	/// CPUID.(EAX=07H, ECX=0H):EDX.PCONFIG[bit 18]
	PCONFIG,
	/// CPUID.(EAX=07H, ECX=0H):ECX.PKU[bit 3]
	PKU,
	/// CPUID.01H:ECX.POPCNT[bit 23]
	POPCNT,
	/// CPUID.80000001H:ECX.PREFETCHW[bit 8]
	PREFETCHW,
	/// CPUID.(EAX=07H, ECX=0H):ECX.PREFETCHWT1[bit 0]
	PREFETCHWT1,
	/// CPUID.(EAX=14H, ECX=0H):EBX.PTWRITE[bit 4]
	PTWRITE,
	/// CPUID.(EAX=07H, ECX=0H):ECX.RDPID[bit 22]
	RDPID,
	/// RDPMC instruction (Pentium MMX or later, or Pentium Pro or later)
	RDPMC,
	/// CPUID.80000008H:EBX.RDPRU[bit 4]
	RDPRU,
	/// CPUID.01H:ECX.RDRAND[bit 30]
	RDRAND,
	/// CPUID.(EAX=07H, ECX=0H):EBX.RDSEED[bit 18]
	RDSEED,
	/// CPUID.80000001H:EDX.RDTSCP[bit 27]
	RDTSCP,
	/// CPUID.(EAX=07H, ECX=0H):EBX.RTM[bit 11]
	RTM,
	/// CPUID.01H:EDX.SEP[bit 11]
	SEP,
	/// CPUID.(EAX=12H, ECX=0H):EAX.SGX1[bit 0]
	SGX1,
	/// CPUID.(EAX=07H, ECX=0H):EBX.SHA[bit 29]
	SHA,
	/// CPUID.80000001H:ECX.SKINIT[bit 12]
	SKINIT,
	/// `SKINIT` or `SVML`
	SKINIT_or_SVML,
	/// CPUID.(EAX=07H, ECX=0H):EBX.SMAP[bit 20]
	SMAP,
	/// CPUID.01H:ECX.SMX[bit 6]
	SMX,
	/// CPUID.01H:EDX.SSE[bit 25]
	SSE,
	/// CPUID.01H:EDX.SSE2[bit 26]
	SSE2,
	/// CPUID.01H:ECX.SSE3[bit 0]
	SSE3,
	/// CPUID.01H:ECX.SSE4_1[bit 19]
	SSE4_1,
	/// CPUID.01H:ECX.SSE4_2[bit 20]
	SSE4_2,
	/// CPUID.80000001H:ECX.SSE4A[bit 6]
	SSE4A,
	/// CPUID.01H:ECX.SSSE3[bit 9]
	SSSE3,
	/// CPUID.80000001H:ECX.SVM[bit 2]
	SVM,
	/// CPUID.8000000AH:EDX.SVML[bit 2]
	SVML,
	/// CPUID.80000001H:EDX.SYSCALL[bit 11]
	SYSCALL,
	/// CPUID.80000001H:ECX.TBM[bit 21]
	TBM,
	/// CPUID.01H:EDX.TSC[bit 4]
	TSC,
	/// CPUID.(EAX=07H, ECX=0H):ECX.VAES[bit 9]
	VAES,
	/// CPUID.01H:ECX.VMX[bit 5]
	VMX,
	/// CPUID.(EAX=07H, ECX=0H):ECX.VPCLMULQDQ[bit 10]
	VPCLMULQDQ,
	/// CPUID.(EAX=07H, ECX=0H):ECX.WAITPKG[bit 5]
	WAITPKG,
	/// CPUID.(EAX=80000008H, ECX=0H):EBX.WBNOINVD[bit 9]
	WBNOINVD,
	/// CPUID.80000001H:ECX.XOP[bit 11]
	XOP,
	/// CPUID.01H:ECX.XSAVE[bit 26]
	XSAVE,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVEC[bit 1]
	XSAVEC,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVEOPT[bit 0]
	XSAVEOPT,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVES[bit 3]
	XSAVES,
}
// GENERATOR-END: CpuidFeature
