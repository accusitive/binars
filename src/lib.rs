#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use binars_sys::*;
use std::rc::Rc;
use std::{convert::TryInto, ffi::CString};
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub struct Literal
{
    inner: BinaryenLiteral,
}
impl Literal
{
    pub fn new(l: BinaryenLiteral) -> Self
    {
        Self { inner: l }
    }
    /// A 32 bit literal integer
    pub fn int_32(x: i32) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralInt32(x)) }
    }
    /// A 64 bit literal integer

    pub fn int_64(x: i64) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralInt64(x)) }
    }
    /// A 32 bit literal float

    pub fn float_32(x: f32) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat32(x)) }
    }
    /// A 64 bit literal integer

    pub fn float_64(x: f64) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat64(x)) }
    }
    /// Creates a 32 bit float from 32 (non-float) bits
    pub fn float_32_bits(x: i32) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat32Bits(x)) }
    }
        /// Creates a 64 bit float from 64 (non-float) bits

    pub fn float_64_bits(x: i64) -> Literal
    {
        unsafe { Self::new(BinaryenLiteralFloat64Bits(x)) }
    }
}
/// An operation. Typically either Binary, or Unary.
/// Binary operations take 2 children, and return one result,
/// while Unary operations take 1 child, and return one result.
#[derive(Debug)]
pub struct Op
{
    inner: BinaryenOp,
}
/// A macro to quickly generate Binary operations code.
macro_rules! binop {
    ($name: ident, $full: ident) => {
        /// Auto generated binary operation. Takes 2 parents, and returns one child.
        pub fn $name() -> Op
        {
            unsafe { Op::new($full()) }
        }
    };
}
impl Op
{
    pub fn new(op: BinaryenOp) -> Self
    {
        Self { inner: op }
    }
    binop!(clz_int32, BinaryenClzInt32);
    binop!(ctz_int32, BinaryenCtzInt32);
    binop!(popcnt_int32, BinaryenPopcntInt32);
    binop!(neg_float32, BinaryenNegFloat32);
    binop!(abs_float32, BinaryenAbsFloat32);
    binop!(ceil_float32, BinaryenCeilFloat32);
    binop!(floor_float32, BinaryenFloorFloat32);
    binop!(trunc_float32, BinaryenTruncFloat32);
    binop!(nearest_float32, BinaryenNearestFloat32);
    binop!(sqrt_float32, BinaryenSqrtFloat32);
    binop!(eq_z_int32, BinaryenEqZInt32);
    binop!(clz_int64, BinaryenClzInt64);
    binop!(ctz_int64, BinaryenCtzInt64);
    binop!(popcnt_int64, BinaryenPopcntInt64);
    binop!(neg_float64, BinaryenNegFloat64);
    binop!(abs_float64, BinaryenAbsFloat64);
    binop!(ceil_float64, BinaryenCeilFloat64);
    binop!(floor_float64, BinaryenFloorFloat64);
    binop!(trunc_float64, BinaryenTruncFloat64);
    binop!(nearest_float64, BinaryenNearestFloat64);
    binop!(sqrt_float64, BinaryenSqrtFloat64);
    binop!(eq_z_int64, BinaryenEqZInt64);
    binop!(extend_s_int32, BinaryenExtendSInt32);
    binop!(extend_u_int32, BinaryenExtendUInt32);
    binop!(wrap_int64, BinaryenWrapInt64);
    binop!(trunc_s_float32_to_int32, BinaryenTruncSFloat32ToInt32);
    binop!(trunc_s_float32_to_int64, BinaryenTruncSFloat32ToInt64);
    binop!(trunc_u_float32_to_int32, BinaryenTruncUFloat32ToInt32);
    binop!(trunc_u_float32_to_int64, BinaryenTruncUFloat32ToInt64);
    binop!(trunc_s_float64_to_int32, BinaryenTruncSFloat64ToInt32);
    binop!(trunc_s_float64_to_int64, BinaryenTruncSFloat64ToInt64);
    binop!(trunc_u_float64_to_int32, BinaryenTruncUFloat64ToInt32);
    binop!(trunc_u_float64_to_int64, BinaryenTruncUFloat64ToInt64);
    binop!(reinterpret_float32, BinaryenReinterpretFloat32);
    binop!(reinterpret_float64, BinaryenReinterpretFloat64);
    binop!(convert_s_int32_to_float32, BinaryenConvertSInt32ToFloat32);
    binop!(convert_s_int32_to_float64, BinaryenConvertSInt32ToFloat64);
    binop!(convert_u_int32_to_float32, BinaryenConvertUInt32ToFloat32);
    binop!(convert_u_int32_to_float64, BinaryenConvertUInt32ToFloat64);
    binop!(convert_s_int64_to_float32, BinaryenConvertSInt64ToFloat32);
    binop!(convert_s_int64_to_float64, BinaryenConvertSInt64ToFloat64);
    binop!(convert_u_int64_to_float32, BinaryenConvertUInt64ToFloat32);
    binop!(convert_u_int64_to_float64, BinaryenConvertUInt64ToFloat64);
    binop!(promote_float32, BinaryenPromoteFloat32);
    binop!(demote_float64, BinaryenDemoteFloat64);
    binop!(reinterpret_int32, BinaryenReinterpretInt32);
    binop!(reinterpret_int64, BinaryenReinterpretInt64);
    binop!(extend_s8_int32, BinaryenExtendS8Int32);
    binop!(extend_s16_int32, BinaryenExtendS16Int32);
    binop!(extend_s8_int64, BinaryenExtendS8Int64);
    binop!(extend_s16_int64, BinaryenExtendS16Int64);
    binop!(extend_s32_int64, BinaryenExtendS32Int64);
    binop!(add_int32, BinaryenAddInt32);
    binop!(sub_int32, BinaryenSubInt32);
    binop!(mul_int32, BinaryenMulInt32);
    binop!(div_s_int32, BinaryenDivSInt32);
    binop!(div_u_int32, BinaryenDivUInt32);
    binop!(rem_s_int32, BinaryenRemSInt32);
    binop!(rem_u_int32, BinaryenRemUInt32);
    binop!(and_int32, BinaryenAndInt32);
    binop!(or_int32, BinaryenOrInt32);
    binop!(xor_int32, BinaryenXorInt32);
    binop!(shl_int32, BinaryenShlInt32);
    binop!(shr_u_int32, BinaryenShrUInt32);
    binop!(shr_s_int32, BinaryenShrSInt32);
    binop!(rot_l_int32, BinaryenRotLInt32);
    binop!(rot_r_int32, BinaryenRotRInt32);
    binop!(eq_int32, BinaryenEqInt32);
    binop!(ne_int32, BinaryenNeInt32);
    binop!(lt_s_int32, BinaryenLtSInt32);
    binop!(lt_u_int32, BinaryenLtUInt32);
    binop!(le_s_int32, BinaryenLeSInt32);
    binop!(le_u_int32, BinaryenLeUInt32);
    binop!(gt_s_int32, BinaryenGtSInt32);
    binop!(gt_u_int32, BinaryenGtUInt32);
    binop!(ge_s_int32, BinaryenGeSInt32);
    binop!(ge_u_int32, BinaryenGeUInt32);
    binop!(add_int64, BinaryenAddInt64);
    binop!(sub_int64, BinaryenSubInt64);
    binop!(mul_int64, BinaryenMulInt64);
    binop!(div_s_int64, BinaryenDivSInt64);
    binop!(div_u_int64, BinaryenDivUInt64);
    binop!(rem_s_int64, BinaryenRemSInt64);
    binop!(rem_u_int64, BinaryenRemUInt64);
    binop!(and_int64, BinaryenAndInt64);
    binop!(or_int64, BinaryenOrInt64);
    binop!(xor_int64, BinaryenXorInt64);
    binop!(shl_int64, BinaryenShlInt64);
    binop!(shr_u_int64, BinaryenShrUInt64);
    binop!(shr_s_int64, BinaryenShrSInt64);
    binop!(rot_l_int64, BinaryenRotLInt64);
    binop!(rot_r_int64, BinaryenRotRInt64);
    binop!(eq_int64, BinaryenEqInt64);
    binop!(ne_int64, BinaryenNeInt64);
    binop!(lt_s_int64, BinaryenLtSInt64);
    binop!(lt_u_int64, BinaryenLtUInt64);
    binop!(le_s_int64, BinaryenLeSInt64);
    binop!(le_u_int64, BinaryenLeUInt64);
    binop!(gt_s_int64, BinaryenGtSInt64);
    binop!(gt_u_int64, BinaryenGtUInt64);
    binop!(ge_s_int64, BinaryenGeSInt64);
    binop!(ge_u_int64, BinaryenGeUInt64);
    binop!(add_float32, BinaryenAddFloat32);
    binop!(sub_float32, BinaryenSubFloat32);
    binop!(mul_float32, BinaryenMulFloat32);
    binop!(div_float32, BinaryenDivFloat32);
    binop!(copy_sign_float32, BinaryenCopySignFloat32);
    binop!(min_float32, BinaryenMinFloat32);
    binop!(max_float32, BinaryenMaxFloat32);
    binop!(eq_float32, BinaryenEqFloat32);
    binop!(ne_float32, BinaryenNeFloat32);
    binop!(lt_float32, BinaryenLtFloat32);
    binop!(le_float32, BinaryenLeFloat32);
    binop!(gt_float32, BinaryenGtFloat32);
    binop!(ge_float32, BinaryenGeFloat32);
    binop!(add_float64, BinaryenAddFloat64);
    binop!(sub_float64, BinaryenSubFloat64);
    binop!(mul_float64, BinaryenMulFloat64);
    binop!(div_float64, BinaryenDivFloat64);
    binop!(copy_sign_float64, BinaryenCopySignFloat64);
    binop!(min_float64, BinaryenMinFloat64);
    binop!(max_float64, BinaryenMaxFloat64);
    binop!(eq_float64, BinaryenEqFloat64);
    binop!(ne_float64, BinaryenNeFloat64);
    binop!(lt_float64, BinaryenLtFloat64);
    binop!(le_float64, BinaryenLeFloat64);
    binop!(gt_float64, BinaryenGtFloat64);
    binop!(ge_float64, BinaryenGeFloat64);
    binop!(atomic_r_m_w_add, BinaryenAtomicRMWAdd);
    binop!(atomic_r_m_w_sub, BinaryenAtomicRMWSub);
    binop!(atomic_r_m_w_and, BinaryenAtomicRMWAnd);
    binop!(atomic_r_m_w_or, BinaryenAtomicRMWOr);
    binop!(atomic_r_m_w_xor, BinaryenAtomicRMWXor);
    binop!(atomic_r_m_w_xchg, BinaryenAtomicRMWXchg);
    binop!(
        trunc_sat_s_float32_to_int32,
        BinaryenTruncSatSFloat32ToInt32
    );
    binop!(
        trunc_sat_s_float32_to_int64,
        BinaryenTruncSatSFloat32ToInt64
    );
    binop!(
        trunc_sat_u_float32_to_int32,
        BinaryenTruncSatUFloat32ToInt32
    );
    binop!(
        trunc_sat_u_float32_to_int64,
        BinaryenTruncSatUFloat32ToInt64
    );
    binop!(
        trunc_sat_s_float64_to_int32,
        BinaryenTruncSatSFloat64ToInt32
    );
    binop!(
        trunc_sat_s_float64_to_int64,
        BinaryenTruncSatSFloat64ToInt64
    );
    binop!(
        trunc_sat_u_float64_to_int32,
        BinaryenTruncSatUFloat64ToInt32
    );
    binop!(
        trunc_sat_u_float64_to_int64,
        BinaryenTruncSatUFloat64ToInt64
    );
    binop!(splat_vec_i8x16, BinaryenSplatVecI8x16);
    binop!(extract_lane_s_vec_i8x16, BinaryenExtractLaneSVecI8x16);
    binop!(extract_lane_u_vec_i8x16, BinaryenExtractLaneUVecI8x16);
    binop!(replace_lane_vec_i8x16, BinaryenReplaceLaneVecI8x16);
    binop!(splat_vec_i16x8, BinaryenSplatVecI16x8);
    binop!(extract_lane_s_vec_i16x8, BinaryenExtractLaneSVecI16x8);
    binop!(extract_lane_u_vec_i16x8, BinaryenExtractLaneUVecI16x8);
    binop!(replace_lane_vec_i16x8, BinaryenReplaceLaneVecI16x8);
    binop!(splat_vec_i32x4, BinaryenSplatVecI32x4);
    binop!(extract_lane_vec_i32x4, BinaryenExtractLaneVecI32x4);
    binop!(replace_lane_vec_i32x4, BinaryenReplaceLaneVecI32x4);
    binop!(splat_vec_i64x2, BinaryenSplatVecI64x2);
    binop!(extract_lane_vec_i64x2, BinaryenExtractLaneVecI64x2);
    binop!(replace_lane_vec_i64x2, BinaryenReplaceLaneVecI64x2);
    binop!(splat_vec_f32x4, BinaryenSplatVecF32x4);
    binop!(extract_lane_vec_f32x4, BinaryenExtractLaneVecF32x4);
    binop!(replace_lane_vec_f32x4, BinaryenReplaceLaneVecF32x4);
    binop!(splat_vec_f64x2, BinaryenSplatVecF64x2);
    binop!(extract_lane_vec_f64x2, BinaryenExtractLaneVecF64x2);
    binop!(replace_lane_vec_f64x2, BinaryenReplaceLaneVecF64x2);
    binop!(eq_vec_i8x16, BinaryenEqVecI8x16);
    binop!(ne_vec_i8x16, BinaryenNeVecI8x16);
    binop!(lt_s_vec_i8x16, BinaryenLtSVecI8x16);
    binop!(lt_u_vec_i8x16, BinaryenLtUVecI8x16);
    binop!(gt_s_vec_i8x16, BinaryenGtSVecI8x16);
    binop!(gt_u_vec_i8x16, BinaryenGtUVecI8x16);
    binop!(le_s_vec_i8x16, BinaryenLeSVecI8x16);
    binop!(le_u_vec_i8x16, BinaryenLeUVecI8x16);
    binop!(ge_s_vec_i8x16, BinaryenGeSVecI8x16);
    binop!(ge_u_vec_i8x16, BinaryenGeUVecI8x16);
    binop!(eq_vec_i16x8, BinaryenEqVecI16x8);
    binop!(ne_vec_i16x8, BinaryenNeVecI16x8);
    binop!(lt_s_vec_i16x8, BinaryenLtSVecI16x8);
    binop!(lt_u_vec_i16x8, BinaryenLtUVecI16x8);
    binop!(gt_s_vec_i16x8, BinaryenGtSVecI16x8);
    binop!(gt_u_vec_i16x8, BinaryenGtUVecI16x8);
    binop!(le_s_vec_i16x8, BinaryenLeSVecI16x8);
    binop!(le_u_vec_i16x8, BinaryenLeUVecI16x8);
    binop!(ge_s_vec_i16x8, BinaryenGeSVecI16x8);
    binop!(ge_u_vec_i16x8, BinaryenGeUVecI16x8);
    binop!(eq_vec_i32x4, BinaryenEqVecI32x4);
    binop!(ne_vec_i32x4, BinaryenNeVecI32x4);
    binop!(lt_s_vec_i32x4, BinaryenLtSVecI32x4);
    binop!(lt_u_vec_i32x4, BinaryenLtUVecI32x4);
    binop!(gt_s_vec_i32x4, BinaryenGtSVecI32x4);
    binop!(gt_u_vec_i32x4, BinaryenGtUVecI32x4);
    binop!(le_s_vec_i32x4, BinaryenLeSVecI32x4);
    binop!(le_u_vec_i32x4, BinaryenLeUVecI32x4);
    binop!(ge_s_vec_i32x4, BinaryenGeSVecI32x4);
    binop!(ge_u_vec_i32x4, BinaryenGeUVecI32x4);
    binop!(eq_vec_f32x4, BinaryenEqVecF32x4);
    binop!(ne_vec_f32x4, BinaryenNeVecF32x4);
    binop!(lt_vec_f32x4, BinaryenLtVecF32x4);
    binop!(gt_vec_f32x4, BinaryenGtVecF32x4);
    binop!(le_vec_f32x4, BinaryenLeVecF32x4);
    binop!(ge_vec_f32x4, BinaryenGeVecF32x4);
    binop!(eq_vec_f64x2, BinaryenEqVecF64x2);
    binop!(ne_vec_f64x2, BinaryenNeVecF64x2);
    binop!(lt_vec_f64x2, BinaryenLtVecF64x2);
    binop!(gt_vec_f64x2, BinaryenGtVecF64x2);
    binop!(le_vec_f64x2, BinaryenLeVecF64x2);
    binop!(ge_vec_f64x2, BinaryenGeVecF64x2);
    binop!(not_vec128, BinaryenNotVec128);
    binop!(and_vec128, BinaryenAndVec128);
    binop!(or_vec128, BinaryenOrVec128);
    binop!(xor_vec128, BinaryenXorVec128);
    binop!(and_not_vec128, BinaryenAndNotVec128);
    binop!(bitselect_vec128, BinaryenBitselectVec128);
    binop!(abs_vec_i8x16, BinaryenAbsVecI8x16);
    binop!(neg_vec_i8x16, BinaryenNegVecI8x16);
    binop!(any_true_vec_i8x16, BinaryenAnyTrueVecI8x16);
    binop!(all_true_vec_i8x16, BinaryenAllTrueVecI8x16);
    binop!(bitmask_vec_i8x16, BinaryenBitmaskVecI8x16);
    binop!(shl_vec_i8x16, BinaryenShlVecI8x16);
    binop!(shr_s_vec_i8x16, BinaryenShrSVecI8x16);
    binop!(shr_u_vec_i8x16, BinaryenShrUVecI8x16);
    binop!(add_vec_i8x16, BinaryenAddVecI8x16);
    binop!(add_sat_s_vec_i8x16, BinaryenAddSatSVecI8x16);
    binop!(add_sat_u_vec_i8x16, BinaryenAddSatUVecI8x16);
    binop!(sub_vec_i8x16, BinaryenSubVecI8x16);
    binop!(sub_sat_s_vec_i8x16, BinaryenSubSatSVecI8x16);
    binop!(sub_sat_u_vec_i8x16, BinaryenSubSatUVecI8x16);
    binop!(mul_vec_i8x16, BinaryenMulVecI8x16);
    binop!(min_s_vec_i8x16, BinaryenMinSVecI8x16);
    binop!(min_u_vec_i8x16, BinaryenMinUVecI8x16);
    binop!(max_s_vec_i8x16, BinaryenMaxSVecI8x16);
    binop!(max_u_vec_i8x16, BinaryenMaxUVecI8x16);
    binop!(avgr_u_vec_i8x16, BinaryenAvgrUVecI8x16);
    binop!(abs_vec_i16x8, BinaryenAbsVecI16x8);
    binop!(neg_vec_i16x8, BinaryenNegVecI16x8);
    binop!(any_true_vec_i16x8, BinaryenAnyTrueVecI16x8);
    binop!(all_true_vec_i16x8, BinaryenAllTrueVecI16x8);
    binop!(bitmask_vec_i16x8, BinaryenBitmaskVecI16x8);
    binop!(shl_vec_i16x8, BinaryenShlVecI16x8);
    binop!(shr_s_vec_i16x8, BinaryenShrSVecI16x8);
    binop!(shr_u_vec_i16x8, BinaryenShrUVecI16x8);
    binop!(add_vec_i16x8, BinaryenAddVecI16x8);
    binop!(add_sat_s_vec_i16x8, BinaryenAddSatSVecI16x8);
    binop!(add_sat_u_vec_i16x8, BinaryenAddSatUVecI16x8);
    binop!(sub_vec_i16x8, BinaryenSubVecI16x8);
    binop!(sub_sat_s_vec_i16x8, BinaryenSubSatSVecI16x8);
    binop!(sub_sat_u_vec_i16x8, BinaryenSubSatUVecI16x8);
    binop!(mul_vec_i16x8, BinaryenMulVecI16x8);
    binop!(min_s_vec_i16x8, BinaryenMinSVecI16x8);
    binop!(min_u_vec_i16x8, BinaryenMinUVecI16x8);
    binop!(max_s_vec_i16x8, BinaryenMaxSVecI16x8);
    binop!(max_u_vec_i16x8, BinaryenMaxUVecI16x8);
    binop!(avgr_u_vec_i16x8, BinaryenAvgrUVecI16x8);
    binop!(abs_vec_i32x4, BinaryenAbsVecI32x4);
    binop!(neg_vec_i32x4, BinaryenNegVecI32x4);
    binop!(any_true_vec_i32x4, BinaryenAnyTrueVecI32x4);
    binop!(all_true_vec_i32x4, BinaryenAllTrueVecI32x4);
    binop!(bitmask_vec_i32x4, BinaryenBitmaskVecI32x4);
    binop!(shl_vec_i32x4, BinaryenShlVecI32x4);
    binop!(shr_s_vec_i32x4, BinaryenShrSVecI32x4);
    binop!(shr_u_vec_i32x4, BinaryenShrUVecI32x4);
    binop!(add_vec_i32x4, BinaryenAddVecI32x4);
    binop!(sub_vec_i32x4, BinaryenSubVecI32x4);
    binop!(mul_vec_i32x4, BinaryenMulVecI32x4);
    binop!(min_s_vec_i32x4, BinaryenMinSVecI32x4);
    binop!(min_u_vec_i32x4, BinaryenMinUVecI32x4);
    binop!(max_s_vec_i32x4, BinaryenMaxSVecI32x4);
    binop!(max_u_vec_i32x4, BinaryenMaxUVecI32x4);
    binop!(dot_s_vec_i16x8_to_vec_i32x4, BinaryenDotSVecI16x8ToVecI32x4);
    binop!(neg_vec_i64x2, BinaryenNegVecI64x2);
    binop!(any_true_vec_i64x2, BinaryenAnyTrueVecI64x2);
    binop!(all_true_vec_i64x2, BinaryenAllTrueVecI64x2);
    binop!(shl_vec_i64x2, BinaryenShlVecI64x2);
    binop!(shr_s_vec_i64x2, BinaryenShrSVecI64x2);
    binop!(shr_u_vec_i64x2, BinaryenShrUVecI64x2);
    binop!(add_vec_i64x2, BinaryenAddVecI64x2);
    binop!(sub_vec_i64x2, BinaryenSubVecI64x2);
    binop!(mul_vec_i64x2, BinaryenMulVecI64x2);
    binop!(abs_vec_f32x4, BinaryenAbsVecF32x4);
    binop!(neg_vec_f32x4, BinaryenNegVecF32x4);
    binop!(sqrt_vec_f32x4, BinaryenSqrtVecF32x4);
    binop!(q_f_m_a_vec_f32x4, BinaryenQFMAVecF32x4);
    binop!(q_f_m_s_vec_f32x4, BinaryenQFMSVecF32x4);
    binop!(add_vec_f32x4, BinaryenAddVecF32x4);
    binop!(sub_vec_f32x4, BinaryenSubVecF32x4);
    binop!(mul_vec_f32x4, BinaryenMulVecF32x4);
    binop!(div_vec_f32x4, BinaryenDivVecF32x4);
    binop!(min_vec_f32x4, BinaryenMinVecF32x4);
    binop!(max_vec_f32x4, BinaryenMaxVecF32x4);
    binop!(p_min_vec_f32x4, BinaryenPMinVecF32x4);
    binop!(p_max_vec_f32x4, BinaryenPMaxVecF32x4);
    binop!(ceil_vec_f32x4, BinaryenCeilVecF32x4);
    binop!(floor_vec_f32x4, BinaryenFloorVecF32x4);
    binop!(trunc_vec_f32x4, BinaryenTruncVecF32x4);
    binop!(nearest_vec_f32x4, BinaryenNearestVecF32x4);
    binop!(abs_vec_f64x2, BinaryenAbsVecF64x2);
    binop!(neg_vec_f64x2, BinaryenNegVecF64x2);
    binop!(sqrt_vec_f64x2, BinaryenSqrtVecF64x2);
    binop!(q_f_m_a_vec_f64x2, BinaryenQFMAVecF64x2);
    binop!(q_f_m_s_vec_f64x2, BinaryenQFMSVecF64x2);
    binop!(add_vec_f64x2, BinaryenAddVecF64x2);
    binop!(sub_vec_f64x2, BinaryenSubVecF64x2);
    binop!(mul_vec_f64x2, BinaryenMulVecF64x2);
    binop!(div_vec_f64x2, BinaryenDivVecF64x2);
    binop!(min_vec_f64x2, BinaryenMinVecF64x2);
    binop!(max_vec_f64x2, BinaryenMaxVecF64x2);
    binop!(p_min_vec_f64x2, BinaryenPMinVecF64x2);
    binop!(p_max_vec_f64x2, BinaryenPMaxVecF64x2);
    binop!(ceil_vec_f64x2, BinaryenCeilVecF64x2);
    binop!(floor_vec_f64x2, BinaryenFloorVecF64x2);
    binop!(trunc_vec_f64x2, BinaryenTruncVecF64x2);
    binop!(nearest_vec_f64x2, BinaryenNearestVecF64x2);
    binop!(
        trunc_sat_s_vec_f32x4_to_vec_i32x4,
        BinaryenTruncSatSVecF32x4ToVecI32x4
    );
    binop!(
        trunc_sat_u_vec_f32x4_to_vec_i32x4,
        BinaryenTruncSatUVecF32x4ToVecI32x4
    );
    binop!(
        trunc_sat_s_vec_f64x2_to_vec_i64x2,
        BinaryenTruncSatSVecF64x2ToVecI64x2
    );
    binop!(
        trunc_sat_u_vec_f64x2_to_vec_i64x2,
        BinaryenTruncSatUVecF64x2ToVecI64x2
    );
    binop!(
        convert_s_vec_i32x4_to_vec_f32x4,
        BinaryenConvertSVecI32x4ToVecF32x4
    );
    binop!(
        convert_u_vec_i32x4_to_vec_f32x4,
        BinaryenConvertUVecI32x4ToVecF32x4
    );
    binop!(
        convert_s_vec_i64x2_to_vec_f64x2,
        BinaryenConvertSVecI64x2ToVecF64x2
    );
    binop!(
        convert_u_vec_i64x2_to_vec_f64x2,
        BinaryenConvertUVecI64x2ToVecF64x2
    );
    binop!(load_splat_vec8x16, BinaryenLoadSplatVec8x16);
    binop!(load_splat_vec16x8, BinaryenLoadSplatVec16x8);
    binop!(load_splat_vec32x4, BinaryenLoadSplatVec32x4);
    binop!(load_splat_vec64x2, BinaryenLoadSplatVec64x2);
    binop!(
        load_ext_s_vec8x8_to_vec_i16x8,
        BinaryenLoadExtSVec8x8ToVecI16x8
    );
    binop!(
        load_ext_u_vec8x8_to_vec_i16x8,
        BinaryenLoadExtUVec8x8ToVecI16x8
    );
    binop!(
        load_ext_s_vec16x4_to_vec_i32x4,
        BinaryenLoadExtSVec16x4ToVecI32x4
    );
    binop!(
        load_ext_u_vec16x4_to_vec_i32x4,
        BinaryenLoadExtUVec16x4ToVecI32x4
    );
    binop!(
        load_ext_s_vec32x2_to_vec_i64x2,
        BinaryenLoadExtSVec32x2ToVecI64x2
    );
    binop!(
        load_ext_u_vec32x2_to_vec_i64x2,
        BinaryenLoadExtUVec32x2ToVecI64x2
    );
    binop!(
        narrow_s_vec_i16x8_to_vec_i8x16,
        BinaryenNarrowSVecI16x8ToVecI8x16
    );
    binop!(
        narrow_u_vec_i16x8_to_vec_i8x16,
        BinaryenNarrowUVecI16x8ToVecI8x16
    );
    binop!(
        narrow_s_vec_i32x4_to_vec_i16x8,
        BinaryenNarrowSVecI32x4ToVecI16x8
    );
    binop!(
        narrow_u_vec_i32x4_to_vec_i16x8,
        BinaryenNarrowUVecI32x4ToVecI16x8
    );
    binop!(
        widen_low_s_vec_i8x16_to_vec_i16x8,
        BinaryenWidenLowSVecI8x16ToVecI16x8
    );
    binop!(
        widen_high_s_vec_i8x16_to_vec_i16x8,
        BinaryenWidenHighSVecI8x16ToVecI16x8
    );
    binop!(
        widen_low_u_vec_i8x16_to_vec_i16x8,
        BinaryenWidenLowUVecI8x16ToVecI16x8
    );
    binop!(
        widen_high_u_vec_i8x16_to_vec_i16x8,
        BinaryenWidenHighUVecI8x16ToVecI16x8
    );
    binop!(
        widen_low_s_vec_i16x8_to_vec_i32x4,
        BinaryenWidenLowSVecI16x8ToVecI32x4
    );
    binop!(
        widen_high_s_vec_i16x8_to_vec_i32x4,
        BinaryenWidenHighSVecI16x8ToVecI32x4
    );
    binop!(
        widen_low_u_vec_i16x8_to_vec_i32x4,
        BinaryenWidenLowUVecI16x8ToVecI32x4
    );
    binop!(
        widen_high_u_vec_i16x8_to_vec_i32x4,
        BinaryenWidenHighUVecI16x8ToVecI32x4
    );
    binop!(swizzle_vec8x16, BinaryenSwizzleVec8x16);
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// An expression, Nearly everything is an expression. Expressions can be chained, making this a very useful datatype.
pub struct ExpressionRef
{
    inner: BinaryenExpressionRef,
}
impl ExpressionRef
{
    /// Crate a new ExpressionRef from an internal BinaryenExpressionRef.
    pub fn new(expr: BinaryenExpressionRef) -> Self
    {
        ExpressionRef { inner: expr }
    }

    /// Create a null expressionref.
    pub fn null_expr() -> ExpressionRef
    {
        Self::new(std::ptr::null_mut::<BinaryenExpression>())
    }
    /// Print the WAT code for the expression ref.
    pub fn print(&self)
    {
        unsafe { BinaryenExpressionPrint(self.inner) }
    }
}
#[derive(Debug)]
/// A WASM export.
pub struct Export
{
    inner: *mut BinaryenExport,
}
impl Export
{
    /// Create a mew export from an internal BinaryenExport
    fn new(expr: *mut BinaryenExport) -> Self
    {
        Self { inner: expr }
    }
}
// Allow to be sent over thread
unsafe impl Send for Export {}
unsafe impl Sync for Export {}
/// The main `Module` struct. 
/// This struct is the base for nearly everything.
//TODO: Document this better (with examples?)
#[derive(Debug)]
pub struct Module
{
    inner: BinaryenModuleRef,
}
unsafe impl Send for Module {}
unsafe impl Sync for Module {}
impl Module
{
    /// Create a `Module` from an internal BinaryenModuleRef.
    pub fn new() -> Self
    {
        return unsafe {
            Self {
                inner: BinaryenModuleCreate(),
            }
        };
    }
    /// Print the WAT to the current `Module`.
    pub fn print(&mut self)
    {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    /// Alias to `Module#print`.
    pub fn print_wat(&mut self)
    {
        unsafe { BinaryenModulePrint(self.inner) }
    }
    /// Print the current Module as ASM.js code.
    pub fn print_asmjs(&mut self)
    {
        unsafe { BinaryenModulePrintAsmjs(self.inner) }
    }
    /// Get a reference to a local variable.
    pub fn get_local(&mut self, index: i32, dype: Type) -> ExpressionRef
    {
        unsafe {
            ExpressionRef::new(BinaryenLocalGet(
                self.inner,
                index.try_into().unwrap(),
                dype.inner,
            ))
        }
    }
    /// Set a local variable by index.
    pub fn set_local(&mut self, index: i32, value: ExpressionRef) -> ExpressionRef
    {
        unsafe {
            ExpressionRef::new(BinaryenLocalSet(
                self.inner,
                index.try_into().unwrap(),
                value.inner,
            ))
        }
    }
    /// Get a reference to a binary expression.
    pub fn binary(&mut self, op: Op, left: ExpressionRef, right: ExpressionRef) -> ExpressionRef
    {
        return unsafe {
            ExpressionRef::new(BinaryenBinary(
                self.inner,
                op.inner,
                left.inner,
                right.inner,
            ))
        };
    }
    /// Add a function to the module.
    pub fn add_function(
        &mut self,
        name: &str,
        params: Type,
        results: Type,
        var_types: Vec<Type>,
        body: ExpressionRef,
    ) -> FunctionRef
    {
        let mut inners = var_types
            .iter()
            .map(|t| t.inner)
            .collect::<Vec<BinaryenType>>();

        FunctionRef::new(unsafe {
            BinaryenAddFunction(
                self.inner,
                CString::new(name).unwrap().as_ptr(),
                params.inner,
                results.inner,
                inners.as_mut_ptr(),
                var_types.len().try_into().unwrap(),
                body.inner,
            )
        })
    }
    /// Bool whether the validation was successful
    pub fn validate(&mut self) -> bool
    {
        unsafe { BinaryenModuleValidate(self.inner) == 1 }
    }
    /// Incase you want to have the raw validation number.
    pub fn validate_i(&mut self) -> i32
    {
        unsafe { BinaryenModuleValidate(self.inner) }
    }
    /// Optimise the current module.
    pub fn optimize(&mut self)
    {
        unsafe { BinaryenModuleOptimize(self.inner) }
    }
    #[doc = "Get current optimization level, set new optimization `level`, optimize, set back to original optimization level."]
    pub fn optimize_with_level(&mut self, level: i32)
    {
        let current_level = unsafe { BinaryenGetOptimizeLevel() };
        unsafe { BinaryenSetOptimizeLevel(level) }
        unsafe { BinaryenModuleOptimize(self.inner) }
        unsafe { BinaryenSetOptimizeLevel(current_level) }
    }
    /// Make a constant from the literal.
    // Maybe add an alias to avoid using `Literal` here? 
    pub fn make_const(&mut self, value: Literal) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenConst(self.inner, value.inner) })
    }
    /// Creates an anonymous block.
    pub fn new_nameless_block(&mut self, children: Vec<ExpressionRef>, type_: Type)
        -> ExpressionRef
    {
        let mut inners = children
            .iter()
            .map(|t| t.inner)
            .collect::<Vec<BinaryenExpressionRef>>();
        ExpressionRef::new(unsafe {
            BinaryenBlock(
                self.inner,
                std::ptr::null(),
                inners.as_mut_ptr(),
                inners.len().try_into().unwrap(),
                type_.inner,
            )
        })
    }
    /// Creates a named block.
    pub fn new_block(
        &mut self,
        name: &str,
        children: Vec<ExpressionRef>,
        type_: Type,
    ) -> ExpressionRef
    {
        let mut inners = children
            .iter()
            .map(|t| t.inner)
            .collect::<Vec<BinaryenExpressionRef>>();
        ExpressionRef::new(unsafe {
            BinaryenBlock(
                self.inner,
                CString::new(name).unwrap().as_ptr(),
                inners.as_mut_ptr(),
                inners.len().try_into().unwrap(),
                type_.inner,
            )
        })
    }
    /// Add an export to the module
    pub fn add_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name.to_string()).unwrap();
        let c_external_name = CString::new(external_name.to_string()).unwrap();
        return Export::new(unsafe {
            BinaryenAddExport(
                self.inner,
                c_internal_name.as_ptr(),
                c_external_name.as_ptr(),
            )
        });
    }
    /// Add an export to a function in the module.
    pub fn add_function_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap();
        let c_external_name = CString::new(external_name).unwrap();

        return Export::new(unsafe {
            BinaryenAddFunctionExport(
                self.inner,
                c_internal_name.as_ptr(),
                c_external_name.as_ptr(),
            )
        });
    }
    /// Add an export to a table
    pub fn add_table_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap().as_ptr();
        let c_external_name = CString::new(external_name).unwrap().as_ptr();

        return Export::new(unsafe {
            BinaryenAddTableExport(self.inner, c_internal_name, c_external_name)
        });
    }
    /// Add an export to a memory.
    pub fn add_memory_export(&mut self, internal_name: &str, external_name: &str) -> Export
    {
        let c_internal_name = CString::new(internal_name).unwrap().as_ptr();
        let c_external_name = CString::new(external_name).unwrap().as_ptr();

        return Export::new(unsafe {
            BinaryenAddMemoryExport(self.inner, c_internal_name, c_external_name)
        });
    }
    //TODO: Fix

    // pub fn write(&mut self, filename: &str)
    // {
    //     let c = unsafe {
    //         let was_color_originally_enabled = BinaryenAreColorsEnabled();
    //         BinaryenSetColorsEnabled(0);
    //         let result =
    //             BinaryenModuleAllocateAndWrite(self.inner, std::ptr::null_mut());
    //         BinaryenSetColorsEnabled(was_color_originally_enabled);
    //         // result
    //         // std::ffi::CStr::from_ptr(result.binary as  *const i8)
    //         // result.binaryBytes
    //         result.binary.as_ref().unwrap()
    //     };
    //     println!("{:?}", c);
    //     // std::fs::write(filename, c.to_string_lossy().to_string()).unwrap();
    // }
    /// TOOD: Broken needs fixed dont use
    pub fn write_text(&mut self, filename: &str)
    {
        let c = unsafe {
            let was_color_originally_enabled = BinaryenAreColorsEnabled();
            BinaryenSetColorsEnabled(0);
            let c: *mut ::std::os::raw::c_char = BinaryenModuleAllocateAndWriteText(self.inner);
            BinaryenSetColorsEnabled(was_color_originally_enabled);

            std::ffi::CStr::from_ptr(c)
        };
        std::fs::write(filename, c.to_string_lossy().to_string()).unwrap();
    }
        ///TOOD: Broken needs fixed dont use

    pub fn compile(&mut self) -> &str
    {
        let c = unsafe {
            let _was_color_originally_enabled = BinaryenAreColorsEnabled();
            let c = BinaryenModuleAllocateAndWrite(self.inner, std::ptr::null());
            println!("c: {:?}\n", c);
            println!("{:?}", 0x7f7afc00bde0 as *const i8);

            std::ffi::CStr::from_ptr(c.binary as *const i8)
        };
        std::fs::write("testing", c.to_string_lossy().to_string()).unwrap();
        std::fs::remove_file("testing").unwrap();

        "   "

        // c
    }
        ///TOOD: Broken needs fixed dont use

    pub fn compile_text(&mut self) -> String
    {
        let c = unsafe {
            let was_color_originally_enabled = BinaryenAreColorsEnabled();
            BinaryenSetColorsEnabled(0);
            let c: *mut ::std::os::raw::c_char = BinaryenModuleAllocateAndWriteText(self.inner);
            BinaryenSetColorsEnabled(was_color_originally_enabled);

            std::ffi::CStr::from_ptr(c)
        };
        c.to_string_lossy().to_string()
        // std::fs::write(filename, c.to_string_lossy().to_string()).unwrap();
    }
    /// Create an unary expression from an Operation.
    pub fn unary(&mut self, op: Op, value: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenUnary(self.inner, op.inner, value.inner) })
    }
    /// Cant use `module.drop()` because thats taken up by `impl Drop`.
    pub fn drop_var(&mut self, var: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenDrop(self.inner, var.inner) })
    }
    /// Initialize some memory.
    pub fn memory_init(
        &mut self,
        segment: i32,
        dest: ExpressionRef,
        offset: ExpressionRef,
        size: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenMemoryInit(
                self.inner,
                segment.try_into().unwrap(),
                dest.inner,
                offset.inner,
                size.inner,
            )
        })
    }
    /// Copy some memory.
    pub fn memory_copy(
        &mut self,
        dest: ExpressionRef,
        source: ExpressionRef,
        size: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenMemoryCopy(self.inner, dest.inner, source.inner, size.inner)
        })
    }
    pub fn memory_fill(
        &mut self,
        dest: ExpressionRef,
        value: ExpressionRef,
        size: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenMemoryFill(self.inner, dest.inner, value.inner, size.inner)
        })
    }
    pub fn data_drop(&mut self, segment: i32) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenDataDrop(self.inner, segment.try_into().unwrap()) })
    }
    pub fn ref_null(&mut self, type_: Type) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenRefNull(self.inner, type_.inner) })
    }

    pub fn ref_func(&mut self, rfunc: &str) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            let cfunc = CString::new(rfunc).unwrap();
            BinaryenRefFunc(self.inner, cfunc.as_ptr())
        })
    }
    pub fn make_i31(&mut self, value: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenI31New(self.inner, value.inner) })
    }
    pub fn add_event(&mut self, name: &str, attribute: i32, params: Type, results: Type)
        -> EventRef
    {
        EventRef::new(unsafe {
            let cname = CString::new(name).unwrap();
            BinaryenAddEvent(
                self.inner,
                cname.as_ptr(),
                attribute.try_into().unwrap(),
                params.inner,
                results.inner,
            )
        })
    }
    pub fn throw(&mut self, event: &str, operands: Vec<ExpressionRef>) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            let mut inners = operands
                .iter()
                .map(|t| t.inner)
                .collect::<Vec<BinaryenExpressionRef>>();

            let cevent = CString::new(event).unwrap();
            BinaryenThrow(
                self.inner,
                cevent.as_ptr(),
                inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
            )
        })
    }
    pub fn pop(&mut self, type_: Type) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenPop(self.inner, type_.inner) })
    }
    pub fn rethrow(&mut self, exnref: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenRethrow(self.inner, exnref.inner) })
    }
    pub fn br_on_exn(
        &mut self,
        name: &str,
        event_name: &str,
        exnref: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            let cname = CString::new(name).unwrap();
            let cevent_name = CString::new(event_name).unwrap();
            BinaryenBrOnExn(
                self.inner,
                cname.as_ptr(),
                cevent_name.as_ptr(),
                exnref.inner,
            )
        })
    }
    pub fn r#if(
        &mut self,
        condition: ExpressionRef,
        if_true: ExpressionRef,
        if_false: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenIf(self.inner, condition.inner, if_true.inner, if_false.inner)
            // match if_false {
            //     Some(z) => BinaryenIf(self.inner, condition.inner, if_true.inner, z.inner),
            //     None => BinaryenIf(
            //         self.inner,
            //         condition.inner,
            //         if_true.inner,
            //         std::mem::MaybeUninit::<BinaryenExpression>::uninit().as_mut_ptr(),
            //     ),
            // }
        })
    }
    pub fn r#loop(&mut self, ins: &str, body: ExpressionRef) -> ExpressionRef
    {
        unsafe {
            ExpressionRef::new(BinaryenLoop(
                self.inner,
                CString::new(ins).unwrap().as_ptr(),
                body.inner,
            ))
        }
    }

    pub fn r#break(
        &mut self,
        name: &str,
        condition: Option<ExpressionRef>,
        value: Option<ExpressionRef>,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            let cins = CString::new(name).unwrap();
            match condition {
                Some(cond) => match value {
                    Some(v) => BinaryenBreak(self.inner, cins.as_ptr(), cond.inner, v.inner),
                    None => BinaryenBreak(
                        self.inner,
                        cins.as_ptr(),
                        cond.inner,
                        ExpressionRef::null_expr().inner,
                    ),
                },
                None => match value {
                    Some(v) => BinaryenBreak(
                        self.inner,
                        cins.as_ptr(),
                        ExpressionRef::null_expr().inner,
                        v.inner,
                    ),
                    None => BinaryenBreak(
                        self.inner,
                        cins.as_ptr(),
                        ExpressionRef::null_expr().inner,
                        ExpressionRef::null_expr().inner,
                    ),
                },
            }
        })
    }

    pub fn r#switch(
        &mut self,
        names: Vec<&str>,
        default_name: &str,
        condition: ExpressionRef,
        value: ExpressionRef,
    ) -> ExpressionRef
    {
        let mut cnames = names
            .iter()
            .map(|&n| CString::new(n).unwrap().as_ptr())
            .collect::<Vec<*const std::os::raw::c_char>>();
        ExpressionRef::new(unsafe {
            BinaryenSwitch(
                self.inner,
                cnames.as_mut_ptr(),
                cnames.len().try_into().unwrap(),
                CString::new(default_name).unwrap().as_ptr(),
                condition.inner,
                value.inner,
            )
        })
    }
    pub fn r#call(
        &mut self,
        target: &str,
        operands: Vec<ExpressionRef>,
        return_type: Type,
    ) -> ExpressionRef
    {
        let mut inners = operands
            .iter()
            .map(|o| o.inner)
            .collect::<Vec<BinaryenExpressionRef>>();

        ExpressionRef::new(unsafe {
            BinaryenCall(
                self.inner,
                CString::new(target).unwrap().as_ptr(),
                inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
                return_type.inner,
            )
        })
    }
    pub fn call_indirect(
        &mut self,
        target: ExpressionRef,
        operands: Vec<ExpressionRef>,
        params: Type,
        results: Type,
    ) -> ExpressionRef
    {
        let mut operands_inners = operands
            .iter()
            .map(|o| o.inner)
            .collect::<Vec<BinaryenExpressionRef>>();

        ExpressionRef::new(unsafe {
            BinaryenCallIndirect(
                self.inner,
                target.inner,
                operands_inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
                params.inner,
                results.inner,
            )
        })
    }
    pub fn tee_local(&mut self, index: i32, value: ExpressionRef, type_: Type) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenLocalTee(self.inner, index as u32, value.inner, type_.inner)
        })
    }
    pub fn load(
        &mut self,
        bytes: i32,
        signed: i8,
        offset: i32,
        align: i32,
        type_: Type,
        ptr: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenLoad(
                self.inner,
                bytes.try_into().unwrap(),
                signed,
                offset.try_into().unwrap(),
                align.try_into().unwrap(),
                type_.inner,
                ptr.inner,
            )
        })
    }
    pub fn store(
        &mut self,
        bytes: i32,
        offset: i32,
        align: i32,
        ptr: ExpressionRef,
        value: ExpressionRef,
        type_: Type,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenStore(
                self.inner,
                bytes.try_into().unwrap(),
                offset.try_into().unwrap(),
                align.try_into().unwrap(),
                ptr.inner,
                value.inner,
                type_.inner,
            )
        })
    }
    pub fn select(
        &mut self,
        condition: ExpressionRef,
        if_true: ExpressionRef,
        if_false: ExpressionRef,
        type_: Type,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenSelect(
                self.inner,
                condition.inner,
                if_true.inner,
                if_false.inner,
                type_.inner,
            )
        })
    }
    pub fn r#return(&mut self, value: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenReturn(self.inner, value.inner) })
    }
    pub fn return_call(
        &mut self,
        target: &str,
        operands: Vec<ExpressionRef>,
        return_type: Type,
    ) -> ExpressionRef
    {
        let mut operands_inners = operands
            .iter()
            .map(|o| o.inner)
            .collect::<Vec<BinaryenExpressionRef>>();
        ExpressionRef::new(unsafe {
            let ctarget = CString::new(target).unwrap();
            BinaryenReturnCall(
                self.inner,
                ctarget.as_ptr(),
                operands_inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
                return_type.inner,
            )
        })
    }
    pub fn return_call_indirect(
        &mut self,
        target: ExpressionRef,
        operands: Vec<ExpressionRef>,
        params: Type,
        result_type: Type,
    ) -> ExpressionRef
    {
        let mut operands_inners = operands
            .iter()
            .map(|o| o.inner)
            .collect::<Vec<BinaryenExpressionRef>>();

        ExpressionRef::new(unsafe {
            BinaryenReturnCallIndirect(
                self.inner,
                target.inner,
                operands_inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
                params.inner,
                result_type.inner,
            )
        })
    }
    pub fn ref_is_null(&mut self, value: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenRefIsNull(self.inner, value.inner) })
    }
    pub fn ref_eq(&mut self, left: ExpressionRef, right: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenRefEq(self.inner, left.inner, right.inner) })
    }
    pub fn r#try(&mut self, body: ExpressionRef, catch: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenTry(self.inner, body.inner, catch.inner) })
    }
    pub fn atomic_store(
        &mut self,
        bytes: i32,
        offset: i32,
        ptr: ExpressionRef,
        value: ExpressionRef,
        type_: Type,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenAtomicStore(
                self.inner,
                bytes.try_into().unwrap(),
                offset.try_into().unwrap(),
                ptr.inner,
                value.inner,
                type_.inner,
            )
        })
    }
    pub fn atomic_load(
        &mut self,
        bytes: i32,
        offset: i32,
        type_: Type,
        ptr: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenAtomicLoad(
                self.inner,
                bytes.try_into().unwrap(),
                offset.try_into().unwrap(),
                type_.inner,
                ptr.inner,
            )
        })
    }
    pub fn atomic_wait(
        &mut self,
        ptr: ExpressionRef,
        expected: ExpressionRef,
        timeout: ExpressionRef,
        type_: Type,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenAtomicWait(
                self.inner,
                ptr.inner,
                expected.inner,
                timeout.inner,
                type_.inner,
            )
        })
    }
    pub fn atomic_notify(
        &mut self,
        ptr: ExpressionRef,
        notify_count: ExpressionRef,
    ) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenAtomicNotify(self.inner, ptr.inner, notify_count.inner)
        })
    }
    pub fn atomic_fence(&mut self) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenAtomicFence(self.inner) })
    }
    pub fn make_tuple(&mut self, operands: Vec<ExpressionRef>) -> ExpressionRef
    {
        let mut operands_inners = operands
            .iter()
            .map(|o| o.inner)
            .collect::<Vec<BinaryenExpressionRef>>();

        ExpressionRef::new(unsafe {
            BinaryenTupleMake(
                self.inner,
                operands_inners.as_mut_ptr(),
                operands.len().try_into().unwrap(),
            )
        })
    }
    pub fn extract_tuple(&mut self, tuple: ExpressionRef, index: i32) -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            BinaryenTupleExtract(self.inner, tuple.inner, index.try_into().unwrap())
        })
    }
    pub fn memory_size(&mut self) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenMemorySize(self.inner) })
    }
    pub fn memory_grow(&mut self, delta: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenMemoryGrow(self.inner, delta.inner) })
    }
    pub fn new_i31(&mut self, value: ExpressionRef) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenI31New(self.inner, value.inner) })
    }
    pub fn get_i31(&mut self, i31: ExpressionRef, signed: i32) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenI31Get(self.inner, i31.inner, signed) })
    }
    pub fn nop(&mut self) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenNop(self.inner) })
    }
    pub fn unreachable(&mut self) -> ExpressionRef
    {
        ExpressionRef::new(unsafe { BinaryenUnreachable(self.inner) })
    }
    //TODO: Use bool instead of i8
    pub fn add_global(
        &mut self,
        name: &str,
        type_: Type,
        mutable: i8,
        init: ExpressionRef,
    ) -> GlobalRef
    {
        GlobalRef::new(unsafe {
            let cname = CString::new(name).unwrap();
            BinaryenAddGlobal(self.inner, cname.as_ptr(), type_.inner, mutable, init.inner)
        })
    }
    pub fn add_function_import(
        &mut self,
        internal_name: &str,
        external_module_name: &str,
        external_base_name: &str,
        params: Type,
        result: Type,
    )
    {
        unsafe {
            let c_internal_name = CString::new(internal_name).unwrap();
            let c_external_module_name = CString::new(external_module_name).unwrap();
            let c_external_base_name = CString::new(external_base_name).unwrap();
            BinaryenAddFunctionImport(
                self.inner,
                c_internal_name.as_ptr(),
                c_external_module_name.as_ptr(),
                c_external_base_name.as_ptr(),
                params.inner,
                result.inner,
            );
        }
    }
    pub fn set_function_table(
        &mut self,
        initial: u32,
        maximum: u32,
        func_names: Vec<&str>,
        offset: ExpressionRef,
    )
    {
        unsafe {
            let mut c_func_names = vec![];
            for n in func_names {
                let c = CString::new(n).unwrap();
                c_func_names.push(c)
            }
            // unsafe {
            BinaryenSetFunctionTable(
                self.inner,
                initial,
                maximum,
                c_func_names
                    .iter()
                    .map(|c| c.as_ptr())
                    .collect::<Vec<*const i8>>()
                    .as_mut_ptr(),
                c_func_names.len().try_into().unwrap(),
                offset.inner,
            )
            // }
        }
    }
    //TODO: More docs on this black box
    pub fn set_memory(
        &mut self,
        initial: u32,
        maximum: u32,
        export_name: &str,
        segments: Vec<&str>,
        mut segment_passive: Vec<i8>,
        segment_offsets: Vec<ExpressionRef>,
        mut segment_sizes: Vec<u32>,
        shared: bool,
    )
    {
        let mut csegs = vec![];
        for s in segments {
            csegs.push(CString::new(s).unwrap());
        }
        unsafe {
            let en = CString::new(export_name).unwrap();

            let mut offset_inners = segment_offsets
                .iter()
                .map(|e| e.inner)
                .collect::<Vec<BinaryenExpressionRef>>();

            BinaryenSetMemory(
                self.inner,
                initial,
                maximum,
                en.as_ptr(),
                csegs
                    .iter()
                    .map(|s| s.as_ptr())
                    .collect::<Vec<*const i8>>()
                    .as_mut_ptr(),
                segment_passive.as_mut_ptr(),
                offset_inners.as_mut_ptr(),
                segment_sizes.as_mut_ptr(),
                csegs.len().try_into().unwrap(),
                if shared { 1 } else { 0 },
            )
        }
    }
    pub fn set_start(&mut self, start: FunctionRef)
    {
        unsafe { BinaryenSetStart(self.inner, start.inner) }
    }
    pub fn auto_drop(&mut self)
    {
        unsafe { BinaryenModuleAutoDrop(self.inner) }
    }
    pub fn set_features(&mut self, features: i32)
    {
        unsafe { BinaryenModuleSetFeatures(self.inner, features as u32) }
    }
    pub fn get_features(&mut self) -> Features
    {
        Features {
            inner: unsafe { BinaryenModuleGetFeatures(self.inner) },
        }
    }
    pub fn make_relooper(&mut self) -> BRelooperRef
    {
        BRelooperRef::new(unsafe { RelooperCreate(self.inner) })
    }
}
impl From<&str> for Module
{
    fn from(s: &str) -> Self
    {
        return Module {
            inner: unsafe {
                let mut c = s
                    .chars()
                    .map(|c| c as std::os::raw::c_char)
                    .collect::<Vec<std::os::raw::c_char>>();
                let z = c.as_mut_ptr();
                println!("{:?}", z);
                BinaryenModuleRead(z, s.len().try_into().unwrap())
            },
        };
    }
}
impl Drop for Module
{
    fn drop(&mut self)
    {
        unsafe { BinaryenModuleDispose(self.inner) }
    }
}
pub struct BLooperBlockRef
{
    inner: RelooperBlockRef,
}
impl BLooperBlockRef
{
    fn new(rb: RelooperBlockRef) -> Self
    {
        Self { inner: rb }
    }
}
//TODO: Find better name for BRelooperRef
pub struct BRelooperRef
{
    inner: RelooperRef,
}
impl BRelooperRef
{
    fn new(r: RelooperRef) -> Self
    {
        Self { inner: r }
    }
    pub fn add_block(&mut self, code: ExpressionRef) -> BLooperBlockRef
    {
        BLooperBlockRef::new(unsafe { RelooperAddBlock(self.inner, code.inner) })
    }
    //TODO: Consistently use i32 or u32, not both
    pub fn render_and_dispose(&mut self, entry: BLooperBlockRef, label_helper: u32)
        -> ExpressionRef
    {
        ExpressionRef::new(unsafe {
            RelooperRenderAndDispose(self.inner, entry.inner, label_helper)
        })
    }
    // Doesnt necessarily have to be in relooper, but I just think that its cleaner this way
    pub fn add_branch(
        from: &BLooperBlockRef,
        to: &BLooperBlockRef,
        condition: ExpressionRef,
        code: ExpressionRef,
    )
    {
        unsafe { RelooperAddBranch(from.inner, to.inner, condition.inner, code.inner) }
    }
    pub fn add_block_with_switch(
        &mut self,
        code: ExpressionRef,
        condition: ExpressionRef,
    ) -> BLooperBlockRef
    {
        BLooperBlockRef::new(unsafe {
            RelooperAddBlockWithSwitch(self.inner, code.inner, condition.inner)
        })
    }
    pub fn add_branch_for_switch(
        from: &BLooperBlockRef,
        to: &BLooperBlockRef,
        mut indexes: Vec<u32>,
        code: ExpressionRef,
    )
    {
        unsafe {
            RelooperAddBranchForSwitch(
                from.inner,
                to.inner,
                indexes.as_mut_ptr(),
                indexes.len().try_into().unwrap(),
                code.inner,
            )
        }
    }
}
#[allow(dead_code)]
pub struct GlobalRef
{
    inner: BinaryenGlobalRef,
}
impl GlobalRef
{
    fn new(g: BinaryenGlobalRef) -> Self
    {
        Self { inner: g }
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MType
{
    I32,
    I64,
    F32,
    F64,
    None_,
    Auto,
    Unreachable,
    Funcref,
    Externref,
    Exnref,
    I31Ref,
    EqRef,
    Multi,
    Neg,
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Type
{
    inner: BinaryenType,
    pub matchable_type: MType,
}
impl Type
{
    pub fn neg() -> Self
    {
        return Self {
            inner: { usize::MAX },
            matchable_type: MType::Neg,
        };
    }
    pub fn int_32() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeInt32() } },
            matchable_type: MType::I32,
        };
    }
    pub fn int_64() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeInt64() } },
            matchable_type: MType::I64,
        };
    }
    pub fn float_32() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeFloat32() } },
            matchable_type: MType::F32,
        };
    }
    pub fn float_64() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeFloat64() } },
            matchable_type: MType::F64,
        };
    }
    pub fn none() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeNone() } },
            matchable_type: MType::None_,
        };
    }
    pub fn unreachable() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeUnreachable() } },
            matchable_type: MType::Unreachable,
        };
    }
    pub fn funcref() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeFuncref() } },
            matchable_type: MType::Funcref,
        };
    }
    pub fn externref() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeExternref() } },
            matchable_type: MType::Externref,
        };
    }
    pub fn exnref() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeExnref() } },
            matchable_type: MType::Exnref,
        };
    }
    pub fn auto() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeAuto() } },
            matchable_type: MType::Auto,
        };
    }
    pub fn i31ref() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeI31ref() } },
            matchable_type: MType::I31Ref,
        };
    }
    pub fn eqref() -> Self
    {
        return Self {
            inner: { unsafe { BinaryenTypeEqref() } },
            matchable_type: MType::EqRef,
        };
    }

    pub fn create(value_types: Vec<Type>) -> Self
    {
        return unsafe {
            let mut inners = value_types
                .iter()
                .map(|t| t.inner)
                .collect::<Vec<BinaryenType>>();
            Self {
                inner: BinaryenTypeCreate(
                    inners.as_mut_ptr(),
                    value_types.len().try_into().unwrap(),
                ),
                matchable_type: MType::Multi,
            }
        };
    }
    pub fn arity(&mut self) -> u32
    {
        unsafe { BinaryenTypeArity(self.inner) }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Features
{
    inner: BinaryenFeatures,
}

/*
        BinaryenFeatureMVP: 0
        BinaryenFeatureAtomics: 1
        BinaryenFeatureBulkMemory: 16
        BinaryenFeatureMutableGlobals: 2
        BinaryenFeatureNontrappingFPToInt: 4
        BinaryenFeatureSignExt: 32
        BinaryenFeatureSIMD128: 8
        BinaryenFeatureExceptionHandling: 64
        BinaryenFeatureTailCall: 128
        BinaryenFeatureReferenceTypes: 256
        BinaryenFeatureMultivalue: 512
        BinaryenFeatureGC: 1024
        BinaryenFeatureMemory64: 2048
        BinaryenFeatureAll: 4095
*/
macro_rules! impl_feature {
    ($name: ident, $check: ident) => {
        pub fn $name() -> i32
        {
            return unsafe { $check() } as i32;
        }
    };
}
impl Features
{
    impl_feature!(mvp, BinaryenFeatureMVP);
    impl_feature!(atomics, BinaryenFeatureAtomics);
    impl_feature!(bulk_memory, BinaryenFeatureBulkMemory);
    impl_feature!(mutable_globals, BinaryenFeatureMutableGlobals);
    impl_feature!(non_trapping_fp_to_int, BinaryenFeatureNontrappingFPToInt);
    impl_feature!(sign_ext, BinaryenFeatureSignExt);
    impl_feature!(simd_128, BinaryenFeatureSIMD128);
    impl_feature!(exception_handling, BinaryenFeatureExceptionHandling);
    impl_feature!(tail_call, BinaryenFeatureTailCall);
    impl_feature!(reference_types, BinaryenFeatureReferenceTypes);
    impl_feature!(multi_value, BinaryenFeatureMultivalue);
    impl_feature!(gc, BinaryenFeatureGC);
    impl_feature!(memory_64, BinaryenFeatureMemory64);
    impl_feature!(feature_all, BinaryenFeatureAll);
}
pub struct EventRef
{
    pub inner: BinaryenEventRef,
}
impl EventRef
{
    fn new(e: BinaryenEventRef) -> Self
    {
        Self { inner: e }
    }
}

pub struct FunctionRef
{
    inner: BinaryenFunctionRef,
}
impl FunctionRef
{
    fn new(e: BinaryenFunctionRef) -> Self
    {
        Self { inner: e }
    }
    pub fn get_name(&self) -> &str
    {
        let c_buf = unsafe { BinaryenFunctionGetName(self.inner) };
        let c_str = unsafe { std::ffi::CStr::from_ptr(c_buf) };
        c_str.to_str().unwrap()
    }
}

#[cfg(test)]
mod tests
{
    #![allow(
        dead_code,
        unused_variables,
        non_camel_case_types,
        non_upper_case_globals,
        non_snake_case
    )]

    use crate::*;
    use std::convert::TryInto;
    // use *;
    lazy_static! {
        static ref v128_byes: Vec<i128> =
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    }
    fn make_unary(module: &mut Module, op: Op, input_type: Type) -> ExpressionRef
    {
        let c = match input_type.matchable_type {
            MType::I32 => module.make_const(Literal::int_32(-10)),
            MType::I64 => module.make_const(Literal::int_64(-22)),
            MType::F32 => module.make_const(Literal::float_32(-33.612f32)),
            MType::F64 => module.make_const(Literal::float_64(-9005.841f64)),
            _ => {
                panic!();
            }
        };
        module.unary(op, c)
        // let c = if input_type == Type::int_32() {
        //     module.make_const(Literal::int_32(-10))
        // } else if input_type == Type::int_64() {
        //     module.make_const(Literal::int_64(-22))
        // } else if input_type == Type::float_32() {
        //     module.make_const(Literal::float_32(-33.612f32))
        // } else if input_type == Type::float_64() {
        //     module.make_const(Literal::float_64(-9005.841f64))
        // } else {
        //     //TODO: Add vec128
        //     // TODO: allow matching expressions so that I dont need this trailing else statement
        //     module.make_const(Literal::int_32(-0))
        // };
        // module.unary(op, c)
    }

    fn make_binary(module: &mut Module, op: Op, input_type: Type) -> ExpressionRef
    {
        if input_type == Type::int_32() {
            let temp = module.make_const(Literal::int_32(-11));
            // Rust limitation, Cannot borrow `module` as mutable more than once in the same line.
            let l = module.make_const(Literal::int_32(-10));
            return module.binary(op, l, temp);
        } else if input_type == Type::int_64() {
            let temp = module.make_const(Literal::int_64(-23));
            let l = module.make_const(Literal::int_64(-22));
            return module.binary(op, l, temp);
        } else if input_type == Type::float_32() {
            let temp = module.make_const(Literal::float_32(-65.5f32));
            let l = module.make_const(Literal::float_32(-33.612f32));
            return module.binary(op, l, temp);
        } else if input_type == Type::float_64() {
            let temp = module.make_const(Literal::float_64(-9007.333f64));
            let l = module.make_const(Literal::float_64(-9005.841f64));
            return module.binary(op, l, temp);
        } else {
            // TODO: allow matching expressions so that I dont need this trailing else statement
            let temp = module.make_const(Literal::int_32(-0));
            let l = module.make_const(Literal::int_32(-0));
            return module.binary(op, l, temp);
        }
    }

    fn make_int_32(module: &mut Module, x: i32) -> ExpressionRef
    {
        module.make_const(Literal::int_32(x))
    }
    fn make_int_64(module: &mut Module, x: i64) -> ExpressionRef
    {
        module.make_const(Literal::int_64(x))
    }
    fn make_float_32(module: &mut Module, x: f32) -> ExpressionRef
    {
        module.make_const(Literal::float_32(x))
    }
    fn make_float_64(module: &mut Module, x: f64) -> ExpressionRef
    {
        module.make_const(Literal::float_64(x))
    }
    fn make_something(module: &mut Module) -> ExpressionRef
    {
        make_int_32(module, 1337)
    }
    fn make_dropped_int_32(module: &mut Module, x: i32) -> ExpressionRef
    {
        let c = module.make_const(Literal::int_32(x));
        module.drop_var(c)
    }
    //TODO: Simd
    fn make_memory_init(module: &mut Module) -> ExpressionRef
    {
        let dest = make_int_32(module, 1024);
        let offset = make_int_32(module, 0);
        let size = make_int_32(module, 12);
        module.memory_init(0, dest, offset, size)
    }
    fn make_data_drop(module: &mut Module) -> ExpressionRef
    {
        module.data_drop(0)
    }

    fn make_memory_copy(module: &mut Module) -> ExpressionRef
    {
        let dest = make_int_32(module, 2048);
        let source = make_int_32(module, 1024);
        let size = make_int_32(module, 12);
        module.memory_copy(dest, source, size)
    }
    fn make_memory_fill(module: &mut Module) -> ExpressionRef
    {
        let dest = make_int_32(module, 2048);
        let source = make_int_32(module, 1024);
        let size = make_int_32(module, 12);
        module.memory_fill(dest, source, size)
    }
    #[test]
    fn test_types()
    {
        //TODO like 152 - 158 example
        //TODO: add expanding example
        {
            let mut unreachable = Type::unreachable();
            println!("  // BinaryenTypeUnreachable: {:?}\n", unreachable);
            assert!(unreachable.arity() == 1);
        }
        {
            let mut i32_ = Type::int_32();
            println!("  // BinaryenTypeInt32: {:?}\n", i32_);
            assert!(i32_.arity() == 1);
        }

        {
            let mut i64_ = Type::int_64();
            println!("  // BinaryenTypeInt64: {:?}\n", i64_);
            assert!(i64_.arity() == 1);
        }
        {
            let mut f32_ = Type::float_32();
            println!("  // BinaryenTypeFloat32: {:?}", f32_);
            assert!(f32_.arity() == 1);
        }
        {
            let mut f64_ = Type::float_32();
            println!("  // BinaryenTypeFloat64: {:?}", f64_);
            assert!(f64_.arity() == 1);
        }
        //TODO: Add v128 test
        {
            let mut funcref = Type::funcref();
            println!("  // BinaryenTypeFuncref: {:?}", funcref);
            assert!(funcref.arity() == 1);
        }
        {
            let mut externref = Type::externref();
            println!("  // BinaryenTypeExternref: {:?}", externref);
            assert!(externref.arity() == 1);
        }
        {
            let mut exnref = Type::exnref();
            println!("  // BinaryenTypeExnreff: {:?}", exnref);
            assert!(exnref.arity() == 1);
        }
        //TODO Eqref, and i31ref has bindings at line 105 binaryen-c.h, but its not in bindings.rs
        {
            let mut eqref = Type::eqref();
            println!("  // BinaryenTypeEqref: {:?}\n", eqref);
            assert!(eqref.arity() == 1);
        }

        let mut i31ref = Type::i31ref();
        println!("  // BinaryenTypeI31Ref: {:?}\n", i31ref);
        assert!(i31ref.arity() == 1);

        {
            println!("  // BinaryenTypeAuto: {:?}", Type::auto())
        }
        {
            let i32_0 = Type::int_32();
            let i32_1 = Type::int_32();

            let pair = vec![i32_0, i32_1];
            let mut i32_pair = Type::create(pair.clone());
            assert!(i32_pair.arity() == 2);
            //TODO: expand ln 239

            let duplicate_pair = Type::create(pair);
            assert!(duplicate_pair == i32_pair);
            let pair = vec![Type::float_32(), Type::float_32()];
            let float_pair = Type::create(pair);
            assert!(float_pair != i32_pair);
        }
    }
    #[test]
    fn test_features()
    {
        assert_eq!(Features::mvp(), 0);
        assert_eq!(Features::atomics(), 1);
        assert_eq!(Features::bulk_memory(), 16);
        assert_eq!(Features::mutable_globals(), 2);
        assert_eq!(Features::non_trapping_fp_to_int(), 4);
        assert_eq!(Features::sign_ext(), 32);
        assert_eq!(Features::simd_128(), 8);
        assert_eq!(Features::exception_handling(), 64);
        assert_eq!(Features::tail_call(), 128);
        assert_eq!(Features::reference_types(), 256);
        assert_eq!(Features::multi_value(), 512);
        assert_eq!(Features::gc(), 1024);
        assert_eq!(Features::feature_all(), 4095);

        //     println!("BinaryenFeatureMVP: {:?}", Features::mvp());
        //     println!("BinaryenFeatureAtomics: {:?}", Features::atomics());
        //     println!("BinaryenFeatureBulkMemory: {:?}", Features::bulk_memory());
    }
    #[test]
    fn test_core()
    {
        let mut module = Module::new();
        let (ConstI32, ConstI64, ConstF32, ConstF64, ConstF32Bits, ConstF64Bits) = (
            module.make_const(Literal::int_32(1)),
            module.make_const(Literal::int_64(2)),
            module.make_const(Literal::float_32(3.14f32)),
            module.make_const(Literal::float_64(2.1824f64)),
            module.make_const(Literal::float_32_bits(0xffff123)), //TODO: Figure out why this cant be 0xffff1234 like in the example (ln 279)
            module.make_const(Literal::float_64_bits(0xffff1234456abcdi64)),
        );
        let switch_value_names = vec!["the-value"];
        let switch_body_names = vec!["the-nothing"];

        let call_operands2 = vec![
            make_int_32(&mut module, 13),
            make_float_64(&mut module, 3.7),
        ];
        let call_operands4 = vec![
            make_int_32(&mut module, 13),
            make_int_64(&mut module, 37),
            make_float_32(&mut module, 1.3f32),
            make_float_64(&mut module, 3.7),
        ];
        let call_operands4b = vec![
            make_int_32(&mut module, 13),
            make_int_64(&mut module, 37),
            make_float_32(&mut module, 1.3f32),
            make_float_64(&mut module, 3.7),
        ];
        let tuple_elements4a = vec![
            make_int_32(&mut module, 13),
            make_int_64(&mut module, 37),
            make_float_32(&mut module, 1.3f32),
            make_float_64(&mut module, 3.7f64),
        ];

        let iIfF = vec![
            Type::int_32(),
            Type::int_64(),
            Type::float_32(),
            Type::float_64(),
        ];

        let iIfF = Type::create(iIfF);

        let (
            temp1,
            temp2,
            temp3,
            temp4,
            temp5,
            temp6,
            temp7,
            temp8,
            temp9,
            temp10,
            temp11,
            temp12,
            temp13,
            temp14,
            temp15,
            temp16,
        ) = (
            make_int_32(&mut module, 1),
            make_int_32(&mut module, 2),
            make_int_32(&mut module, 3),
            make_int_32(&mut module, 4),
            make_int_32(&mut module, 5),
            make_int_32(&mut module, 0),
            make_int_32(&mut module, 1),
            make_int_32(&mut module, 0),
            make_int_32(&mut module, 1),
            make_int_32(&mut module, 1),
            make_int_32(&mut module, 3),
            make_int_32(&mut module, 5),
            make_int_32(&mut module, 10),
            make_int_32(&mut module, 11),
            make_int_32(&mut module, 110),
            make_int_64(&mut module, 111),
        );
        let externrefExpr = module.ref_null(Type::externref());
        let mut funcrefExpr = module.ref_null(Type::funcref());
        funcrefExpr = module.ref_func("kitchen()sinker");
        let exnref = module.ref_null(Type::exnref());
        let i31ref = {
            let temp = make_int_32(&mut module, 1);
            module.make_i31(temp)
        };

        //Events
        module.add_event("a-event", 0, Type::int_32(), Type::none());

        let try_body = {
            let temp = vec![make_int_32(&mut module, 0)];
            module.throw("a-event", temp)
        };

        //TODO: Clean this up and put comments
        let catch_body = {
            // We have t ostart from the inside, so
            /*
              (BinaryenExpressionRef[]){
            BinaryenLocalSet(module, 5, BinaryenPop(module, BinaryenTypeExnref())),
            BinaryenDrop(
              module,
              BinaryenBlock(module,
                            "try-block",
                            (BinaryenExpressionRef[]){BinaryenRethrow(
                              module,
                              BinaryenBrOnExn(
                                module,
                                "try-block",
                                "a-event",
                                BinaryenLocalGet(module, 5, BinaryenTypeExnref())))},
                            1,
                            BinaryenTypeInt32()))}
                            */
            let popped = module.pop(Type::exnref());
            let lg = module.get_local(5, Type::exnref());
            // lg.print();
            let b = module.br_on_exn("try-block", "a-event", lg);
            let try_children = vec![module.rethrow(b)];
            let try_blk = module.new_block("try-block", try_children, Type::int_32());
            let children = vec![module.set_local(5, popped), module.drop_var(try_blk)];

            let catch_body = module.new_nameless_block(children, Type::none());
            catch_body

            // let pop = module.pop(Type::exnref());

            // let loca = module.get_local(5, Type::exnref());
            // let try_block_children = vec![module.br_on_exn("try-block", "a-event", loca)];
            // let blk = module.new_block("try-block", try_block_children, Type::int_32());
            // let mut children = vec![module.set_local(5, pop, ), module.drop_var(blk)];
            // // children.push();
            // module.new_nameless_block(children, Type::none())
        };

        let i32_ = Type::int_32();
        let i64_ = Type::int_64();
        let f32_ = Type::float_32();
        let f64_ = Type::float_64();
        macro_rules! binop {
            ($name: ident, $type: ident) => {
                make_binary(&mut module, Op::$name(), $type)
            };
        }
        macro_rules! unop {
            ($name: ident, $type: ident) => {
                make_unary(&mut module, Op::$name(), $type)
            };
        }
        let mut value_list = vec![
            unop!(clz_int32, i32_),
            unop!(ctz_int32, i32_),
            // //TODO: Fill the rest of the operators in
            binop!(add_int32, i32_),
            binop![add_int64, i64_],
            make_memory_init(&mut module),
            make_data_drop(&mut module),
            make_memory_copy(&mut module),
            make_memory_fill(&mut module),
            module.r#if(temp1, temp2, temp3),
            module.r#if(temp4, temp5, ExpressionRef::null_expr()),
            {
                let temp = make_int_32(&mut module, 0);
                module.r#loop("in", temp)
            },
            {
                let temp = make_int_32(&mut module, 0);
                module.r#loop("z", temp)
            },
            module.r#break("the-value", Some(temp6), Some(temp7)),
            {
                let temp = make_int_32(&mut module, 2);
                module.r#break("the-nothing", Some(temp), None)
            },
            {
                let temp = make_int_32(&mut module, 2);
                module.r#break("the-nothing", Some(temp), None)
            },
            {
                let temp = make_int_32(&mut module, 3);
                module.r#break("the-value", None, Some(temp))
            },
            {
                // let temp = make_int_32(&mut module, 3);
                module.r#break("the-nothing", None, None)
            },
            module.switch(switch_value_names, "the-value", temp8, temp9),
            {
                let temp = make_int_32(&mut module, 2);
                module.switch(
                    switch_body_names,
                    "the-nothing",
                    temp,
                    ExpressionRef::null_expr(),
                )
            },
            {
                let val = module.r#call("kitchen()sinker", call_operands4, Type::int_32());
                module.unary(Op::eq_z_int32(), val)
            },
            {
                let v = module.r#call("an-imported", call_operands2, Type::float_32());
                let val = module.unary(Op::trunc_s_float32_to_int32(), v);
                module.unary(Op::eq_z_int32(), val)
            },
            {
                let m = make_int_32(&mut module, 2449);
                let indirectly_called =
                    module.call_indirect(m, call_operands4b, iIfF, Type::int_32());
                module.unary(Op::eq_z_int32(), indirectly_called)
            },
            {
                let l0 = module.get_local(0, Type::int_32());
                module.drop_var(l0)
            },
            {
                let i = make_int_32(&mut module, 101);
                module.set_local(0, i)
            },
            {
                let i = make_int_32(&mut module, 102);
                module.tee_local(0, i, Type::int_32())
            },
            {
                //    BinaryenLoad(module, 4, 0, 0, 0, BinaryenTypeInt32(), makeInt32(module, 1)),
                let i = make_int_32(&mut module, 1);
                module.load(4, 0, 0, 0, Type::int_32(), i)
            },
            {
                //       BinaryenLoad(module, 2, 1, 2, 1, BinaryenTypeInt64(), makeInt32(module, 8)),
                let i = make_int_32(&mut module, 8);
                module.load(2, 1, 2, 1, Type::int_64(), i)
            },
            {
                //            module, 4, 0, 0, 0, BinaryenTypeFloat32(), makeInt32(module, 2)),
                let i = make_int_32(&mut module, 2);
                module.load(4, 0, 0, 0, Type::float_32(), i)
            },
            {
                //BinaryenLoad( module, 8, 0, 2, 8, BinaryenTypeFloat64(), makeInt32(module, 9)),
                let i = make_int_32(&mut module, 9);
                module.load(8, 0, 2, 8, Type::float_64(), i)
            },
            { module.store(4, 0, 0, temp13, temp14, Type::int_32()) },
            { module.store(8, 2, 4, temp15, temp16, Type::int_64()) },
            { module.select(temp10, temp11, temp12, Type::auto()) },
            {
                let i = make_int_32(&mut module, 1337);
                module.r#return(i)
            },
            {
                //TODO, find a way around redefining call_operands4
                let call_operands4 = vec![
                    make_int_32(&mut module, 13),
                    make_int_64(&mut module, 37),
                    make_float_32(&mut module, 1.3f32),
                    make_float_64(&mut module, 3.7),
                ];
                module.return_call("kitchen()sinker", call_operands4, Type::int_32())
            },
            {
                let call_operands4b = vec![
                    make_int_32(&mut module, 13),
                    make_int_64(&mut module, 37),
                    make_float_32(&mut module, 1.3f32),
                    make_float_64(&mut module, 3.7),
                ];
                let target = make_int_32(&mut module, 2499);
                module.return_call_indirect(target, call_operands4b, iIfF, Type::int_32())
                // call_operands4b,
            },
            //Reference types
            { module.ref_is_null(externrefExpr) },
            { module.ref_is_null(funcrefExpr) },
            { module.ref_is_null(exnref) },
            {
                let t = module.ref_null(Type::funcref());
                let f = module.ref_func("kitchen()sinker");
                module.select(temp10, t, f, Type::funcref())
            },
            //Gc
            {
                let l = module.ref_null(Type::eqref());
                let r = module.ref_null(Type::eqref());

                module.ref_eq(l, r)
            },
            //TODO: Exception handleing test
            //Exception handling
            // { module.r#try(try_body, catch_body) },
            //Atomics
            {
                let ld = module.atomic_load(4, 0, Type::int_32(), temp6);
                module.atomic_store(4, 0, temp6, ld, Type::int_32())
            },
            {
                let baw = module.atomic_wait(temp6, temp6, temp16, Type::int_32());
                module.drop_var(baw)
            },
            {
                let ban = module.atomic_notify(temp6, temp6);
                module.drop_var(ban)
            },
            { module.atomic_fence() },
            //tuples
            { module.make_tuple(tuple_elements4a) },
            {
                let tuple_elements4a = vec![
                    make_int_32(&mut module, 13),
                    make_int_64(&mut module, 37),
                    make_float_32(&mut module, 1.3f32),
                    make_float_64(&mut module, 3.7f64),
                ];
                let made = module.make_tuple(tuple_elements4a);
                module.extract_tuple(made, 2)
            },
            //pop
            { module.pop(Type::int_32()) },
            { module.pop(Type::int_64()) },
            { module.pop(Type::float_32()) },
            { module.pop(Type::float_64()) },
            { module.pop(Type::funcref()) },
            { module.pop(Type::externref()) },
            { module.pop(Type::exnref()) },
            //memory
            { module.memory_size() },
            {
                let i = make_int_32(&mut module, 2);
                module.memory_grow(i)
            },
            // Gc
            {
                let i = make_int_32(&mut module, 0);
                module.new_i31(i)
            },
            { module.get_i31(i31ref, 1) },
            {
                let i = make_int_32(&mut module, 2);
                let i31 = module.new_i31(i);
                module.get_i31(i31, 0)
            },
            // Other
            { module.nop() },
            { module.unreachable() },
        ];

        value_list.get(3).unwrap().print(); // test printing a standalone expression
                                            // Make the main body of the function. and one block with a return value, one without
        let value = module.new_block("the-value", value_list, Type::auto());
        let dropped_value = module.drop_var(value);
        let nothing = module.new_block("the-nothing", vec![dropped_value], Type::none());
        let body_list = vec![nothing, make_int_32(&mut module, 42)];
        let body = module.new_block("the-body", body_list, Type::auto());
        // Create the function
        let local_types = [Type::int_32(); 2].to_vec();
        let sinker =
            module.add_function("kitchen()sinker", iIfF, Type::int_32(), local_types, body);

        // Globals
        {
            let i = make_int_32(&mut module, 7);
            module.add_global("a-global", Type::int_32(), 0, i);
        }
        {
            let i = make_float_32(&mut module, 7.5f32);
            module.add_global("a-mutable-global", Type::float_32(), 0, i);
        }
        // Imports
        let iF = Type::create(vec![Type::int_32(), Type::float_64()]);
        module.add_function_import("an-imported", "module", "base", iF, Type::float_32());

        // Exports
        module.add_function_export("kitchen()sinker", "kitchen_sinker");

        // Function table. One per module
        let func_names = vec![sinker.get_name()];
        println!("func_names = {:#?}", func_names);

        {
            let offset = module.make_const(Literal::int_32(0));
            module.set_function_table(1, 1, func_names, offset);
        }
        //Memory.  One per module
        //TODO: Try to provide higher level api, something like abstracting away vec length.
        // let hw = std::ffi::CString::new("hello, world").unwrap();
        // let ip = std::ffi::CString::new("im passive").unwrap();
        let segments = vec!["hello, world", "im passive"];
        println!("segments = {:?}", segments);
        // panic!();
        let segment_passive: Vec<i8> = vec![0, 1];
        let segment_offsets = vec![
            module.make_const(Literal::int_32(10)),
            ExpressionRef::null_expr(),
        ];

        let segment_sizes = vec![12, 12];
        module.set_memory(
            1,
            256,
            "exported_mem",
            segments,
            segment_passive,
            segment_offsets,
            segment_sizes,
            true,
        );

        // Start function. One per module

        let starter = {
            let nop = module.nop();
            module.add_function("starter", Type::none(), Type::none(), vec![], nop)
        };

        module.set_start(starter);

        // A bunch of our code needs drop(), auto-add it
        module.auto_drop();
        let features = Features::feature_all();
        module.set_features(features);
        //TODO assert module.get_featres() == features

        module.print();

        assert!(module.validate());
        //Module is implicitly droped here v (see `module.drop`)
    } // <--------------------------------/
    #[test]
    pub fn test_unreachable()
    {
        let mut module = Module::new();
        let unr = module.unreachable();
        let body = module.call_indirect(unr, vec![], Type::none(), Type::int_64());
        let fn_ = module.add_function("unreachable-fn", Type::none(), Type::none(), vec![], body);
        assert!(module.validate());
        module.print();
        //Module is implicitly disposed
    }

    pub fn make_call_check(module: &mut Module, x: i32) -> ExpressionRef
    {
        let call_operands = vec![make_int_32(module, x)];
        module.call("check", call_operands, Type::none())
    }
    #[test]
    pub fn test_relooper()
    {
        let mut module = Module::new();

        module.add_function_import("check", "module", "check", Type::int_32(), Type::none());

        {
            // trivial: just one block
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block = relooper.add_block(make_call_check(&mut module, 1337));
            let body = relooper.render_and_dispose(block, 0);
            let sinker = module.add_function(
                "just-one-block",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // two blocks
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );

            let body = relooper.render_and_dispose(block0, 0);
            let sinker =
                module.add_function("two-block", Type::none(), Type::none(), local_types, body);
        }
        {
            // two blocks with code between them
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 77),
            );

            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function(
                "two-block-plus-code",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // two blocks in  a loop
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block0,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function("loop", Type::none(), Type::none(), local_types, body);
        }
        {
            // two blocks in  a loop with codes
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 33),
            );
            BRelooperRef::add_branch(
                &block1,
                &block0,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, -66),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function(
                "loop-plus-code",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // split
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                make_int_32(&mut module, 55),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );

            let body = relooper.render_and_dispose(block0, 0);
            let sinker =
                module.add_function("split", Type::none(), Type::none(), local_types, body);
        }
        {
            // split + code
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            let temp = make_dropped_int_32(&mut module, 10);
            BRelooperRef::add_branch(&block0, &block1, make_int_32(&mut module, 55), temp);
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 20),
            );

            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function(
                "split-plus-code",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // if + code
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            let temp = make_dropped_int_32(&mut module, -1);
            BRelooperRef::add_branch(&block0, &block1, make_int_32(&mut module, 55), temp);
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, -2),
            );
            BRelooperRef::add_branch(
                &block1,
                &block2,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, -3),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function(
                "if-plus-code",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // if
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                make_int_32(&mut module, 55),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function("if", Type::none(), Type::none(), local_types, body);
        }
        {
            // if-else
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            let block3 = relooper.add_block(make_call_check(&mut module, 3));

            BRelooperRef::add_branch(
                &block0,
                &block1,
                make_int_32(&mut module, 55),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block3,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block2,
                &block3,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker =
                module.add_function("if-else", Type::none(), Type::none(), local_types, body);
        }
        {
            // loop+tail
            let local_types = vec![Type::int_32()];
            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block0,
                make_int_32(&mut module, 10),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let snker =
                module.add_function("loop-tail", Type::none(), Type::none(), local_types, body);
        }
        {
            // nontrivial loop + phi to head
            let local_types = vec![Type::int_32()];
            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            let block3 = relooper.add_block(make_call_check(&mut module, 3));
            let block4 = relooper.add_block(make_call_check(&mut module, 4));
            let block5 = relooper.add_block(make_call_check(&mut module, 5));
            let block6 = relooper.add_block(make_call_check(&mut module, 6));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 10),
            );
            BRelooperRef::add_branch(
                &block1,
                &block2,
                make_int_32(&mut module, -2),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block6,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 20),
            );
            BRelooperRef::add_branch(
                &block2,
                &block3,
                make_int_32(&mut module, -6),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block2,
                &block1,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 30),
            );
            BRelooperRef::add_branch(
                &block3,
                &block4,
                make_int_32(&mut module, -10),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block3,
                &block5,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block4,
                &block5,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block5,
                &block6,
                ExpressionRef::null_expr(),
                make_dropped_int_32(&mut module, 40),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker = module.add_function(
                "non-trivial-loop-plus-phi-to-head",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            // switch
            let local_types = vec![Type::int_32()];
            let mut relooper = module.make_relooper();
            let temp = make_int_32(&mut module, -99);
            let block0 = relooper.add_block_with_switch(make_call_check(&mut module, 0), temp);
            // TODO: this example is not very good, the blocks should end in a |return| as otherwise they
            //       fall through to each other. A relooper should end in something that stops control
            //       flow, if it doesn't have branches going out
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            let block3 = relooper.add_block(make_call_check(&mut module, 3));
            let to_block1 = vec![2, 5];
            BRelooperRef::add_branch_for_switch(
                &block0,
                &block1,
                to_block1,
                ExpressionRef::null_expr(),
            );
            let to_block2 = vec![4];
            BRelooperRef::add_branch_for_switch(
                &block0,
                &block2,
                to_block2,
                make_dropped_int_32(&mut module, 55),
            );
            BRelooperRef::add_branch_for_switch(
                &block0,
                &block3,
                vec![],
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 0);
            let sinker =
                module.add_function("switch", Type::none(), Type::none(), local_types, body);
        }
        {
            // duff's device
            let local_types = vec![
                Type::int_32(),
                Type::int_32(),
                Type::int_64(),
                Type::int_32(),
                Type::float_32(),
                Type::float_64(),
                Type::int_32(),
            ];
            let mut relooper = module.make_relooper();
            let block0 = relooper.add_block(make_call_check(&mut module, 0));
            let block1 = relooper.add_block(make_call_check(&mut module, 1));
            let block2 = relooper.add_block(make_call_check(&mut module, 2));
            BRelooperRef::add_branch(
                &block0,
                &block1,
                make_int_32(&mut module, 10),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block0,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block1,
                &block2,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            BRelooperRef::add_branch(
                &block2,
                &block1,
                ExpressionRef::null_expr(),
                ExpressionRef::null_expr(),
            );
            let body = relooper.render_and_dispose(block0, 3); // use $3 as the helper var
            let sinker = module.add_function(
                "duffs-device",
                Type::none(),
                Type::none(),
                local_types,
                body,
            );
        }
        {
            //return in a block
            let local_types = vec![Type::int_32()];

            let mut relooper = module.make_relooper();
            let ret = make_int_32(&mut module, 1337);
            let list_list = vec![make_call_check(&mut module, 42), module.r#return(ret)];
            let list = module.new_block("the-lst", list_list, Type::none());
            let block = relooper.add_block(list);
            let body = relooper.render_and_dispose(block, 0);
            let sinker =
                module.add_function("return", Type::none(), Type::int_32(), local_types, body);
        }

        println!("raw:");
        module.print();
        assert!(module.validate())
        // assert!(module.validate());
    }
    //TODO: Test binaries
    // #[test]
    // pub fn test_binaryes() {
    //     let mut module = Module::new();

    //     let buffer = {
    //         let ii = Type::create(vec![Type::int_32(), Type::int_32()]);
    //         let x = module.get_local(0, Type::int_32());
    //         let y = module.get_local(1, Type::int_32());
    //         let add = module.binary(Op::add_int32(), x, y);
    //         let adder = module.add_function("adder", ii, Type::int_32(), vec![], add);
    //         //TODO BinaryenSetDebugInfo
    //         // module.compile()
    //         ""
    //     };
    //     let size = buffer.len();
    //     let compiled = module.compile();
    //     println!("size = {:?}\ncompiled = {:?}", compiled.len(), compiled);
    //     // std::fs::write("compiled_test", module.compile()).unwrap();
    //     module.print();
    //     assert!(size > 0);
    //     assert!(size < 512);
    // }

    fn main()
    {
        println!("You should run with `cargo test` from command line, not `cargo run` :)");
    }
}
