use anchor_lang::{
    prelude::*, system_program
};
use anchor_spl::token_interface::{
    self, Mint, MintTo, TokenAccount, TokenInterface, Burn, TransferChecked
};
use anchor_spl::associated_token::AssociatedToken;
use program::SolStrike;

declare_id!("6DpfdoF5HV6W8HG3tewwGnQRyFbR8muKGS144HgfAVER");

const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod sol_strike {
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

        let chip_price = global_config.lamports_chip_price;

        let total_payment = chip_price.checked_mul(amount).ok_or(Errors::Overflow)?;

        let transfer_cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
            },
        );
        system_program::transfer(transfer_cpi_ctx, total_payment)?;

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

        msg!(
            "Successfully bought {} CHIPs for {} lamports.",
            amount,
            total_payment
        );

        Ok(())
    }

    pub fn sell_chip(ctx: Context<SellChip>, amount: u64) -> Result<()> {
        let global_config = &ctx.accounts.global_config;

        let chip_price = global_config.lamports_chip_price;

        let total_payment = chip_price.checked_mul(amount).ok_or(Errors::Overflow)?;

        let burn_accounts = Burn {
            mint: ctx.accounts.chip_mint.to_account_info(),
            from: ctx.accounts.seller_chip_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            burn_accounts,
        );
        token_interface::burn(cpi_ctx, amount)?;

        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= total_payment;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += total_payment;    

        Ok(())
    }

    pub fn update_sol_chip_price(ctx: Context<UpdateSolChipPrice>, new_price: u64) -> Result<()> {
        let global_config = &mut ctx.accounts.global_config;
        global_config.lamports_chip_price = new_price;
        Ok(())
    }

    // updates the state of the ClaimableRewards PDA (how many chips can a user claim)
    // user will have a label to see how many chips can he calim and a button to claim
    pub fn distribute_chips(ctx: Context<DistributeChips>) -> Result<()> {
        let first_place_claimable_rewards = &mut ctx.accounts.first_place_claimable_rewards_account;
        let second_place_claimable_rewards = &mut ctx.accounts.second_place_claimable_rewards_account;
        let third_place_claimable_rewards = &mut ctx.accounts.third_place_claimable_rewards_account;

        first_place_claimable_rewards.amount += (2.5 * 1_000_000_000.0) as u64;
        second_place_claimable_rewards.amount += (1.0 * 1_000_000_000.0) as u64;
        third_place_claimable_rewards.amount += (0.3 * 1_000_000_000.0) as u64;

        Ok(())
    }

    //reserved chips are transfered to the treasury ATA and saved in the DB
    //when user joins a game, reserved chips are deducted from the DB
    pub fn reserve_chips(ctx: Context<ReserveChips>, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.user_chip_account.to_account_info(),
            to: ctx.accounts.treasury_chip_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
            mint: ctx.accounts.chip_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        token_interface::transfer_checked(
            cpi_ctx,
            amount,
            ctx.accounts.chip_mint.decimals,
        )?;

        Ok(())
    }

    pub fn claim_chips(ctx: Context<ClaimChips>) -> Result<()> {
        let claimable_rewards_account = &mut ctx.accounts.claimable_rewards_account;

        let treasury_seeds: &[&[u8]] = &[b"TREASURY", &[ctx.accounts.treasury.bump]];

        let signer_seeds: &[&[&[u8]]] = &[treasury_seeds];

        let transfer_accounts = TransferChecked {
            from: ctx.accounts.treasury_chip_token_account.to_account_info(),
            to: ctx.accounts.claimer_chip_account.to_account_info(),
            authority: ctx.accounts.treasury.to_account_info(),
            mint: ctx.accounts.chip_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );
        token_interface::transfer_checked(cpi_ctx, claimable_rewards_account.amount, ctx.accounts.chip_mint.decimals)?;

        //user claimed rewards, reset them
        claimable_rewards_account.amount = 0;
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
    // pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
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
pub struct DistributeChips<'info> {
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
        seeds = [second_place_authority.key().as_ref()],
        bump
    )]
    pub second_place_claimable_rewards_account: Account<'info, ClaimableRewards>,
    /// CHECK: only an address that is the recipient, no need for checking
    pub second_place_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        space = ANCHOR_DISCRIMINATOR + ClaimableRewards::INIT_SPACE,
        payer = signer,
        seeds = [third_place_authority.key().as_ref()],
        bump
    )]
    pub third_place_claimable_rewards_account: Account<'info, ClaimableRewards>,
    /// CHECK: only an address that is the recipient, no need for checking
    pub third_place_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReserveChips<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, // vrv  ne treba testiraj
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
        mut, // vrv  ne treba testiraj
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

#[error_code]
pub enum Errors {
    Overflow,
}
