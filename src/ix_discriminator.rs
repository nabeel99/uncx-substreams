use anchor_lang::{AnchorDeserialize, Discriminator};
// use base64::engine::general_purpose::STANDARD;
use super::pb::uncx::{
    OnIncrementLock as pbIncrementLock, OnMigrate as pbMigrateLock, OnNewLock as pbNewLock,
    OnRelock as pbRelock, OnSplitLock as pbSplitLock,
    OnTransferLockOwnership as pbTransferLockOwnership, OnWithdraw as pbWithdrawLp,
};
use prost_types::Timestamp;
use uncx_solana_lp_locker::instruction::{
    CreateAndLockLp, IncrementLockLp, MigrateLp, RelockLp, SplitRelockLp, TransferLockOwnership,
    WithdrawLp,
};
use uncx_solana_lp_locker::{
    OnIncrementLock, OnMigrate, OnNewLock, OnRelock, OnSplitLock, OnTransferLockOwnership,
    OnWithdraw,
};
use super::pb::uncx::uncx_locker_event::Event as UncxProgramEvent;

// pub trait IxAnchorDiscriminatorForStructuredIx {
//     fn get_anchor_discriminator_bytes(&self) -> Result<[u8; 8],&'static str>;
// }
// impl IxAnchorDiscriminatorForStructuredIx for StructuredInstruction {
//     //TODO ADD Structured Error Handling
//     fn get_anchor_discriminator_bytes(&self) -> Result<[u8; 8],&'static str> {
//         self.data[..8].try_into().map_err(|_| "Failed to get discriminator")
//     }
// }

pub enum UncxProgramInstruction {
    CreateAndLockLp(CreateAndLockLp),
    RelockLp(RelockLp),
    SplitRelockLp(SplitRelockLp),
    TransferLockOwnership(TransferLockOwnership),
    WithdrawLp(WithdrawLp),
    MigrateLp(MigrateLp),
    IncrementLockLp(IncrementLockLp),
    // Initialize(Initialize),
    // AddWhitelist(AddWhitelist),
    // RemoveWhitelist(RemoveWhitelist),
    // AddMigrator(AddMigrator),
    // RemoveMigrator(RemoveMigrator),
    // SetDev(SetDev),
    // SetNewAdmin(SetNewAdmin),
    // SetNewFeesConfig(SetNewFeesConfig),
    // SetReferralTokenHoldBalance(SetReferralTokenHoldBalance),
    // SetSecondaryToken(SetSecondaryToken),
    // AddCountryToBlacklist(AddCountryToBlacklist),
    // RemoveCountryFromBlacklist(RemoveCountryFromBlacklist),
}
// pub enum Self {
//     OnNewLock(pbNewLock),
//     OnRelock(pbRelock),
//     OnWithdraw(pbWithdrawLp),
//     OnIncrementLock(pbIncrementLock),
//     OnSplitLock(pbSplitLock),
//     OnTransferLockOnwership(pbTransferLockOwnership),
//     OnMigrate(pbMigrateLock),
// }

impl From<OnNewLock> for pbNewLock {
    fn from(value: OnNewLock) -> Self {
        Self {
            lock_id: value.lock_id,
            amm_id: value.amm_id.to_string(),
            owner: value.owner.to_string(),
            amount: value.amount,
            lock_date: Timestamp {
                seconds: value.lock_date,
                nanos: 0,
            }
            .into(),
            unlock_date: Timestamp {
                seconds: value.unlock_date,
                nanos: 0,
            }
            .into(),
            country_code: value.country_code as u32,
        }
    }
}
impl From<OnMigrate> for pbMigrateLock {
    fn from(value: OnMigrate) -> Self {
        Self {
            lock_id: value.lock_id,
            owner: value.owner.to_string(),
            lp_token: value.lp_token.to_string(),
            amount_remaining_in_lock: value.amount_remaining_in_lock,
            amount_migrated: value.amount_migrated,
            migration_option: value.migration_option as u32,
        }
    }
}
impl From<OnRelock> for pbRelock {
    fn from(value: OnRelock) -> Self {
        Self {
            lock_id: value.lock_id,
            owner: value.owner.to_string(),
            lp_token: value.lp_token.to_string(),
            amount_remaining_in_lock: value.amount_remaining_in_lock,
            unlock_date: Timestamp {
                seconds: value.unlock_date,
                nanos: 0,
            }
            .into(),
            liquidity_fee: value.liquidity_fee,
        }
    }
}
impl From<OnIncrementLock> for pbIncrementLock {
    fn from(value: OnIncrementLock) -> Self {
        Self {
            lock_id: value.lock_id,
            owner: value.owner.to_string(),
            lp_token: value.lp_token.to_string(),
            amount_remaining_in_lock: value.amount_remaining_in_lock,
            liquidity_fee: value.liquidity_fee,
            amount_added: value.amount_added,
        }
    }
}
impl From<OnTransferLockOwnership> for pbTransferLockOwnership {
    fn from(value: OnTransferLockOwnership) -> Self {
        Self {
            lock_id: value.lock_id,
            lp_token: value.lp_token.to_string(),
            old_owner: value.old_owner.toString(),
            new_owner: value.new_owner.toString(),
        }
    }
}
impl From<OnSplitLock> for pbSplitLock {
    fn from(value: OnSplitLock) -> Self {
        Self {
            lock_id: value.lock_id,
            owner: value.owner.to_string(),
            lp_token: value.lp_token.to_string(),
            amount_remaining_in_old_lock: value.amount_remaining_in_old_lock,
            amount_removed: value.amount_removed,
        }
    }
}
impl From<OnWithdraw> for pbWithdrawLp {
    fn from(value: OnWithdraw) -> Self {
        Self {
            lock_id: value.lock_id,
            owner: value.owner.to_string(),
            lp_token: value.lp_token.to_string(),
            amount_removed: value.amount_removed,
            amount_remaining_in_lock: value.amount_remaining_in_lock,
        }
    }
}
impl UncxProgramInstruction {
    pub fn unpack(ix: &[u8]) -> Result<Self, &'static str> {
        let (sighash, ix_data) = ix.split_at(8);

        sighash
            .try_into()
            .map_err(|_| "failed to get discriminator bytes")
            .and_then(|discriminator: [u8; 8]| match discriminator {
                CreateAndLockLp::DISCRIMINATOR => CreateAndLockLp::try_from_slice(ix_data)
                    .map(Self::CreateAndLockLp)
                    .map_err(|_| "failed to deserialize CreateAndLockLp ix data"),
                RelockLp::DISCRIMINATOR => RelockLp::try_from_slice(ix_data)
                    .map(Self::RelockLp)
                    .map_err(|_| "failed to deserialize RelockLp ix data"),
                SplitRelockLp::DISCRIMINATOR => SplitRelockLp::try_from_slice(ix_data)
                    .map(Self::SplitRelockLp)
                    .map_err(|_| "failed to deserialize SplitRelockLp ix data"),
                IncrementLockLp::DISCRIMINATOR => IncrementLockLp::try_from_slice(ix_data)
                    .map(Self::IncrementLockLp)
                    .map_err(|_| "failed to deserialize IncrementLockLp ix data"),
                WithdrawLp::DISCRIMINATOR => WithdrawLp::try_from_slice(ix_data)
                    .map(Self::WithdrawLp)
                    .map_err(|_| "failed to deserialize WithdrawLp ix data"),
                MigrateLp::DISCRIMINATOR => MigrateLp::try_from_slice(ix_data)
                    .map(Self::MigrateLp)
                    .map_err(|_| "failed to deserialize MigrateLp ix data"),
                _ => Err("unknown instruction discriminator"),
            })
    }
}

impl UncxProgramEvent {
    pub fn unpack_event(ix: &[u8]) -> Result<Self, &'static str> {
        let (sighash, ix_data) = ix.split_at(8);

        sighash
            .try_into()
            .map_err(|_| "failed to get discriminator bytes")
            .and_then(|discriminator: [u8; 8]| match discriminator {
                OnNewLock::DISCRIMINATOR => OnNewLock::try_from_slice(ix_data)
                    .map(|event| Self::NewLock(pbNewLock::from(event)))
                    .map_err(|_| "failed to deserialize OnNewLock event data"),
                OnRelock::DISCRIMINATOR => OnRelock::try_from_slice(ix_data)
                    .map(|event| Self::Relock(pbRelock::from(event)))
                    .map_err(|_| "failed to deserialize OnRelock  event data"),
                OnSplitLock::DISCRIMINATOR => OnSplitLock::try_from_slice(ix_data)
                    .map(|event| Self::SplitLock(pbSplitLock::from(event)))
                    .map_err(|_| "failed to deserialize OnSplitLock event data"),
                OnIncrementLock::DISCRIMINATOR => OnIncrementLock::try_from_slice(ix_data)
                    .map(|event| Self::IncrementLock(pbIncrementLock::from(event)))
                    .map_err(|_| "failed to deserialize OnIncrementLockLp event data"),
                OnWithdraw::DISCRIMINATOR => OnWithdraw::try_from_slice(ix_data)
                    .map(|event| Self::WithdrawLp(pbWithdrawLp::from(event)))
                    .map_err(|_| "failed to deserialize OnWithdrawLp event data"),
                OnMigrate::DISCRIMINATOR => OnMigrate::try_from_slice(ix_data)
                    .map(|event| Self::MigrateLock(pbMigrateLock::from(event)))
                    .map_err(|_| "failed to deserialize OnMigrateLp event data"),
                _ => Err("unknown event discriminator"),
            })
    }
}
