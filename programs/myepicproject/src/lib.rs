use anchor_lang::{prelude::*};

declare_id!("42ex3aonMY8Tkv4sAYvgizJVapWreEds1W469So6FinC");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String, _user_name: String) -> Result <()>{
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: user.to_account_info().key(),
            user_name: _user_name
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link : String,
    pub user_address: Pubkey,
    pub user_name: String
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub name: String,
    pub gif_list: Vec<ItemStruct>
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,

    pub user: Signer<'info>,
}