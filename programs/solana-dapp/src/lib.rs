use anchor_lang::prelude::*;

declare_id!("G6EWd6ucYtZZmo6pEfGkFzkkWVVQDxd98TcVMvRWXeBH");

#[program]
pub mod solana_dapp {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, _user_name: String) -> Result<()> {
        let user = &mut ctx.accounts.user_profile;

        user.authority = ctx.accounts.authority.key();
        user.user_name = _user_name;
        user.comments = 0;

        Ok(())
    }

    pub fn write_comment(ctx: Context<WriteComment>, _data: String, random_val: u8) -> Result<()> {
        let user = &mut ctx.accounts.user_profile;
        let comment = &mut ctx.accounts.comment;

        comment.authority = ctx.accounts.authority.key();
        comment.data = _data;

        user.comments = user.comments.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_ID, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4 + MAX_COMMENT_LENGTH,
    )]
    pub user_profile: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(random_val: u8)]
pub struct WriteComment<'info> {
    #[account(
        mut,
        seeds = [USER_ID, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Account<'info, User>,

    #[account(
        init,
        payer = authority,
        seeds = [COMMENT_ID, authority.key().as_ref(), &random_val.to_le_bytes()],
        bump,
        space = 8 + 32 + 4 + MAX_COMMENT_LENGTH,
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub user_name: String,
    pub comments: u8,
}

#[account]
pub struct Comment {
    pub authority: Pubkey,
    pub data: String,
}

#[constant]
pub const USER_ID: &[u8] = b"USER_ID";

#[constant]
pub const COMMENT_ID: &[u8] = b"COMMENT_ID";

const MAX_USER_NAME_LENGTH: usize = 32; // Define maximum length for user_name
const MAX_COMMENT_LENGTH: usize = 256;