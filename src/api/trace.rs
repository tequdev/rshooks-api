use super::*;

/// Write the contents of a buffer to the XRPLD trace log
#[inline(always)]
pub fn trace(msg: &[u8], data: &[u8], data_repr: DataRepr) -> Result<i64> {
    let res = unsafe {
        _c::trace(
            msg.as_ptr() as u32,
            msg.len() as u32,
            data.as_ptr() as u32,
            data.len() as u32,
            data_repr as _,
        )
    };

    result_i64(res)
}

/// Write an integer to the XRPLD trace log
#[inline(always)]
pub fn trace_num(msg: &[u8], number: i64) -> Result<i64> {
    let res = unsafe { _c::trace_num(msg.as_ptr() as u32, msg.len() as u32, number) };

    result_i64(res)
}

/// Write a XFL float to the XRPLD trace log
#[inline(always)]
pub fn trace_float(msg: &[u8], float: XFL) -> Result<i64> {
    let res = unsafe { _c::trace_float(msg.as_ptr() as u32, msg.len() as u32, float.0) };

    result_i64(res)
}
