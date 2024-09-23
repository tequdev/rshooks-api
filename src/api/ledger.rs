use super::*;

/// Retreive the 20 byte Account ID the Hook is executing on
#[inline(always)]
pub fn hook_account(accid: &mut [u8]) -> Result<i64> {
    buf_write(accid, _c::hook_account)
}

/// Retreive the 32 byte namespace biased SHA512H of the currently executing Hook
#[inline(always)]
pub fn hook_hash(hash: &mut [u8], hook_no: i32) -> Result<i64> {
    let res = unsafe { _c::hook_hash(hash.as_ptr() as u32, hash.len() as u32, hook_no as i32) };

    result_i64(res)
}

/// Fetch the fee base of the current ledger
#[inline(always)]
pub fn fee_base() -> i64 {
    unsafe { _c::fee_base() }
}

/// Fetch the current ledger sequence number
#[inline(always)]
pub fn ledger_seq() -> i64 {
    unsafe { _c::ledger_seq() }
}

/// Retreive the 32 byte namespace biased SHA512H of the last closed ledger
#[inline(always)]
pub fn ledger_last_hash(hash: &mut [u8]) -> Result<i64> {
    buf_write(hash, _c::ledger_last_hash)
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn ledger_nonce(n: &mut [u8]) -> Result<i64> {
    buf_write(n, _c::ledger_nonce)
}
