use anchor_lang::prelude::*;

declare_id!("3FFYCYGMqkjjpxMvGXu5XiRnZQtGJMN9r73Hh1yiBVjH");

#[program]
pub mod sol_strike {
    use super::*;

    pub fn buy_chip(ctx: Context<BuyChip>, amount: u64, payment_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64, receive_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn update_chip_price(ctx: Context<UpdatePrice>, new_price_sol: u64, new_token_prices: Vec<(Pubkey, u64)>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct State {
    pub sol_price: u64,
    pub token_prices: Vec<(Pubkey, u64)>
}

#[derive(Accounts)]
pub struct BuyChip {}

#[derive(Accounts)]
pub struct SellChip {}

#[derive(Accounts)]
pub struct UpdatePrice {}

