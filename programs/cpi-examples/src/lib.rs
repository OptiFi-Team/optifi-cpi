use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cpi_examples {
    use super::*;

    pub fn init_user_account_example(ctx: Context<InitializeUserExampleAccount>) -> Result<()> {
        // TODO: need fix seed and test
        let bump = 3;
        let base_seeds = &[&b"daoProgramAuthority"[..], &[bump]];
        let seeds = &[base_seeds.as_ref()];

        let mut cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.optifi_program_id.clone(),
            optifi_cpi::cpi::accounts::InitializeUserAccount {
                optifi_exchange: ctx.accounts.optifi_exchange.to_account_info(),
                user_account: ctx.accounts.user_account.to_account_info(),
                liquidation_account: ctx.accounts.liquidation_account.to_account_info(),
                user_margin_account_usdc: ctx.accounts.user_margin_account_usdc.to_account_info(),
                owner: ctx.accounts.owner.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            seeds,
        );

        cpi_ctx.accounts.owner.is_signer = true;
        cpi_ctx.accounts.payer.is_signer = true;

        let bumps = optifi_cpi::instructions::InitUserAccountBumpSeeds {
            user_account: bump,
            liquidation_account: bump,
        };

        optifi_cpi::cpi::init_user_account(cpi_ctx, bumps).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUserExampleAccount<'info> {
    #[account()]
    pub optifi_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub optifi_exchange: AccountInfo<'info>,
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    #[account(mut)]
    pub liquidation_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_margin_account_usdc: AccountInfo<'info>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}
