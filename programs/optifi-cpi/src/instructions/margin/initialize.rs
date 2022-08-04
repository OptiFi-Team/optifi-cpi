use super::*;

#[derive(Accounts, Clone)]
#[instruction(bump: u8,asset:u8)]
pub struct InitMarginStressContext<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(init_if_needed, 
        seeds=[
            PREFIX_MARGIN_STRESS.as_bytes(),
            optifi_exchange.key().as_ref(),
            &[asset],
        ],
        payer=payer, bump, space=MarginStressAccount::get_space_allocation())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,


    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
