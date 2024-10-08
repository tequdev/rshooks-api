use super::*;

/// Get the burden of the originating transaction
#[inline(always)]
pub fn otxn_burden() -> i64 {
    unsafe { _c::otxn_burden() }
}

/// Serialize and output a field from the originating transaction
#[inline(always)]
pub fn otxn_field(data: &mut [u8], field_id: FieldId) -> Result<i64> {
    buf_write_1arg(data, field_id as _, _c::otxn_field)
}

/// Get the generation of the originating transaction
#[inline(always)]
pub fn otxn_generation() -> i64 {
    unsafe { _c::otxn_generation() }
}

/// Output the canonical hash of the originating transaction
#[inline(always)]
pub fn otxn_id(hash: &mut [u8], flags: TxnTypeFlags) -> Result<i64> {
    buf_write_1arg(hash, flags as u32, _c::otxn_id)
}

/// Get the Transaction Type of the originating transaction
#[inline(always)]
pub fn otxn_type() -> i64 {
    unsafe { _c::otxn_type() }
}

/// Load the originating transaction into a slot
#[inline(always)]
pub fn otxn_slot(slot_no: u32) -> Result<i64> {
    api_1arg_call(slot_no, _c::otxn_slot)
}
