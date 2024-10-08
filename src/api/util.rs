use super::*;

/// Convert a 20 byte Account ID to an r-address
#[inline(always)]
pub fn util_raddr(raddr_out: &mut [u8], accid: &[u8]) -> Result<i64> {
    buf_write_read(raddr_out, accid, _c::util_raddr)
}

/// Convert an r-address into a 20 byte Account ID
#[inline(always)]
pub fn util_accid(accid_out: &mut [u8], raddr_in: &[u8]) -> Result<i64> {
    buf_write_read(accid_out, raddr_in, _c::util_accid)
}

/// Verify a cryptographic signature
#[inline(always)]
pub fn util_verify(payload: &[u8], signature: &[u8], publickey: &[u8]) -> bool {
    let res = buf_3_read(payload, signature, publickey, _c::util_verify);

    match res {
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}

/// Compute an sha512-half over some data
#[inline(always)]
pub fn util_sha512h(hash_out: &mut [u8], data_in: &[u8]) -> Result<i64> {
    buf_write_read(hash_out, data_in, _c::util_sha512h)
}

/// Compute a serialized keylet of a given type
#[inline(always)]
pub fn util_keylet(keylet: &mut [u8], keylet_type: KeyletType) -> Result<i64> {
    let write_ptr = keylet.as_mut_ptr() as _;
    let write_len = keylet.len() as _;

    match keylet_type {
        KeyletType::Hook(accid) => buf_read_and_zeroes(keylet, accid, _c::KEYLET_HOOK),

        KeyletType::HookState(accid, statekey) => {
            buf_2_read_and_zeroes(keylet, accid, statekey, _c::KEYLET_HOOK_STATE)
        }

        KeyletType::Account(accid) => buf_read_and_zeroes(keylet, accid, _c::KEYLET_ACCOUNT),

        KeyletType::Amendments => all_zeroes(keylet, _c::KEYLET_AMENDMENTS),

        KeyletType::Child(key) => buf_read_and_zeroes(keylet, key, _c::KEYLET_CHILD),

        KeyletType::Skip(opt) => match opt {
            None => all_zeroes(keylet, _c::KEYLET_SKIP),

            Some((ledger_index, num)) => {
                let res = unsafe {
                    _c::util_keylet(
                        write_ptr,
                        write_len,
                        _c::KEYLET_SKIP,
                        ledger_index,
                        num,
                        0,
                        0,
                        0,
                        0,
                    )
                };

                result_i64(res)
            }
        },

        KeyletType::Fees => all_zeroes(keylet, _c::KEYLET_FEES),

        KeyletType::NegativeUnl => all_zeroes(keylet, _c::KEYLET_NEGATIVE_UNL),

        KeyletType::Line(accid_high, accid_low, currency_code) => {
            let res = unsafe {
                _c::util_keylet(
                    write_ptr,
                    write_len,
                    _c::KEYLET_LINE,
                    accid_high.as_ptr() as _,
                    accid_high.len() as _,
                    accid_low.as_ptr() as _,
                    accid_low.len() as _,
                    currency_code.as_ptr() as _,
                    currency_code.len() as _,
                )
            };

            result_i64(res)
        }

        KeyletType::Offer(accid, num) => buf_read_and_1_arg(keylet, accid, num, _c::KEYLET_OFFER),

        KeyletType::Quality(serialized_keylet, bits_high, bits_low) => buf_read_and_2_args(
            keylet,
            serialized_keylet,
            bits_high,
            bits_low,
            _c::KEYLET_QUALITY,
        ),

        KeyletType::EmittedDir => all_zeroes(keylet, _c::KEYLET_EMITTED_DIR),

        KeyletType::Signers(accid) => buf_read_and_zeroes(keylet, accid, _c::KEYLET_SIGNERS),

        KeyletType::Check(accid, num) => buf_read_and_1_arg(keylet, accid, num, _c::KEYLET_CHECK),

        KeyletType::DepositPreauth(accid_1, accid_2) => {
            buf_2_read_and_zeroes(keylet, accid_1, accid_2, _c::KEYLET_DEPOSIT_PREAUTH)
        }

        KeyletType::Unchecked(key) => buf_read_and_zeroes(keylet, key, _c::KEYLET_UNCHECKED),

        KeyletType::OwnerDir(accid) => buf_read_and_zeroes(keylet, accid, _c::KEYLET_OWNER_DIR),

        KeyletType::Page(key, bits_high, bits_low) => {
            buf_read_and_2_args(keylet, key, bits_high, bits_low, _c::KEYLET_PAGE)
        }

        KeyletType::Escrow(accid, num) => buf_read_and_1_arg(keylet, accid, num, _c::KEYLET_ESCROW),

        KeyletType::Paychan(accid_1, accid_2, num) => {
            let res = unsafe {
                _c::util_keylet(
                    write_ptr,
                    write_len,
                    _c::KEYLET_PAYCHAN,
                    accid_1.as_ptr() as _,
                    accid_1.len() as _,
                    accid_2.as_ptr() as _,
                    accid_2.len() as _,
                    num,
                    0,
                )
            };

            result_i64(res)
        }

        KeyletType::EmittedTxn(key) => buf_read_and_zeroes(keylet, key, _c::KEYLET_EMITTED),

        KeyletType::NFTOffer(accid, num) => {
            buf_read_and_1_arg(keylet, accid, num, _c::KEYLET_NFT_OFFER)
        }

        KeyletType::HookDefinition(hash) => {
            buf_read_and_zeroes(keylet, hash, _c::KEYLET_HOOK_DEFINITION)
        }

        KeyletType::HookStateDir(accid, namespace) => {
            buf_2_read_and_zeroes(keylet, accid, namespace, _c::KEYLET_HOOK_STATE_DIR)
        }
    }
}
