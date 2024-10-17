use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod voting {
    use super::*;

    pub fn initilize_poll(_ctx: Context<InitializePoll>, poll_id:u64) -> Result<()> {
        msg!("Initilizing poll with id: {}", poll_id);
        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(poll_id: u64)]
    pub struct InitializePoll<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
            init_if_needed,
            payer = user,
            space = 8 + PollAccount::INIT_SPACE,
            seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
            bump,
        )]
        pub poll: Account<'info, PollAccount>,

        pub system_program: Program<'info, System>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct PollAccount {
        pub poll_id: u64,
        #[max_len(50)]
        pub poll_name: String,
        #[max_len(250)]
        pub poll_description: String,
        pub start_date: u64,
        pub end_date: u64,
        pub poll_option_index: u64,
    }
}

