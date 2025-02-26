use anchor_lang::{prelude::*, solana_program::native_token::{sol_to_lamports, LAMPORTS_PER_SOL}};

declare_id!("3FFYCYGMqkjjpxMvGXu5XiRnZQtGJMN9r73Hh1yiBVjH");

#[constant]
const CHIP_PRICE :u64 = (LAMPORTS_PER_SOL as f64 * 0.01) as u64;

#[program]
pub mod sol_strike {
    use super::*;

    pub fn add_token(ctx: Context<AddToken>) -> Result<()> {
        Ok(())
    }

    pub fn buy_chip(ctx: Context<BuyChip>, amount: u64, payment_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64, receive_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn update_chip_lamports_price(ctx: Context<UpdateChipLamportsPrice>, new_lamports_price: u64) -> Result<()> {
        Ok(())
    }

    pub fn update_chip_token_price(ctx: Context<UpdateChipTokenPrice>, token_address: Pubkey, new_token_price: u64) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct GlobalConfigState {
    pub lamport_price: u64,
}

#[account]
pub struct ChipTokenPriceState {
    pub token_address: Pubkey,
    pub token_price: u64
}

#[derive(Accounts)] 
pub struct AddToken {}

#[derive(Accounts)]
pub struct BuyChip {}

#[derive(Accounts)]
pub struct SellChip {}

#[derive(Accounts)]
pub struct UpdateChipTokenPrice {}

#[derive(Accounts)]
pub struct UpdateChipLamportsPrice {}