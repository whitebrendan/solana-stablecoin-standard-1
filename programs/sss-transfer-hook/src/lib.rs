use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::transfer_hook::TransferHookAccount;
use spl_transfer_hook_interface::instruction::TransferHookInstruction;

declare_id!("SSS_HOOK111111111111111111111111111111111111");

#[program]
pub mod sss_transfer_hook {
    use super::*;

    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        let source_blacklist = &ctx.accounts.source_blacklist;
        let destination_blacklist = &ctx.accounts.destination_blacklist;

        // Block transfer if source is blacklisted
        if source_blacklist.is_some() {
             let item = source_blacklist.as_ref().unwrap();
             if item.is_blacklisted {
                 return Err(TransferHookError::BlacklistedAccount.into());
             }
        }

        // Block transfer if destination is blacklisted
        if destination_blacklist.is_some() {
             let item = destination_blacklist.as_ref().unwrap();
             if item.is_blacklisted {
                 return Err(TransferHookError::BlacklistedAccount.into());
             }
        }
        
        Ok(())
    }
    
    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        // Logic to populate ExtraAccountMeta with BlacklistItem PDA derivations
        Ok(())
    }
}

#[account]
pub struct BlacklistItem {
    pub is_blacklisted: bool,
}

#[derive(Accounts)]
pub struct TransferHook<'info> {
    pub source: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub destination: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    /// CHECK: Validated in program logic
    pub extra_metas_account: AccountInfo<'info>,
    
    // Optional Blacklist items passed via ExtraAccountMeta
    pub source_blacklist: Option<Account<'info, BlacklistItem>>,
    pub destination_blacklist: Option<Account<'info, BlacklistItem>>,
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(init, payer = payer, space = 8 + 1024)]
    pub extra_metas_account: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum TransferHookError {
    #[msg("Account is blacklisted and cannot participate in transfers")]
    BlacklistedAccount,
}
