use bs58;

// use pb::uncx;
use pb::uncx::raydium_event;
use pb::uncx::UncxLockerEvent;
use substreams::errors::Error;
// use substreams_database_change::pb::database::TableChange;
// use substreams_database_change::tables::Tables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana_structured_instructions::{
    get_structured_instructions, StructuredInstruction, StructuredInstructions,
};

use spl_token_substream;

use raydium_amm::instruction::AmmInstruction;
use raydium_amm::RAYDIUM_LIQUIDITY_POOL;
use substreams_solana_raydium_amm as raydium_amm;

use substreams_solana_utils::{ConfirmedTransactionExt, TransactionContext};
pub mod ix_discriminator;
pub mod pb;
mod utils;
use pb::uncx::uncx_locker_event::Event as UncxProgramEvent;
use pb::uncx::{
    DepositEvent, InitializeEvent, RaydiumEvent, RaydiumTransactionEvents, SwapEvent, UncxEvents,
    UncxTransactionEvents, WithdrawEvent,
};
use raydium_event::Event as RaydiumProgramEvent;
use utils::parse_uncx_logs;
pub const UNCX_LOCKER_ADDRESS: &str = "6SzgqHxLUTwVRH9DGgFToBVjcLGot6aqk7AtTjdNsx4Z";

#[substreams::handlers::map]
fn raydium_block_events(block: Block) -> Result<UncxEvents, Error> {
    let raydium_parsed_transactions = parse_block_for_raydium_events(&block);
    let uncx_parsed_transactions = parse_block_for_uncx_events(&block);
    Ok(UncxEvents {
        raydium_transactions: raydium_parsed_transactions,
        uncx_transactions: uncx_parsed_transactions,
    })
}

pub fn parse_block_for_raydium_events(block: &Block) -> Vec<RaydiumTransactionEvents> {
    let mut block_events: Vec<RaydiumTransactionEvents> = Vec::new();
    for (i, transaction) in block.transactions.iter().enumerate() {
        let events = parse_raydium_transaction(transaction);
        if !events.is_empty() {
            block_events.push(RaydiumTransactionEvents {
                signature: bs58::encode(transaction.signature()).into_string(),
                transaction_index: i as u32,
                events,
            });
        }
    }
    block_events
}
pub fn parse_block_for_uncx_events(block: &Block) -> Vec<UncxTransactionEvents> {
    let mut block_events: Vec<UncxTransactionEvents> = Vec::new();
    for (i, transaction) in block.transactions.iter().enumerate() {
        let events = parse_uncx_transaction(transaction);
        if !events.is_empty() {
            block_events.push(UncxTransactionEvents {
                signature: bs58::encode(transaction.signature()).into_string(),
                transaction_index: i as u32,
                events,
            });
        }
    }
    block_events
}

pub fn parse_raydium_transaction(transaction: &ConfirmedTransaction) -> Vec<RaydiumEvent> {
    let context = TransactionContext::construct(transaction);
    let mut events: Vec<RaydiumEvent> = Vec::new();
    let instructions = get_structured_instructions(transaction);

    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    for (i, instruction) in instructions.flattened().iter().enumerate() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize))
            .into_string()
            != RAYDIUM_LIQUIDITY_POOL
        {
            continue;
        }
        match parse_raydium_instruction(&instruction, &context) {
            Ok(Some(event)) => events.push(RaydiumEvent {
                instruction_index: i as u32,
                event: Some(event),
            }),
            Ok(None) => (),
            Err(error) => substreams::log::println(format!(
                "Failed to process instruction of transaction {}: {}",
                &context.signature, error
            )),
        }
    }
    events
}
pub fn parse_uncx_transaction(transaction: &ConfirmedTransaction) -> Vec<UncxLockerEvent> {
    let context = TransactionContext::construct(transaction);
    let mut events: Vec<UncxLockerEvent> = Vec::new();
    let instructions = get_structured_instructions(transaction);

    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    for (i, instruction) in instructions.flattened().iter().enumerate() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize))
            .into_string()
            != UNCX_LOCKER_ADDRESS
        {
            continue;
        }
        match parse_uncx_instruction(&instruction, &context) {
            Ok(Some(event)) => events.push(UncxLockerEvent {
                instruction_index: i as u32,
                event: Some(event),
            }),
            Ok(None) => (),
            Err(error) => substreams::log::println(format!(
                "Failed to process instruction of transaction {}: {}",
                &context.signature, error
            )),
        }
    }
    events
}

pub fn parse_raydium_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<Option<RaydiumProgramEvent>, String> {
    if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize))
        .into_string()
        != RAYDIUM_LIQUIDITY_POOL
    {
        return Err("Not a Raydium instruction.".to_string());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data)?;
    match unpacked {
        AmmInstruction::SwapBaseIn(_) | AmmInstruction::SwapBaseOut(_) => {
            let event = _parse_swap_instruction(instruction, context)?;
            Ok(Some(RaydiumProgramEvent::Swap(event)))
        }
        AmmInstruction::Initialize2(initialize) => {
            let event = _parse_initialize_instruction(instruction, context, initialize.nonce)?;
            Ok(Some(RaydiumProgramEvent::Initialize(event)))
        }
        AmmInstruction::Deposit(_deposit) => {
            let event = _parse_deposit_instruction(instruction, context)?;
            Ok(Some(RaydiumProgramEvent::Deposit(event)))
        }
        AmmInstruction::Withdraw(_withdraw) => {
            let event = _parse_withdraw_instruction(instruction, context)?;
            Ok(Some(RaydiumProgramEvent::Withdraw(event)))
        }
        _ => Ok(None),
    }
}
pub fn parse_uncx_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<Option<UncxProgramEvent>, String> {
    _parse_uncx_event_log_from_ix(instruction, context)
    // match unpacked {
    //     UncxProgramInstruction::CreateAndLockLp(create_and_lock_lp_ix) => {
    //         let event = _parse_swap_instruction(instruction, context)?;
    //         Ok(Some(UncxProgramEvent::Swap(event)))
    //     }
    //     UncxProgramInstruction::Initialize2(initialize) => {
    //         let event = _parse_initialize_instruction(instruction, context, initialize.nonce)?;
    //         Ok(Some(UncxProgramEvent::Initialize(event)))
    //     }
    //     UncxProgramInstruction::Deposit(_deposit) => {
    //         let event = _parse_deposit_instruction(instruction, context)?;
    //         Ok(Some(UncxProgramEvent::Deposit(event)))
    //     }
    //     UncxProgramInstruction::Withdraw(_withdraw) => {
    //         let event = _parse_withdraw_instruction(instruction, context)?;
    //         Ok(Some(UncxProgramEvent::Withdraw(event)))
    //     }
    //     _ => Ok(None),
    // }
}

fn _parse_uncx_event_log_from_ix(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<Option<UncxProgramEvent>, String> {
    parse_uncx_logs(&instruction.logs)
}

fn _parse_swap_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<SwapEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize))
        .into_string();
    let user = bs58::encode(
        context.get_account_from_index(*instruction.accounts.last().unwrap() as usize),
    )
    .into_string();

    let instructions_len = instruction.inner_instructions.len();
    let transfer_in = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 2],
        context,
    )?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 1],
        context,
    )?;

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;
    let mint_in = transfer_in.source.unwrap().mint;
    let mint_out = transfer_out.source.unwrap().mint;

    Ok(SwapEvent {
        amm,
        user,
        mint_in,
        mint_out,
        amount_in,
        amount_out,
    })
}

fn _parse_initialize_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    nonce: u8,
) -> Result<InitializeEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[4] as usize))
        .into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[17] as usize))
        .into_string();

    let instructions_len = instruction.inner_instructions.len();
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 3],
        context,
    )?;
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 2],
        context,
    )?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(
        &instruction.inner_instructions[instructions_len - 1],
        context,
    )?;

    let pc_init_amount = pc_transfer.amount;
    let coin_init_amount = coin_transfer.amount;
    let lp_init_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    Ok(InitializeEvent {
        amm,
        user,
        pc_init_amount,
        coin_init_amount,
        lp_init_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        nonce: nonce as u32,
    })
}

fn _parse_deposit_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<DepositEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize))
        .into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[12] as usize))
        .into_string();

    let instructions_len = instruction.inner_instructions.len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 2],
        context,
    )?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 3],
        context,
    )?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(
        &instruction.inner_instructions[instructions_len - 1],
        context,
    )?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    Ok(DepositEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
    })
}

fn _parse_withdraw_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<WithdrawEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize))
        .into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[16] as usize))
        .into_string();

    let instructions_len = instruction.inner_instructions.len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 2],
        context,
    )?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions[instructions_len - 3],
        context,
    )?;
    let lp_burn = spl_token_substream::parse_burn_instruction(
        &instruction.inner_instructions[instructions_len - 1],
        context,
    )?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_burn.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_burn.source.unwrap().mint;

    Ok(WithdrawEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
    })
}

// pub fn tables_changes(block: &Block) -> Result<Vec<TableChange>, substreams::errors::Error> {
//     let mut tables = Tables::new();
//     for transaction in parse_block(block) {
//         for event in transaction.events.iter() {
//             match &event.event {
//                 Some(Event::Swap(swap)) => {
//                     tables
//                         .create_row(
//                             "raydium_swap_events",
//                             [
//                                 ("signature", transaction.signature.clone()),
//                                 ("instruction_index", event.instruction_index.to_string()),
//                             ],
//                         )
//                         .set("transaction_index", transaction.transaction_index)
//                         .set("slot", block.slot)
//                         .set("amm", &swap.amm)
//                         .set("user", &swap.user)
//                         .set("amount_in", swap.amount_in)
//                         .set("amount_out", swap.amount_out)
//                         .set("mint_in", &swap.mint_in)
//                         .set("mint_out", &swap.mint_out);
//                 }
//                 Some(Event::Initialize(initialize)) => {
//                     tables
//                         .create_row(
//                             "raydium_initialize_events",
//                             [
//                                 ("signature", transaction.signature.clone()),
//                                 ("instruction_index", event.instruction_index.to_string()),
//                             ],
//                         )
//                         .set("transaction_index", transaction.transaction_index)
//                         .set("slot", block.slot)
//                         .set("amm", &initialize.amm)
//                         .set("user", &initialize.user)
//                         .set("pc_init_amount", initialize.pc_init_amount)
//                         .set("coin_init_amount", initialize.coin_init_amount)
//                         .set("lp_init_amount", initialize.lp_init_amount)
//                         .set("pc_mint", &initialize.pc_mint)
//                         .set("coin_mint", &initialize.coin_mint)
//                         .set("lp_mint", &initialize.lp_mint);
//                 }
//                 Some(Event::Deposit(deposit)) => {
//                     tables
//                         .create_row(
//                             "raydium_deposit_events",
//                             [
//                                 ("signature", transaction.signature.clone()),
//                                 ("instruction_index", event.instruction_index.to_string()),
//                             ],
//                         )
//                         .set("transaction_index", transaction.transaction_index)
//                         .set("slot", block.slot)
//                         .set("amm", &deposit.amm)
//                         .set("user", &deposit.user)
//                         .set("pc_amount", deposit.pc_amount)
//                         .set("coin_amount", deposit.coin_amount)
//                         .set("lp_amount", deposit.lp_amount)
//                         .set("pc_mint", &deposit.pc_mint)
//                         .set("coin_mint", &deposit.coin_mint)
//                         .set("lp_mint", &deposit.lp_mint);
//                 }
//                 Some(Event::Withdraw(withdraw)) => {
//                     tables
//                         .create_row(
//                             "raydium_withdraw_events",
//                             [
//                                 ("signature", transaction.signature.clone()),
//                                 ("instruction_index", event.instruction_index.to_string()),
//                             ],
//                         )
//                         .set("transaction_index", transaction.transaction_index)
//                         .set("slot", block.slot)
//                         .set("amm", &withdraw.amm)
//                         .set("user", &withdraw.user)
//                         .set("pc_amount", withdraw.pc_amount)
//                         .set("coin_amount", withdraw.coin_amount)
//                         .set("lp_amount", withdraw.lp_amount)
//                         .set("pc_mint", &withdraw.pc_mint)
//                         .set("coin_mint", &withdraw.coin_mint)
//                         .set("lp_mint", &withdraw.lp_mint);
//                 }
//                 None => (),
//             }
//         }
//     }
//     Ok(tables.to_database_changes().table_changes)
// }
