use anchor_lang::prelude::*;

declare_id!("AqPFKQK2Nh5yurpGUZc5kJoAb1d2ETAMP49fZQbATH4y");

#[program]
pub mod solana_chat {

    use super::*;

    pub fn create_message(
        ctx: Context<CreateMessage>,
        text: String
    ) -> Result<()> {

        let message = &mut ctx.accounts.message;
        let clock: Clock = Clock::get().unwrap();

        message.owner = ctx.accounts.user.key();
        message.text = text;
        message.timestamp = clock.unix_timestamp;

        Ok(())
    }

}


#[derive(Accounts)]
#[instruction(text: String)]
pub struct CreateMessage<'info> {
    #[account(
        init,
        payer = user,
        space = 8  +                             // Discriminator Anchor
                32 +                             // Owner
                text.as_bytes().len() + 4 +      // Text + 4    (String prefix)
                8                                // Timestamp
    )]
    pub message: Account<'info, Message>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Message {
    pub owner: Pubkey,    // 32
    pub text: String,     // n + 4
    pub timestamp: i64,   // 8
}
