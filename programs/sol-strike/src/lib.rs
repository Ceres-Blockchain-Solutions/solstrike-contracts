mod constants;

use crate::constants::*;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self, Burn, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};
use program::SolStrike;

declare_id!("F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA");

#[program]
pub mod sol_strike {
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, lamports_price: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.bump = ctx.bumps.treasury;

        let global_config = &mut ctx.accounts.global_config;
        global_config.lamports_chip_price = lamports_price;
        global_config.bump = ctx.bumps.global_config;

        Ok(())
    }

    pub fn buy_chip_with_sol(ctx: Context<BuyChipWithSol>, amount: u64) -> Result<()> {
        let global_config = &ctx.accounts.global_config;
        let treasury = &mut ctx.accounts.treasury;
        let chip_price = global_config.lamports_chip_price;

        let mut total_payment = chip_price.checked_mul(amount).ok_or(Errors::Overflow)?;
        total_payment = total_payment
            .checked_div(10_u64.checked_pow(CHIP_DECIMALS as u32).unwrap())
            .ok_or(Errors::Overflow)?;
        let total_payment_with_fee = apply_fee(total_payment, true)?;

        treasury.claimable_lamports += total_payment_with_fee - total_payment;

        let transfer_cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: treasury.to_account_info(),
            },
        );
        system_program::transfer(transfer_cpi_ctx, total_payment_with_fee)?;

        let chip_mint_seeds: &[&[u8]] = &[b"CHIP_MINT", &[ctx.bumps.chip_mint]];
        let signer_seeds: &[&[&[u8]]] = &[chip_mint_seeds];

        let mint_to_cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.chip_mint.to_account_info(),
                to: ctx.accounts.buyer_chip_account.to_account_info(),
                authority: ctx.accounts.chip_mint.to_account_info(),
            },
            signer_seeds,
        );
        token_interface::mint_to(mint_to_cpi_ctx, amount)?;
        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64) -> Result<()> {
        let global_config = &ctx.accounts.global_config;
        let treasury = &mut ctx.accounts.treasury;

        let chip_price = global_config.lamports_chip_price;

        let mut total_payment = chip_price.checked_mul(amount).ok_or(Errors::Overflow)?;
        total_payment = total_payment
            .checked_div(10_u64.checked_pow(CHIP_DECIMALS as u32).unwrap())
            .ok_or(Errors::Overflow)?;

        let total_payment_with_fee = apply_fee(total_payment, false)?;

        treasury.claimable_lamports += total_payment - total_payment_with_fee;

        let burn_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.chip_mint.to_account_info(),
                from: ctx.accounts.seller_chip_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        );
        token_interface::burn(burn_cpi_ctx, amount)?;

        **treasury.to_account_info().try_borrow_mut_lamports()? -= total_payment;
        **ctx
            .accounts
            .seller
            .to_account_info()
            .try_borrow_mut_lamports()? += total_payment;

        Ok(())
    }

    pub fn update_sol_chip_price(ctx: Context<UpdateSolChipPrice>, new_price: u64) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;
        global_config.lamports_chip_price = new_price;
        Ok(())
    }

    // reserved chips are transfered to the treasury ATA and saved in the DB
    // when user joins a game, reserved chips are deducted from the DB
    pub fn reserve_chips(ctx: Context<ReserveChips>, amount: u64) -> Result<()> {
        let transfer_checked_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.user_chip_account.to_account_info(),
                to: ctx.accounts.treasury_chip_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
                mint: ctx.accounts.chip_mint.to_account_info(),
            },
        );

        token_interface::transfer_checked(
            transfer_checked_cpi_ctx,
            amount,
            ctx.accounts.chip_mint.decimals,
        )?;

        emit!(ReserveChipsEvent {
            amount: amount,
            user: ctx.accounts.signer.key()
        });

        Ok(())
    }

    // updates the state of the ClaimableRewards PDA (how many chips can a user claim)
    // user will have a label to see how many chips can he claim and a button to claim
    pub fn set_claimable_rewards(ctx: Context<SetClaimableRewards>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let first_place_claimable_rewards = &mut ctx.accounts.first_place_claimable_rewards_account;

        let multiplier = 10_i32.checked_pow(CHIP_DECIMALS as u32).unwrap() as f64;

        first_place_claimable_rewards.amount += (FIRST_PRIZE * multiplier) as u64;

        let mut burn_amount: u64 = 0;

        if let Some(second_place_claimable_rewards) =
            ctx.accounts.second_place_claimable_rewards_account.as_mut()
        {
            second_place_claimable_rewards.amount += (SECOND_PRIZE * multiplier) as u64;
        } else {
            burn_amount += (SECOND_PRIZE * multiplier) as u64;
        }

        if let Some(third_place_claimable_rewards) =
            ctx.accounts.third_place_claimable_rewards_account.as_mut()
        {
            third_place_claimable_rewards.amount += (THIRD_PRIZE * multiplier) as u64;
        } else {
            burn_amount += (THIRD_PRIZE * multiplier) as u64;
        }

        if burn_amount != 0 {
            let treasury_seeds: &[&[u8]] = &[b"TREASURY", &[treasury.bump]];
            let signer_seeds: &[&[&[u8]]] = &[treasury_seeds];

            let burn_cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    from: ctx.accounts.treasury_chip_token_account.to_account_info(),
                    authority: treasury.to_account_info(),
                    mint: ctx.accounts.chip_mint.to_account_info(),
                },
                signer_seeds,
            );
            token_interface::burn(burn_cpi_ctx, burn_amount)?;
        }

        treasury.claimable_chips += (TREASURY_PRIZE * multiplier) as u64;

        Ok(())
    }

    pub fn claim_chips(ctx: Context<ClaimChips>) -> Result<()> {
        let claimable_rewards_account = &mut ctx.accounts.claimable_rewards_account;

        let treasury_seeds: &[&[u8]] = &[b"TREASURY", &[ctx.accounts.treasury.bump]];

        let signer_seeds: &[&[&[u8]]] = &[treasury_seeds];

        let transfer_checked_cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.treasury_chip_token_account.to_account_info(),
                to: ctx.accounts.claimer_chip_account.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
                mint: ctx.accounts.chip_mint.to_account_info(),
            },
            signer_seeds,
        );
        token_interface::transfer_checked(
            transfer_checked_cpi_ctx,
            claimable_rewards_account.amount,
            ctx.accounts.chip_mint.decimals,
        )?;

        emit!(ClaimChipsEvent {
            amount: claimable_rewards_account.amount,
            user: ctx.accounts.signer.key()
        });

        //user claimed rewards, reset them
        claimable_rewards_account.amount = 0;

        Ok(())
    }

    pub fn claim_platform_fees(ctx: Context<ClaimPlatfromFees>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;

        let treasury_seeds: &[&[u8]] = &[b"TREASURY", &[treasury.bump]];

        let signer_seeds: &[&[&[u8]]] = &[treasury_seeds];

        // claim chips
        let transfer_checked_cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.treasury_chip_token_account.to_account_info(),
                to: ctx.accounts.authority_chip_account.to_account_info(),
                authority: treasury.to_account_info(),
                mint: ctx.accounts.chip_mint.to_account_info(),
            },
            signer_seeds,
        );
        token_interface::transfer_checked(
            transfer_checked_cpi_ctx,
            treasury.claimable_chips,
            ctx.accounts.chip_mint.decimals,
        )?;

        // claim lamports
        **treasury.to_account_info().try_borrow_mut_lamports()? -= treasury.claimable_lamports;
        **ctx
            .accounts
            .authority
            .to_account_info()
            .try_borrow_mut_lamports()? += treasury.claimable_lamports;

        treasury.claimable_chips = 0;
        treasury.claimable_lamports = 0;

        Ok(())
    }

    pub fn migrate_treasury_to_v2(ctx: Context<MigrateTreasuryToV2>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let new_treasury_size = ANCHOR_DISCRIMINATOR + Treasury::INIT_SPACE;

        if treasury.data_len() == new_treasury_size {
            return err!(Errors::TreasuryAlreadyMigratedToV2);
        }

        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_treasury_size);

        let lamports_diff = new_minimum_balance.saturating_sub(treasury.lamports());
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &treasury.key(),
                lamports_diff,
            ),
            &[
                ctx.accounts.signer.to_account_info(),
                treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        treasury.realloc(new_treasury_size, false)?;

        let mut treasury_account_data = treasury.try_borrow_mut_data()?;

        let new_treasury = Treasury {
            claimable_lamports: 0,
            claimable_chips: 0,
            bump: treasury_account_data[ANCHOR_DISCRIMINATOR],
        };
        let new_treasury_vec = new_treasury.try_to_vec()?;

        treasury_account_data[8..new_treasury_vec.len() + 8].copy_from_slice(&new_treasury_vec);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    lamports_chip_price: u64,
    bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ClaimableRewards {
    pub amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub claimable_lamports: u64,
    pub claimable_chips: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = ANCHOR_DISCRIMINATOR + GlobalConfig::INIT_SPACE,
        payer = signer,
        seeds = [b"GLOBAL_CONFIG"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + Treasury::INIT_SPACE,
        seeds = [b"TREASURY"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = chip_mint,
        associated_token::authority = treasury,
        associated_token::token_program = token_program,
    )]
    pub treasury_chip_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, SolStrike>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(signer.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyChipWithSol<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        seeds = [b"GLOBAL_CONFIG"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        mut,
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = chip_mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellChip<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        seeds = [b"GLOBAL_CONFIG"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        mut,
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::mint = chip_mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program
    )]
    pub seller_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct UpdateSolChipPrice<'info> {
    #[account(
        mut,
        seeds = [b"GLOBAL_CONFIG"], 
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, SolStrike>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(signer.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetClaimableRewards<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, SolStrike>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(signer.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(
        mut,
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::authority = treasury,
        associated_token::mint = chip_mint,
        associated_token::token_program = token_program
    )]
    pub treasury_chip_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        space = ANCHOR_DISCRIMINATOR + ClaimableRewards::INIT_SPACE,
        payer = signer,
        seeds = [first_place_authority.key().as_ref()],
        bump
    )]
    pub first_place_claimable_rewards_account: Account<'info, ClaimableRewards>,
    /// CHECK: only an address that is the recipient, no need for checking
    pub first_place_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        space = ANCHOR_DISCRIMINATOR + ClaimableRewards::INIT_SPACE,
        payer = signer,
        seeds = [second_place_authority.as_ref().expect("Missing second_place_authority").key().as_ref()],
        bump
    )]
    pub second_place_claimable_rewards_account: Option<Account<'info, ClaimableRewards>>,
    /// CHECK: only an address thatJ is the recipient, no need for checking
    pub second_place_authority: Option<UncheckedAccount<'info>>,
    #[account(
        init_if_needed,
        space = ANCHOR_DISCRIMINATOR + ClaimableRewards::INIT_SPACE,
        payer = signer,
        seeds = [third_place_authority.as_ref().expect("Missing third_place_authority").key().as_ref()],
        bump
    )]
    pub third_place_claimable_rewards_account: Option<Account<'info, ClaimableRewards>>,
    /// CHECK: only an address that is the recipient, no need for checking
    pub third_place_authority: Option<UncheckedAccount<'info>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReserveChips<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = treasury,
        associated_token::mint = chip_mint,
        associated_token::token_program = token_program
    )]
    pub treasury_chip_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = chip_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ClaimChips<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub claimable_rewards_account: Account<'info, ClaimableRewards>,
    #[account(
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        associated_token::authority = treasury,
        associated_token::mint = chip_mint,
        associated_token::token_program = token_program
    )]
    pub treasury_chip_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = chip_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub claimer_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ClaimPlatfromFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, SolStrike>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(
        mut,
        seeds = [b"TREASURY"], 
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mint::authority = chip_mint,
        seeds = [b"CHIP_MINT"],
        bump
    )]
    pub chip_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = treasury,
        associated_token::mint = chip_mint,
        associated_token::token_program = token_program
    )]
    pub treasury_chip_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = chip_mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub authority_chip_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct MigrateTreasuryToV2<'info> {
    pub signer: Signer<'info>,

    /// CHECK: Manual check implemented in the instruction
    #[account(
        mut,
        seeds = [b"TREASURY"], 
        bump,
    )]
    pub treasury: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

/// Applies a fee by either adding or subtracting it.
/// Use add_fee = true for buying, false for selling
/// when buying round up, when selling round down
fn apply_fee(value: u64, add_fee: bool) -> Result<u64> {
    if add_fee {
        let numerator = value
            .checked_mul(CHIP_FEE_BASIS_DIVISOR + CHIP_FEE_BASIS_POINTS)
            .ok_or(Errors::Overflow)?;
        let rounded = numerator
            .checked_add(CHIP_FEE_BASIS_DIVISOR - 1)
            .ok_or(Errors::Overflow)?;
        rounded
            .checked_div(CHIP_FEE_BASIS_DIVISOR)
            .ok_or(Errors::Overflow.into())
    } else {
        let numerator = value
            .checked_mul(CHIP_FEE_BASIS_DIVISOR - CHIP_FEE_BASIS_POINTS)
            .ok_or(Errors::Overflow)?;
        numerator
            .checked_div(CHIP_FEE_BASIS_DIVISOR)
            .ok_or(Errors::Overflow.into())
    }
}

#[event]
pub struct ReserveChipsEvent {
    pub amount: u64,
    pub user: Pubkey,
}

#[event]
pub struct ClaimChipsEvent {
    pub amount: u64,
    pub user: Pubkey,
}

#[error_code]
pub enum Errors {
    Overflow,
    MissingSecondPlaceAccount,
    MissingThirdPlaceAccount,
    TreasuryAlreadyMigratedToV2,
}
