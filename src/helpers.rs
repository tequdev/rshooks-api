use crate::api::*;
use byteorder::{ByteOrder, LittleEndian};

/// Rolls back the transaction if the condition is not met
///
/// # Arguments
/// * `cond` - The condition to check
/// * `message` - The error message to display on rollback
///
/// # Example
/// ```
/// require(amount > 0, b"Amount must be positive");
/// ```
#[cfg(not(doctest))]
#[inline(always)]
pub fn require(cond: bool, message: &[u8]) -> () {
    if !cond {
        rollback(message, 0);
    }
}

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

/// Tests two 20-byte buffers for equality
///
/// This function efficiently compares two 20-byte buffers by reading them as
/// little-endian integers. It's optimized for performance and avoids byte-by-byte
/// comparison.
///
/// # Arguments
/// * `buf_1` - The first 20-byte buffer to compare
/// * `buf_2` - The second 20-byte buffer to compare
///
/// # Returns
/// `true` if the buffers are equal, `false` otherwise
#[inline(always)]
pub fn is_buffer_equal_20(buf_1: &[u8], buf_2: &[u8]) -> bool {
    LittleEndian::read_u64(&buf_1[0..]) == LittleEndian::read_u64(&buf_2[0..])
        && LittleEndian::read_u64(&buf_1[8..]) == LittleEndian::read_u64(&buf_2[8..])
        && LittleEndian::read_u32(&buf_1[16..]) == LittleEndian::read_u32(&buf_2[16..])
}

/// Tests two 32-byte buffers for equality
///
/// This function efficiently compares two 32-byte buffers by reading them as
/// little-endian 64-bit integers. It's optimized for performance and avoids
/// byte-by-byte comparison.
///
/// # Arguments
/// * `buf_1` - The first 32-byte buffer to compare
/// * `buf_2` - The second 32-byte buffer to compare
///
/// # Returns
/// `true` if the buffers are equal, `false` otherwise
#[inline(always)]
pub fn is_buffer_equal_32(buf_1: &[u8], buf_2: &[u8]) -> bool {
    if LittleEndian::read_u64(&buf_1[0..]) == LittleEndian::read_u64(&buf_2[0..])
        && LittleEndian::read_u64(&buf_1[8..]) == LittleEndian::read_u64(&buf_2[8..])
        && LittleEndian::read_u64(&buf_1[16..]) == LittleEndian::read_u64(&buf_2[16..])
        && LittleEndian::read_u64(&buf_1[24..]) == LittleEndian::read_u64(&buf_2[24..])
    {
        return true;
    }

    return false;
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

    Ok(is_buffer_equal_20(&hook_acc_id[..], &otnx_acc_id[..]))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_buffer_equal_20_test() {
        const ACCOUNT_ID: AccountId = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        const ACCOUNT_ID_2: AccountId = [
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        const ACCOUNT_ID_3: [u8; 21] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ];
        assert_eq!(is_buffer_equal_20(&ACCOUNT_ID, &ACCOUNT_ID), true);
        assert_eq!(is_buffer_equal_20(&ACCOUNT_ID, &ACCOUNT_ID_2), false);
        assert_eq!(is_buffer_equal_20(&ACCOUNT_ID, &ACCOUNT_ID_3), true);
    }

    #[test]
    fn is_buffer_equal_32_test() {
        const DATA_1: Hash = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        const DATA_2: Hash = [
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        const DATA_3: [u8; 33] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33,
        ];
        assert_eq!(is_buffer_equal_32(&DATA_1, &DATA_1), true);
        assert_eq!(is_buffer_equal_32(&DATA_1, &DATA_2), false);
        assert_eq!(is_buffer_equal_32(&DATA_1, &DATA_3), true);
    }
}
