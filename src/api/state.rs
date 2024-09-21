use super::*;

/// Retrieve the data pointed to by a Hook State key and write it to an output buffer
#[inline(always)]
pub fn state(data: &mut [u8], key: &StateKey) -> Result<i64> {
    buf_write_read(data, key, _c::state)
}

/// Set the Hook State for a given key and value
#[inline(always)]
pub fn state_set(data: &[u8], key: &StateKey) -> Result<i64> {
    buf_2read(data, key, _c::state_set)
}

/// Retrieve the data pointed to, on another account, by a Hook State key and write it to an output buffer
#[inline(always)]
pub fn state_foreign(data: &mut [u8], key: &StateKey, namespace: &NameSpace, accid: &AccountId) -> Result<i64> {
    let res = unsafe {
        _c::state_foreign(
            data.as_mut_ptr() as u32,
            data.len() as u32,
            key.as_ptr() as u32,
            key.len() as u32,
            namespace.as_ptr() as u32,
            namespace.len() as u32,
            accid.as_ptr() as u32,
            accid.len() as u32,
        )
    };

    result_i64(res)
}

/// Set the Hook State on another account for a given key, value and namespace
#[inline(always)]
pub fn state_foreign_set(data: &[u8], key: &StateKey, namespace: &NameSpace, accid: &AccountId) -> Result<i64> {
    let res = unsafe {
        _c::state_foreign_set(
            data.as_ptr() as u32,
            data.len() as u32,
            key.as_ptr() as u32,
            key.len() as u32,
            namespace.as_ptr() as u32,
            namespace.len() as u32,
            accid.as_ptr() as u32,
            accid.len() as u32,
        )
    };

    result_i64(res)
}
