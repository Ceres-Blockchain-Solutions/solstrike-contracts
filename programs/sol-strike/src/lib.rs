use anchor_lang::{prelude::*, solana_program::native_token::{LAMPORTS_PER_SOL}};

declare_id!("3FFYCYGMqkjjpxMvGXu5XiRnZQtGJMN9r73Hh1yiBVjH");

const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
const CHIP_PRICE :u64 = (LAMPORTS_PER_SOL as f64 * 0.01) as u64;

#[program]
pub mod sol_strike {
    use super::*;

    pub fn initalize_global_config(ctx: Context<InitializeGlobalConfig>, lamport_price: u64) -> Result<()> {
        let global_config_state = &mut ctx.accounts.global_config;
        global_config_state.lamport_price = lamport_price;
        global_config_state.bump = ctx.bumps.global_config;
        Ok(())
    }

    pub fn add_token(ctx: Context<AddToken>, token_address: Pubkey, token_price: u64) -> Result<()> {
        let chip_token_price_state = &mut ctx.accounts.chip_token_price_state;
        chip_token_price_state.token_address = token_address;
        chip_token_price_state.token_price = token_price;
        chip_token_price_state.bump = ctx.bumps.chip_token_price_state;
        Ok(())
    }

    pub fn buy_chip(ctx: Context<BuyChip>, amount: u64, payment_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64, receive_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn update_chip_lamports_price(ctx: Context<UpdateChipLamportsPrice>, new_lamports_price: u64) -> Result<()> {
        let global_config_state = &mut ctx.accounts.global_config;
        global_config_state.lamport_price = new_lamports_price;
        Ok(())
    }

    pub fn update_chip_token_price(ctx: Context<UpdateChipTokenPrice>, token_address: Pubkey, new_token_price: u64) -> Result<()> {
        let chip_token_price_state = &mut ctx.accounts.chip_token_price_state;
        chip_token_price_state.token_price = new_token_price;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct GlobalConfigState {
    pub lamport_price: u64,
    pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct ChipTokenPriceState {
    pub token_address: Pubkey,
    pub token_price: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + GlobalConfigState::INIT_SPACE,
        seeds = [b"GLOBAL_CONFIG"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfigState>,
    #[account(
        mut
    )]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)] 
#[instruction(token_address: Pubkey)]
pub struct AddToken<'info> {
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + ChipTokenPriceState::INIT_SPACE,
        seeds = [b"CHIP_TOKEN_PRICE", token_address.key().as_ref()],
        bump
    )]
    pub chip_token_price_state: Account<'info, ChipTokenPriceState>,
    #[account(
        mut
    )]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyChip {}

#[derive(Accounts)]
pub struct SellChip {}

#[derive(Accounts)]
#[instruction(token_address: Pubkey)]
pub struct UpdateChipTokenPrice<'info> {
    #[account(
        mut,
        seeds = [b"CHIP_TOKEN_PRICE", token_address.key().as_ref()],
        bump = chip_token_price_state.bump
    )]
    pub chip_token_price_state: Account<'info, ChipTokenPriceState>,
    #[account(
        mut
    )]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct UpdateChipLamportsPrice<'info> {
    #[account(
        mut,
        seeds = [b"GLOBAL_CONFIG"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfigState>,
    #[account(
        mut
    )]
    pub signer: Signer<'info>
}