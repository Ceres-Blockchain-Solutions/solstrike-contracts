use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, MintTo, Token, TokenAccount, Transfer}};

declare_id!("3FFYCYGMqkjjpxMvGXu5XiRnZQtGJMN9r73Hh1yiBVjH");

const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod sol_strike {
    use anchor_spl::token::TransferChecked;

    use super::*;

    pub fn initalize(ctx: Context<Initialize>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.bump = ctx.bumps.treasury;
        Ok(())
    }

    pub fn add_token(ctx: Context<AddToken>, token_address: Pubkey, token_price: u64) -> Result<()> {
        let chip_token_price_state = &mut ctx.accounts.chip_token_price_state;
        chip_token_price_state.token_address = token_address;
        chip_token_price_state.token_price = token_price;
        chip_token_price_state.bump = ctx.bumps.chip_token_price_state;
        Ok(())
    }

    pub fn buy_chip(ctx: Context<BuyChip>, amount: u64) -> Result<()> {
        let chip_token_price_state = &ctx.accounts.chip_token_price_state;
        let chip_price = chip_token_price_state.token_price;

        // calculate price in tokens which the buyer is paying with
        let total_payment = chip_price.checked_mul(amount).ok_or(Errors::Overflow)?;

        // debit buyers account and transfer to our treasury
        // CHECK if its done this way
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
            mint: ctx.accounts.payment_token_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts);
        // https://docs.rs/anchor-spl/latest/anchor_spl/token/fn.transfer_checked.html , decimals, da li nam treba? ako ne samo obican transfer
        token::transfer_checked(cpi_ctx, total_payment, 9)?;

        // mint the chips to user
        let mint_accounts = MintTo {
            mint: ctx.accounts.chip_mint.to_account_info(),
            to: ctx.accounts.buyer_chip_account.to_account_info(),
            authority: ctx.accounts.treasury.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_accounts);
        token::mint_to(cpi_ctx, amount)?;
    
        msg!(
            "✅ Successfully bought {} CHIP tokens for {} payment tokens.",
            amount,
            total_payment
        );

        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64, receive_token: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn update_chip_token_price(ctx: Context<UpdateChipTokenPrice>, new_token_price: u64) -> Result<()> {
        let chip_token_price_state = &mut ctx.accounts.chip_token_price_state;
        chip_token_price_state.token_price = new_token_price;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct ChipTokenPriceState {
    pub token_address: Pubkey,
    pub token_price: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub bump: u8
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + Treasury::INIT_SPACE,
        seeds = [b"TREASURY"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
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
        seeds = [b"TREASURY"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = payment_token_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub payment_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyChip<'info> {
    #[account(
        seeds = [b"CHIP_TOKEN_PRICE", payment_token_mint.key().as_ref()],
        bump = chip_token_price_state.bump
    )]
    pub chip_token_price_state: Account<'info, ChipTokenPriceState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::mint = payment_token_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>, 
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        mint::authority = treasury
    )]
    pub chip_mint: Account<'info, Mint>, 
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = chip_mint,
        associated_token::authority = buyer
    )]
    pub buyer_chip_account: Account<'info, TokenAccount>,
    // CHECK THE buyer_token_account constraints, 
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    pub payment_token_mint: Account<'info, Mint>,    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SellChip {}

#[derive(Accounts)]
pub struct UpdateChipTokenPrice<'info> {
    #[account(
        mut,
        seeds = [b"CHIP_TOKEN_PRICE", token_mint.key().as_ref()],
        bump = chip_token_price_state.bump
    )]
    pub chip_token_price_state: Account<'info, ChipTokenPriceState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_mint: Account<'info, Mint>
}

#[error_code]
pub enum Errors {
    Overflow,
}