use crate::api::*;

/// Tests two buffers for equality
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn is_buffer_equal<const GUARD_ID: u32>(buf_1: &[u8], buf_2: &[u8]) -> bool {
    let buf1_len = buf_1.len();

    if buf1_len != buf_2.len() {
        return false;
    };

    // guarded loop
    let mut i = 0;
    while {
        _g(GUARD_ID, buf1_len as u32 + 1);
        i < buf1_len
    } {
        if buf_1[i] != buf_2[i] {
            return false;
        }
        i += 1;
    }

    true
}

/// Zeroize a buffer
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn buffer_zeroize<const GUARD_ID: u32>(buf: &mut [u8]) {
    let buf_len = buf.len();
    // guarded loop
    let mut i = 0;
    while {
        _g(GUARD_ID, buf_len as u32 + 1);
        i < buf_len
    } {
        buf[0] = 0;
        i += 1;
    }
}

/// Checks whether the transaction is outgoing
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn is_txn_outgoing<const GUARD_ID: u32>(
    hook_acc_id: &mut AccountId,
    otnx_acc_id: &mut AccountId,
) -> Result<bool> {
    match hook_account(hook_acc_id) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    match otxn_field(otnx_acc_id, FieldId::Account) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    Ok(is_buffer_equal::<GUARD_ID>(
        &hook_acc_id[..],
        &otnx_acc_id[..],
    ))
}

/// Checks whether the transaction is ingoing
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn is_txn_ingoing<const GUARD_ID: u32>(
    hook_acc_id: &mut AccountId,
    otnx_acc_id: &mut AccountId,
) -> Result<bool> {
    match is_txn_outgoing::<GUARD_ID>(hook_acc_id, otnx_acc_id) {
        Err(e) => Err(e),
        Ok(res) => Ok(!res),
    }
}

/// Convert amount to drops
#[inline(always)]
pub const fn amount_to_drops(amount_buf: &NativeAmount) -> Result<u64> {
    if (amount_buf[0] >> 7) == 1 {
        return Err(Error::InternalError);
    }

    Ok((((amount_buf[0] as u64) & 0xb00111111) << 56)
        + ((amount_buf[1] as u64) << 48)
        + ((amount_buf[2] as u64) << 40)
        + ((amount_buf[3] as u64) << 32)
        + ((amount_buf[4] as u64) << 24)
        + ((amount_buf[5] as u64) << 16)
        + ((amount_buf[6] as u64) << 8)
        + (amount_buf[7] as u64))
}
