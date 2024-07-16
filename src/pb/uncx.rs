// @generated
/// TODO rename to UNCX BLOCK EVENTS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UncxEvents {
    #[prost(message, repeated, tag="1")]
    pub raydium_transactions: ::prost::alloc::vec::Vec<RaydiumTransactionEvents>,
    #[prost(message, repeated, tag="2")]
    pub uncx_transactions: ::prost::alloc::vec::Vec<UncxTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UncxTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub transaction_index: u32,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<UncxLockerEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub transaction_index: u32,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<RaydiumEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UncxLockerEvent {
    #[prost(uint32, tag="1")]
    pub instruction_index: u32,
    /// TODO rename to uncx_locker_event
    #[prost(oneof="uncx_locker_event::Event", tags="2, 3, 4, 5, 6, 7, 8")]
    pub event: ::core::option::Option<uncx_locker_event::Event>,
}
/// Nested message and enum types in `UncxLockerEvent`.
pub mod uncx_locker_event {
    /// TODO rename to uncx_locker_event
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="2")]
        NewLock(super::OnNewLock),
        #[prost(message, tag="3")]
        SplitLock(super::OnSplitLock),
        #[prost(message, tag="4")]
        WithdrawLp(super::OnWithdraw),
        #[prost(message, tag="5")]
        IncrementLock(super::OnIncrementLock),
        #[prost(message, tag="6")]
        MigrateLock(super::OnMigrate),
        #[prost(message, tag="7")]
        TransferLockOwnership(super::OnTransferLockOwnership),
        #[prost(message, tag="8")]
        Relock(super::OnRelock),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumEvent {
    #[prost(uint32, tag="1")]
    pub instruction_index: u32,
    /// rename to raydium_event
    #[prost(oneof="raydium_event::Event", tags="2, 3, 4, 5")]
    pub event: ::core::option::Option<raydium_event::Event>,
}
/// Nested message and enum types in `RaydiumEvent`.
pub mod raydium_event {
    /// rename to raydium_event
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="2")]
        Initialize(super::InitializeEvent),
        #[prost(message, tag="3")]
        Deposit(super::DepositEvent),
        #[prost(message, tag="4")]
        Withdraw(super::WithdrawEvent),
        #[prost(message, tag="5")]
        Swap(super::SwapEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnNewLock {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub amm_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(message, optional, tag="5")]
    pub lock_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub unlock_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag="7")]
    pub country_code: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnRelock {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount_remaining_in_lock: u64,
    #[prost(uint64, tag="5")]
    pub liquidity_fee: u64,
    #[prost(message, optional, tag="6")]
    pub unlock_date: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnWithdraw {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount_remaining_in_lock: u64,
    #[prost(uint64, tag="5")]
    pub amount_removed: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnIncrementLock {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    ///    string entity = 4;
    #[prost(uint64, tag="5")]
    pub amount_remaining_in_lock: u64,
    #[prost(uint64, tag="6")]
    pub amount_added: u64,
    #[prost(uint64, tag="7")]
    pub liquidity_fee: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnSplitLock {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount_remaining_in_old_lock: u64,
    #[prost(uint64, tag="5")]
    pub amount_removed: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnTransferLockOwnership {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnMigrate {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount_remaining_in_lock: u64,
    #[prost(uint64, tag="5")]
    pub amount_migrated: u64,
    #[prost(uint32, tag="6")]
    pub migration_option: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_init_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_init_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_init_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub nonce: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_in: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint_out: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub amount_in: u64,
    #[prost(uint64, tag="6")]
    pub amount_out: u64,
}
// @@protoc_insertion_point(module)
