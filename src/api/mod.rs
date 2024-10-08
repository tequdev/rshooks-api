use crate::_c;

mod control;
mod etxn;
mod float;
mod ledger;
mod otxn;
mod slot;
mod state;
mod sto;
mod trace;
mod util;

pub use control::*;
pub use etxn::*;
pub use float::*;
pub use ledger::*;
pub use otxn::*;
pub use slot::*;
pub use state::*;
pub use sto::*;
pub use trace::*;
pub use util::*;

/// Flags canonical
pub const TF_CANONICAL: u32 = _c::tfCANONICAL;

/// Account id buffer lenght
pub const ACC_ID_LEN: usize = 20;
/// Public key buffer lenght
pub const PUB_KEY_LEN: usize = 33;
/// Currency code buffer lenght
pub const CURRENCY_CODE_SIZE: usize = 20;
/// Ledger hash buffer lenght
pub const LEDGER_HASH_LEN: usize = 32;
/// Keylet buffer lenght
pub const KEYLET_LEN: usize = 34;
/// State key buffer lenght
pub const STATE_KEY_LEN: usize = 32;
/// NameSpace buffer lenght
pub const NAMESPACE_LEN: usize = 32;
/// Nonce buffer lenght
pub const NONCE_LEN: usize = 32;
/// Hash buffer lenght
pub const HASH_LEN: usize = 32;
/// Native amount buffer lenght
pub const NATIVE_AMOUNT_LEN: usize = 8;
/// IOU amount buffer lenght
pub const IOU_AMOUNT_LEN: usize = 48;
/// Emit details buffer lenght
pub const EMIT_DETAILS_SIZE: usize = 105;

/// Buffer of the specified size
pub type Buffer<const T: usize> = [u8; T];

/// Account id buffer
pub type AccountId = Buffer<ACC_ID_LEN>;
/// Public key buffer
pub type PublicKey = Buffer<PUB_KEY_LEN>;
/// Hash buffer
pub type Hash = Buffer<HASH_LEN>;
/// Keylet buffer
pub type Keylet = Buffer<KEYLET_LEN>;
/// State key buffer
pub type StateKey = Buffer<STATE_KEY_LEN>;
/// Namespace key buffer
pub type NameSpace = Buffer<NAMESPACE_LEN>;
/// Nonce buffer
pub type Nonce = Buffer<NONCE_LEN>;
/// Native amount buffer
pub type NativeAmount = Buffer<NATIVE_AMOUNT_LEN>;
/// IOU amount buffer
pub type NonNativeAmount = Buffer<IOU_AMOUNT_LEN>;
/// Emit details buffer
pub type EmitDetails = Buffer<EMIT_DETAILS_SIZE>;
/// Currency code buffer
pub type CurrencyCode = Buffer<CURRENCY_CODE_SIZE>;

/// Transaction type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TxnType {
    Payment = _c::ttPAYMENT,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    RegularKeySet = 5,
    // NicknameSet = 6,
    OfferCreate = 7,
    OfferCancel = 8,
    TicketCreate = 10,
    // SpinalTap = 11,
    SignerListSet = 12,
    PaychanCreate = 13,
    PaychanFund = 14,
    PaychanClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    HookSet = 22,
    NFTokenMint = 25,
    NFTokenBurn = 26,
    NFTokenCreateOffer = 27,
    NFTokenCancelOffer = 28,
    NFTokenAcceptOffer = 29,
    URITokenMint = 45,
    URITokenBurn = 46,
    URITokenBuy = 47,
    URITokenCreateSellOffer = 48,
    URITokenCancelSellOffer = 49,
    Remit = 95,
    GenesisMint = 96,
    Import = 97,
    ClaimReward = 98,
    Invoke = 99,
    Amendment = 100,
    Fee = 101,
    UnlModify = 102,
}

/// Account type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AccountType {
    Account = _c::atACCOUNT,
    Owner = _c::atOWNER,
    Destination = _c::atDESTINATION,
    Issuer = _c::atISSUER,
    Authorize = _c::atAUTHORIZE,
    Unauthorize = _c::atUNAUTHORIZE,
    Target = _c::atTARGET,
    RegularKey = _c::atREGULARKEY,
    PseudoCallback = _c::atPSEUDOCALLBACK,
}

/// Amount type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AmountType {
    Amount = _c::amAMOUNT,
    Balance = _c::amBALANCE,
    LimitAmount = _c::amLIMITAMOUNT,
    TakerPays = _c::amTAKERPAYS,
    TakerGets = _c::amTAKERGETS,
    LowLimit = _c::amLOWLIMIT,
    HighLimit = _c::amHIGHLIMIT,
    Fee = _c::amFEE,
    SendMax = _c::amSENDMAX,
    DeliverMin = _c::amDELIVERMIN,
    MinimumOffer = _c::amMINIMUMOFFER,
    RippleEscrow = _c::amRIPPLEESCROW,
    DeliveredAmount = _c::amDELIVEREDAMOUNT,
}

/// Keylet type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum KeyletType<'a> {
    Hook(&'a AccountId),
    HookState(&'a AccountId, &'a StateKey),
    Account(&'a AccountId),
    Amendments,
    Child(&'a Hash),
    Skip(Option<(u32, u32)>),
    Fees,
    NegativeUnl,
    Line(&'a AccountId, &'a AccountId, &'a CurrencyCode),
    Offer(&'a AccountId, u32),
    Quality(&'a [u8], u32, u32),
    EmittedDir,
    Signers(&'a AccountId),
    Check(&'a AccountId, u32),
    DepositPreauth(&'a AccountId, &'a AccountId),
    Unchecked(&'a Hash),
    OwnerDir(&'a AccountId),
    Page(&'a Hash, u32, u32),
    Escrow(&'a AccountId, u32),
    Paychan(&'a AccountId, &'a AccountId, u32),
    EmittedTxn(&'a Hash),
    NFTOffer(&'a AccountId, u32),
    HookDefinition(&'a Hash),
    HookStateDir(&'a AccountId, &'a NameSpace),
}

/// Field or amount type
///
/// Used as return of [slot_type] function
#[derive(Clone, Copy)]
pub enum FieldOrXrpAmount {
    /// Field ID
    Field(FieldId),
    /// STI_AMOUNT type contains a native amount
    NativeAmount,
    /// STI_AMOUNT type contains non-native amount
    NonNativeAmount,
}

/// Flags for [slot_type]
#[derive(Clone, Copy)]
pub enum SlotTypeFlags {
    /// Field
    Field,
    /// STI_AMOUNT type contains a native amount
    NativeAmount,
}

/// Field type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum FieldId {
    CloseResolution = _c::sfCloseResolution,
    Method = _c::sfMethod,
    TransactionResult = _c::sfTransactionResult,
    TickSize = _c::sfTickSize,
    UNLModifyDisabling = _c::sfUNLModifyDisabling,
    HookResult = _c::sfHookResult,
    LedgerEntryType = _c::sfLedgerEntryType,
    TransactionType = _c::sfTransactionType,
    SignerWeight = _c::sfSignerWeight,
    TransferFee = _c::sfTransferFee,
    Version = _c::sfVersion,
    HookStateChangeCount = _c::sfHookStateChangeCount,
    HookEmitCount = _c::sfHookEmitCount,
    HookExecutionIndex = _c::sfHookExecutionIndex,
    HookApiVersion = _c::sfHookApiVersion,
    NetworkID = _c::sfNetworkID,
    Flags = _c::sfFlags,
    SourceTag = _c::sfSourceTag,
    Sequence = _c::sfSequence,
    PreviousTxnLgrSeq = _c::sfPreviousTxnLgrSeq,
    LedgerSequence = _c::sfLedgerSequence,
    CloseTime = _c::sfCloseTime,
    ParentCloseTime = _c::sfParentCloseTime,
    SigningTime = _c::sfSigningTime,
    Expiration = _c::sfExpiration,
    TransferRate = _c::sfTransferRate,
    WalletSize = _c::sfWalletSize,
    OwnerCount = _c::sfOwnerCount,
    DestinationTag = _c::sfDestinationTag,
    HighQualityIn = _c::sfHighQualityIn,
    HighQualityOut = _c::sfHighQualityOut,
    LowQualityIn = _c::sfLowQualityIn,
    LowQualityOut = _c::sfLowQualityOut,
    QualityIn = _c::sfQualityIn,
    QualityOut = _c::sfQualityOut,
    StampEscrow = _c::sfStampEscrow,
    BondAmount = _c::sfBondAmount,
    LoadFee = _c::sfLoadFee,
    OfferSequence = _c::sfOfferSequence,
    FirstLedgerSequence = _c::sfFirstLedgerSequence,
    LastLedgerSequence = _c::sfLastLedgerSequence,
    TransactionIndex = _c::sfTransactionIndex,
    OperationLimit = _c::sfOperationLimit,
    ReferenceFeeUnits = _c::sfReferenceFeeUnits,
    ReserveBase = _c::sfReserveBase,
    ReserveIncrement = _c::sfReserveIncrement,
    SetFlag = _c::sfSetFlag,
    ClearFlag = _c::sfClearFlag,
    SignerQuorum = _c::sfSignerQuorum,
    CancelAfter = _c::sfCancelAfter,
    FinishAfter = _c::sfFinishAfter,
    SignerListID = _c::sfSignerListID,
    SettleDelay = _c::sfSettleDelay,
    TicketCount = _c::sfTicketCount,
    TicketSequence = _c::sfTicketSequence,
    NFTokenTaxon = _c::sfNFTokenTaxon,
    MintedNFTokens = _c::sfMintedNFTokens,
    BurnedNFTokens = _c::sfBurnedNFTokens,
    HookStateCount = _c::sfHookStateCount,
    EmitGeneration = _c::sfEmitGeneration,
    LockCount = _c::sfLockCount,
    FirstNFTokenSequence = _c::sfFirstNFTokenSequence,
    XahauActivationLgrSeq = _c::sfXahauActivationLgrSeq,
    ImportSequence = _c::sfImportSequence,
    RewardTime = _c::sfRewardTime,
    RewardLgrFirst = _c::sfRewardLgrFirst,
    RewardLgrLast = _c::sfRewardLgrLast,
    IndexNext = _c::sfIndexNext,
    IndexPrevious = _c::sfIndexPrevious,
    BookNode = _c::sfBookNode,
    OwnerNode = _c::sfOwnerNode,
    BaseFee = _c::sfBaseFee,
    ExchangeRate = _c::sfExchangeRate,
    LowNode = _c::sfLowNode,
    HighNode = _c::sfHighNode,
    DestinationNode = _c::sfDestinationNode,
    Cookie = _c::sfCookie,
    ServerVersion = _c::sfServerVersion,
    NFTokenOfferNode = _c::sfNFTokenOfferNode,
    EmitBurden = _c::sfEmitBurden,
    HookInstructionCount = _c::sfHookInstructionCount,
    HookReturnCode = _c::sfHookReturnCode,
    ReferenceCount = _c::sfReferenceCount,
    AccountIndex = _c::sfAccountIndex,
    AccountCount = _c::sfAccountCount,
    RewardAccumulator = _c::sfRewardAccumulator,
    EmailHash = _c::sfEmailHash,
    TakerPaysCurrency = _c::sfTakerPaysCurrency,
    TakerPaysIssuer = _c::sfTakerPaysIssuer,
    TakerGetsCurrency = _c::sfTakerGetsCurrency,
    TakerGetsIssuer = _c::sfTakerGetsIssuer,
    LedgerHash = _c::sfLedgerHash,
    ParentHash = _c::sfParentHash,
    TransactionHash = _c::sfTransactionHash,
    AccountHash = _c::sfAccountHash,
    PreviousTxnID = _c::sfPreviousTxnID,
    LedgerIndex = _c::sfLedgerIndex,
    WalletLocator = _c::sfWalletLocator,
    RootIndex = _c::sfRootIndex,
    AccountTxnID = _c::sfAccountTxnID,
    NFTokenID = _c::sfNFTokenID,
    EmitParentTxnID = _c::sfEmitParentTxnID,
    EmitNonce = _c::sfEmitNonce,
    EmitHookHash = _c::sfEmitHookHash,
    BookDirectory = _c::sfBookDirectory,
    InvoiceID = _c::sfInvoiceID,
    // Nickname = _c::sfNickname,
    Amendment = _c::sfAmendment,
    HookOn = _c::sfHookOn,
    Digest = _c::sfDigest,
    Channel = _c::sfChannel,
    ConsensusHash = _c::sfConsensusHash,
    CheckID = _c::sfCheckID,
    ValidatedHash = _c::sfValidatedHash,
    PreviousPageMin = _c::sfPreviousPageMin,
    NextPageMin = _c::sfNextPageMin,
    NFTokenBuyOffer = _c::sfNFTokenBuyOffer,
    NFTokenSellOffer = _c::sfNFTokenSellOffer,
    HookStateKey = _c::sfHookStateKey,
    HookHash = _c::sfHookHash,
    HookNamespace = _c::sfHookNamespace,
    HookSetTxnID = _c::sfHookSetTxnID,
    OfferID = _c::sfOfferID,
    EscrowID = _c::sfEscrowID,
    URITokenID = _c::sfURITokenID,
    GovernanceFlags = _c::sfGovernanceFlags,
    GovernanceMarks = _c::sfGovernanceMarks,
    EmittedTxnID = _c::sfEmittedTxnID,
    Amount = _c::sfAmount,
    Balance = _c::sfBalance,
    LimitAmount = _c::sfLimitAmount,
    TakerPays = _c::sfTakerPays,
    TakerGets = _c::sfTakerGets,
    LowLimit = _c::sfLowLimit,
    HighLimit = _c::sfHighLimit,
    Fee = _c::sfFee,
    SendMax = _c::sfSendMax,
    DeliverMin = _c::sfDeliverMin,
    MinimumOffer = _c::sfMinimumOffer,
    RippleEscrow = _c::sfRippleEscrow,
    DeliveredAmount = _c::sfDeliveredAmount,
    NFTokenBrokerFee = _c::sfNFTokenBrokerFee,
    LockedBalance = _c::sfLockedBalance,
    BaseFeeDrops = _c::sfBaseFeeDrops,
    ReserveBaseDrops = _c::sfReserveBaseDrops,
    ReserveIncrementDrops = _c::sfReserveIncrementDrops,
    PublicKey = _c::sfPublicKey,
    MessageKey = _c::sfMessageKey,
    SigningPubKey = _c::sfSigningPubKey,
    TxnSignature = _c::sfTxnSignature,
    URI = _c::sfURI,
    Signature = _c::sfSignature,
    Domain = _c::sfDomain,
    FundCode = _c::sfFundCode,
    RemoveCode = _c::sfRemoveCode,
    ExpireCode = _c::sfExpireCode,
    CreateCode = _c::sfCreateCode,
    MemoType = _c::sfMemoType,
    MemoData = _c::sfMemoData,
    MemoFormat = _c::sfMemoFormat,
    Fulfillment = _c::sfFulfillment,
    Condition = _c::sfCondition,
    MasterSignature = _c::sfMasterSignature,
    UNLModifyValidator = _c::sfUNLModifyValidator,
    ValidatorToDisable = _c::sfValidatorToDisable,
    ValidatorToReEnable = _c::sfValidatorToReEnable,
    HookStateData = _c::sfHookStateData,
    HookReturnString = _c::sfHookReturnString,
    HookParameterName = _c::sfHookParameterName,
    HookParameterValue = _c::sfHookParameterValue,
    Blob = _c::sfBlob,
    Account = _c::sfAccount,
    Owner = _c::sfOwner,
    Destination = _c::sfDestination,
    Issuer = _c::sfIssuer,
    Authorize = _c::sfAuthorize,
    Unauthorize = _c::sfUnauthorize,
    RegularKey = _c::sfRegularKey,
    NFTokenMinter = _c::sfNFTokenMinter,
    EmitCallback = _c::sfEmitCallback,
    HookAccount = _c::sfHookAccount,
    Inform = _c::sfInform,
    Indexes = _c::sfIndexes,
    Hashes = _c::sfHashes,
    Amendments = _c::sfAmendments,
    NFTokenOffers = _c::sfNFTokenOffers,
    HookNamespaces = _c::sfHookNamespaces,
    URITokenIDs = _c::sfURITokenIDs,
    Paths = _c::sfPaths,
    TransactionMetaData = _c::sfTransactionMetaData,
    CreatedNode = _c::sfCreatedNode,
    DeletedNode = _c::sfDeletedNode,
    ModifiedNode = _c::sfModifiedNode,
    PreviousFields = _c::sfPreviousFields,
    FinalFields = _c::sfFinalFields,
    NewFields = _c::sfNewFields,
    TemplateEntry = _c::sfTemplateEntry,
    Memo = _c::sfMemo,
    SignerEntry = _c::sfSignerEntry,
    EmitDetails = _c::sfEmitDetails,
    Hook = _c::sfHook,
    Signer = _c::sfSigner,
    Majority = _c::sfMajority,
    DisabledValidator = _c::sfDisabledValidator,
    EmittedTxn = _c::sfEmittedTxn,
    HookExecution = _c::sfHookExecution,
    HookDefinition = _c::sfHookDefinition,
    HookParameter = _c::sfHookParameter,
    HookGrant = _c::sfHookGrant,
    GenesisMint = _c::sfGenesisMint,
    ActiveValidator = _c::sfActiveValidator,
    ImportVLKey = _c::sfImportVLKey,
    HookEmission = _c::sfHookEmission,
    MintURIToken = _c::sfMintURIToken,
    AmountEntry = _c::sfAmountEntry,
    Signers = _c::sfSigners,
    SignerEntries = _c::sfSignerEntries,
    Template = _c::sfTemplate,
    Necessary = _c::sfNecessary,
    Sufficient = _c::sfSufficient,
    AffectedNodes = _c::sfAffectedNodes,
    Memos = _c::sfMemos,
    NFTokens = _c::sfNFTokens,
    Majorities = _c::sfMajorities,
    DisabledValidators = _c::sfDisabledValidators,
    HookExecutions = _c::sfHookExecutions,
    HookParameters = _c::sfHookParameters,
    HookGrants = _c::sfHookGrants,
    GenesisMints = _c::sfGenesisMints,
    ActiveValidators = _c::sfActiveValidators,
    ImportVLKeys = _c::sfImportVLKeys,
    HookEmissions = _c::sfHookEmissions,
    Amounts = _c::sfAmounts,
}

/// Data representation
#[derive(Clone, Copy)]
pub enum DataRepr {
    /// As UTF-8
    AsUTF8 = 0,
    /// As hexadecimal
    AsHex = 1,
}

/// Flags for otxn_id\
/// If 0:\
/// Write the canonical hash of the originating transaction.\
///
/// If 1 AND the originating transaction is an EMIT_FAILURE:\
/// Write the canonical hash of the emitting transaction.\
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TxnTypeFlags {
    /// Write the canonical hash of the originating transaction.
    OriginatingTxn = 0,
    /// Write the canonical hash of the emitting transaction.
    EmittingTxnIfEmitFailure = 1,
}

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
//
/// This is simple version of Result type
/// to comply XRPL Hooks Webassembly restrictions
#[must_use]
pub enum Result<T> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(Error),
}

pub use self::Result::*;

#[allow(unused_attributes)]
#[must_use]
impl<T> Result<T> {
    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Rollbacks
    ///
    /// Rollbacks if the value is an [`Err`], with a rollback message and error code.
    #[inline(always)]
    pub fn expect(self, msg: &[u8]) -> T {
        match self {
            Err(e) => rollback(msg, e.code() as _),
            Ok(val) => val,
        }
    }

    /// Returns the contained [`Ok`] value, or a None if the value is an DOES_NOT_EXIST error.
    ///
    /// # Rollbacks
    ///
    /// Rollbacks if the value is an [`Err`] except for DOES_NOT_EXIST error.
    #[inline(always)]
    pub fn optional(self) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) if e.code() == Error::DoesntExist.code() => None,
            Err(e) => {
                rollback(b"error", e.code() as _);
            }
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may rollback, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly.
    ///
    /// # Rollbacks
    ///
    /// Rollbacks if the value is an [`Err`], with a "error" and error code provided by the
    /// [`Err`]'s value.
    #[inline(always)]
    pub fn unwrap(self) -> T {
        match self {
            Err(e) => rollback(b"error", e.code() as _),
            Ok(val) => val,
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value,
    /// without checking that the value is not an [`Err`].
    ///
    /// # Safety
    ///
    /// Calling this method on an [`Err`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline(always)]
    pub unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Ok(val) => val,
            // SAFETY: the safety contract must be upheld by the caller.
            Err(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Returns `true` if the result is [`Ok`].
    #[must_use]
    #[inline(always)]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, Ok(_))
    }

    /// Returns `true` if the result is [`Err`].
    #[must_use]
    #[inline(always)]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

/// Possible errors returned by Hook APIs.
///
/// Errors are global across all Hook APIs.
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Error {
    /// Non-negative return codes refer always to success and usually indicate the number of bytes written or events performed, depending on the specific API.
    // SUCCESS = _c::SUCCESS,
    /// A pointer or buffer length provided as a parameter described memory outside of the Hook's allowed memory region.
    OutOfBounds = _c::OUT_OF_BOUNDS,
    /// Reserved for internal invariant trips, generally unrelated to inputs.
    /// These should be reported with an issue.
    InternalError = _c::INTERNAL_ERROR,
    /// Attempted to set a parameter or value larger than the allowed space .
    TooBig = _c::TOO_BIG,
    /// The API was unable to produce output to the write_ptr because the specified write_len was too small
    TooSmall = _c::TOO_SMALL,
    /// The requested object or item wasn't found
    DoesntExist = _c::DOESNT_EXIST,
    /// The Hook attempted to allocate an item into a slot, but there were no slots free.
    /// To avoid ensure re-use of existing slots. The maximum number of slots is 255.
    NoFreeSlots = _c::NO_FREE_SLOTS,
    /// One or more of the parameters to the API were invalid according to the individual API's specification.
    InvalidArgument = _c::INVALID_ARGUMENT,
    /// Some APIs allow for a once-per-execution parameter to be set.
    /// A second attempt to set a once-per-execution parameter results in this error.
    AlreadySet = _c::ALREADY_SET,
    /// An API required the Hook to do something before the API is allowed to be called.
    /// Check the API's documentation.
    PrerequisiteNotMet = _c::PREREQUISITE_NOT_MET,
    /// During fee calculation if an absurdly large fee is calculated this error is returned.
    FeeTooLarge = _c::FEE_TOO_LARGE,
    /// An attempt to emit() a TXN was unsccessful for any of a number of reasons.
    /// Check the trace log of the rippled to which you are submitting the originating TXN.
    EmissionFailure = _c::EMISSION_FAILURE,
    /// A Hook may only use up to 256 calls to nonce() per execution.
    /// Further calls result in this error code.
    TooManyNonces = _c::TOO_MANY_NONCES,
    /// A Hook must declare ahead of time how many TXN it intends to emit().
    /// If it emits fewer than this many, this is allowed.
    /// If it emits more than this many this error is returned.
    TooManyEmittedTxn = _c::TOO_MANY_EMITTED_TXN,
    /// While Hooks is/was in development an API may return this if some or all of that API is planned but not yet implemented.
    NotImplemented = _c::NOT_IMPLEMENTED,
    /// An API which accepts a 20 byte Account ID may return this if, in its opinion, the Account ID was not valid for any reason.
    InvalidAccount = _c::INVALID_ACCOUNT,
    /// All loops inside a Hook must declare at the top of the loop, as the first non trivial instruction,
    /// before any branch instruction, the promised maximum number of iterations of the loop.
    /// If this promise is violated the hook terminates immediately with this error code.
    GuardViolation = _c::GUARD_VIOLATION,
    /// The requested serialized field could not be found in the specified object.
    InvalidField = _c::INVALID_FIELD,
    /// While parsing serialized content an error was encountered (typically indicating an invalidly serialized object).
    ParseError = _c::PARSE_ERROR,
    /// Used internally to communicate a rollback event.
    RcRollback = _c::RC_ROLLBACK,
    /// Used internally to communicate an accept event.
    RcAccept = _c::RC_ACCEPT,
    /// Specified keylet could not be found, or keylet is invalid
    NoSuchKeylet = _c::NO_SUCH_KEYLET,
    /// API was asked to assume object under analysis is an STArray but it was not.
    NotAnArray = _c::NOT_AN_ARRAY,
    /// API was asked to assume object under analysis is an STObject but it was not.
    NotAnObject = _c::NOT_AN_OBJECT,
    /// A floating point operation resulted in Not-A-Number or API call attempted to specify an XFL floating point number outside of the expressible range of XFL.
    InvalidFloat = _c::INVALID_FLOAT,
    /// API call would result in a division by zero, so API ended early.
    DivisionByZero = _c::DIVISION_BY_ZERO,
    /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    MantissaOversized = _c::MANTISSA_OVERSIZED,
    /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    MantissaUndersized = _c::MANTISSA_UNDERSIZED,
    /// When attempting to create an XFL the exponent must not exceed 80.
    ExponentOversized = _c::EXPONENT_OVERSIZED,
    /// When attempting to create an XFL the exponent must not be less than -96.
    ExponentUndersized = _c::EXPONENT_UNDERSIZED,
    /// A floating point operation done on an XFL resulted in a value larger than XFL format is able to represent.
    XflOverflow = _c::XFL_OVERFLOW,
    /// An API assumed an STAmount was an IOU when in fact it was XRP.
    NotIouAmount = _c::NOT_IOU_AMOUNT,
    /// An API assumed an STObject was an STAmount when in fact it was not.
    NotAnAmount = _c::NOT_AN_AMOUNT,
    /// An API would have returned a negative integer except that negative integers are reserved for error codes (i.e. what you are reading.)
    CantReturnNegative = _c::CANT_RETURN_NEGATIVE,
    /// Hook attempted to set foreign state but was not authorized to do so (grant was missing or invalid.)
    NotAuthorized = _c::NOT_AUTHORIZED,
    /// Hook previously received a NOT_AUTHORIZED return code and is not allowed to retry.
    PreviousFailurePreventsRetry = _c::PREVIOUS_FAILURE_PREVENTS_RETRY,
    /// Attempted to set a hook parameter for a later hook in the chain, but there are now too many parameters.
    TooManyParams = _c::TOO_MANY_PARAMS,
    /// Serialized transaction was not a valid transaction (usually because of a missing required field or data corruption / truncation.)
    InvalidTxn = _c::INVALID_TXN,
    /// Setting an additional state object on this account would cause the reserve requirements to exceed the account's balance.
    ReserveInssuficient = _c::RESERVE_INSUFFICIENT,
    /// Hook API would be forced to return a complex number, which it cannot do.
    ComplexNotSupported = _c::COMPLEX_NOT_SUPPORTED,
    /// Two arguments were required to be of the same type but are not.
    DoesNotMatch = _c::DOES_NOT_MATCH,
    /// The provided public key was not valid.
    InvalidKey = _c::INVALID_KEY,
    /// The buffer did not contain a nul terminated string.
    NotAString = _c::NOT_A_STRING,
    /// The writing pointer points to a buffer that overlaps with the reading pointer.
    MemOverlap = _c::MEM_OVERLAP,
    /// More than 5000 modified state entries in the combined hook chains
    TooManyStateModifications = _c::TOO_MANY_STATE_MODIFICATIONS,
    /// More than 256 namespaces on this account
    TooManyNamespaces = _c::TOO_MANY_NAMESPACES,
}

impl Error {
    #[inline(always)]
    fn from_code(code: i32) -> Self {
        unsafe { core::mem::transmute(code) }
    }

    /// Error code
    #[inline(always)]
    pub fn code(self) -> i32 {
        self as _
    }
}

type Api1ArgsU32 = unsafe extern "C" fn(u32) -> i64;
type Api2ArgsU32 = unsafe extern "C" fn(u32, u32) -> i64;
type Api3ArgsU32 = unsafe extern "C" fn(u32, u32, u32) -> i64;
type Api4ArgsU32 = unsafe extern "C" fn(u32, u32, u32, u32) -> i64;
type Api6ArgsU32 = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32) -> i64;

type BufWriter = Api2ArgsU32;
type BufReader = Api2ArgsU32;
type Buf2Reader = Api4ArgsU32;
type BufWriterReader = Api4ArgsU32;
type Buf3Reader = Api6ArgsU32;
type BufWriter1Arg = Api3ArgsU32;

#[inline(always)]
fn api_1arg_call(arg: u32, fun: Api1ArgsU32) -> Result<i64> {
    let res = unsafe { fun(arg) };

    result_i64(res)
}

#[inline(always)]
fn api_3arg_call(arg_1: u32, arg_2: u32, arg_3: u32, fun: Api3ArgsU32) -> Result<i64> {
    let res = unsafe { fun(arg_1, arg_2, arg_3) };

    result_i64(res)
}

#[inline(always)]
fn buf_write(buf_write: &mut [u8], fun: BufWriter) -> Result<i64> {
    let res = unsafe { fun(buf_write.as_mut_ptr() as u32, buf_write.len() as u32) };

    result_i64(res)
}

#[inline(always)]
fn buf_write_1arg(buf_write: &mut [u8], arg: u32, fun: BufWriter1Arg) -> Result<i64> {
    let res = unsafe { fun(buf_write.as_mut_ptr() as u32, buf_write.len() as u32, arg) };

    result_i64(res)
}

#[inline(always)]
fn buf_read(buf: &[u8], fun: BufReader) -> Result<i64> {
    let res = unsafe { fun(buf.as_ptr() as u32, buf.len() as u32) };

    result_i64(res)
}

#[inline(always)]
fn buf_2read(buf_1: &[u8], buf_2: &[u8], fun: Buf2Reader) -> Result<i64> {
    let res = unsafe {
        fun(
            buf_1.as_ptr() as u32,
            buf_1.len() as u32,
            buf_2.as_ptr() as u32,
            buf_2.len() as u32,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_write_read(buf_write: &mut [u8], buf_read: &[u8], fun: BufWriterReader) -> Result<i64> {
    let res = unsafe {
        fun(
            buf_write.as_mut_ptr() as u32,
            buf_write.len() as u32,
            buf_read.as_ptr() as u32,
            buf_read.len() as u32,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_3_read(
    buf_read_1: &[u8],
    buf_read_2: &[u8],
    buf_read_3: &[u8],
    fun: Buf3Reader,
) -> Result<i64> {
    let res = unsafe {
        fun(
            buf_read_1.as_ptr() as u32,
            buf_read_1.len() as u32,
            buf_read_2.as_ptr() as u32,
            buf_read_2.len() as u32,
            buf_read_3.as_ptr() as u32,
            buf_read_3.len() as u32,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn range_from_location(location: i64) -> core::ops::Range<usize> {
    let offset: i32 = (location >> 32) as _;
    let lenght: i32 = (location & 0xFFFFFFFF) as _;

    core::ops::Range {
        start: offset as _,
        end: (offset + lenght) as _,
    }
}

#[inline(always)]
fn all_zeroes(buf_write: &mut [u8], keylet_type_c: u32) -> Result<i64> {
    let res = unsafe {
        _c::util_keylet(
            buf_write.as_mut_ptr() as _,
            buf_write.len() as _,
            keylet_type_c,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_read_and_zeroes(buf_write: &mut [u8], buf_read: &[u8], keylet_type_c: u32) -> Result<i64> {
    let res = unsafe {
        _c::util_keylet(
            buf_write.as_mut_ptr() as _,
            buf_write.len() as _,
            keylet_type_c,
            buf_read.as_ptr() as _,
            buf_read.len() as _,
            0,
            0,
            0,
            0,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_read_and_1_arg(
    buf_write: &mut [u8],
    buf_read: &[u8],
    arg: u32,
    keylet_type_c: u32,
) -> Result<i64> {
    let res = unsafe {
        _c::util_keylet(
            buf_write.as_mut_ptr() as _,
            buf_write.len() as _,
            keylet_type_c,
            buf_read.as_ptr() as _,
            buf_read.len() as _,
            arg,
            0,
            0,
            0,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_read_and_2_args(
    buf_write: &mut [u8],
    buf_read: &[u8],
    arg_1: u32,
    arg_2: u32,
    keylet_type_c: u32,
) -> Result<i64> {
    let res = unsafe {
        _c::util_keylet(
            buf_write.as_mut_ptr() as _,
            buf_write.len() as _,
            keylet_type_c,
            buf_read.as_ptr() as _,
            buf_read.len() as _,
            arg_1,
            arg_2,
            0,
            0,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn buf_2_read_and_zeroes(
    buf_write: &mut [u8],
    buf_1_read: &[u8],
    buf_2_read: &[u8],
    keylet_type_c: u32,
) -> Result<i64> {
    let res = unsafe {
        _c::util_keylet(
            buf_write.as_mut_ptr() as _,
            buf_write.len() as _,
            keylet_type_c,
            buf_1_read.as_ptr() as _,
            buf_1_read.len() as _,
            buf_2_read.as_ptr() as _,
            buf_2_read.len() as _,
            0,
            0,
        )
    };

    result_i64(res)
}

#[inline(always)]
fn result_i64(res: i64) -> Result<i64> {
    match res {
        res if res >= 0 => Ok(res as _),
        _ => Err(Error::from_code(res as _)),
    }
}

#[inline(always)]
fn result_xfl(res: i64) -> Result<XFL> {
    match res {
        res if res >= 0 => Ok(XFL(res)),
        _ => Err(Error::from_code(res as _)),
    }
}
