use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{self, MintTo, Burn, TransferChecked},
    token_interface::{Mint, TokenAccount, Token2022},
};

declare_id!("SSS1111111111111111111111111111111111111111");

#[program]
pub mod sss_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, config: StablecoinConfig) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.name = config.name;
        state.symbol = config.symbol;
        state.decimals = config.decimals;
        state.enable_permanent_delegate = config.enable_permanent_delegate;
        state.enable_transfer_hook = config.enable_transfer_hook;
        state.paused = false;
        state.mint = ctx.accounts.mint.key();
        
        state.roles.master = ctx.accounts.authority.key();
        state.roles.minter = ctx.accounts.authority.key();
        state.roles.blacklister = ctx.accounts.authority.key();
        
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(!state.paused, SSSCoreError::ContractPaused);
        require!(ctx.accounts.authority.key() == state.roles.minter, SSSCoreError::Unauthorized);

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_2022::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn burn_from(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(!state.paused, SSSCoreError::ContractPaused);
        
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_2022::burn(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn seize(ctx: Context<SeizeTokens>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(state.enable_permanent_delegate, SSSCoreError::FeatureNotEnabled);
        require!(ctx.accounts.authority.key() == state.roles.master, SSSCoreError::Unauthorized);
        
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.from.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.permanent_delegate.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_2022::transfer_checked(cpi_ctx, amount, state.decimals)?;

        Ok(())
    }

    pub fn update_roles(ctx: Context<UpdateState>, roles: Roles) -> Result<()> {
        ctx.accounts.state.roles = roles;
        Ok(())
    }

    pub fn set_paused(ctx: Context<UpdateState>, paused: bool) -> Result<()> {
        ctx.accounts.state.paused = paused;
        Ok(())
    }

    pub fn add_to_blacklist(ctx: Context<UpdateBlacklist>, _address: Pubkey) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(ctx.accounts.authority.key() == state.roles.blacklister, SSSCoreError::Unauthorized);
        let blacklist_item = &mut ctx.accounts.blacklist_item;
        blacklist_item.is_blacklisted = true;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StablecoinConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub enable_permanent_delegate: bool,
    pub enable_transfer_hook: bool,
    pub default_account_frozen: bool,
}

#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub enable_permanent_delegate: bool,
    pub enable_transfer_hook: bool,
    pub paused: bool,
    pub roles: Roles,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Roles {
    pub master: Pubkey,
    pub minter: Pubkey,
    pub blacklister: Pubkey,
    pub pauser: Pubkey,
}

#[account]
pub struct BlacklistItem {
    pub is_blacklisted: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 64 + 64 + 1 + 1 + 1 + 1 + 128)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, GlobalState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct UpdateBlacklist<'info> {
    #[account(init_if_needed, payer = authority, space = 8 + 1, seeds = [b"blacklist", address.as_ref()], bump)]
    pub blacklist_item: Account<'info, BlacklistItem>,
    #[account(has_one = authority)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(has_one = mint)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub recipient: InterfaceAccount<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(has_one = mint)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct SeizeTokens<'info> {
    #[account(has_one = mint)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: InterfaceAccount<'info, TokenAccount>,
    pub permanent_delegate: Signer<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[error_code]
pub enum SSSCoreError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Feature not enabled for this stablecoin preset")]
    FeatureNotEnabled,
}
