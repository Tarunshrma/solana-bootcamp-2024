use anchor_lang::prelude::*;

declare_id!("96EgiUrjjEBLE5BuZppisACZH4SfFPsR5RYQKqr7MugN");

#[program]
pub mod Voting {
    use super::*;

    pub fn initilize_poll(ctx: Context<InitializePoll>,
                                        poll_id:u64, 
                                        poll_name:String, 
                                        poll_description:String,
                                        start_date:u64,
                                        end_date: u64,
                                        poll_option_index: u64) -> Result<()> {
        msg!("Initilizing poll with id: {}", poll_id);
        ctx.accounts.poll.poll_id = poll_id;
        ctx.accounts.poll.poll_name = poll_name;
        ctx.accounts.poll.poll_description = poll_description;
        ctx.accounts.poll.start_date = start_date;
        ctx.accounts.poll.end_date = end_date;
        ctx.accounts.poll.poll_option_index = poll_option_index;
        Ok(())
    }

    pub fn initilize_candidate(ctx: Context<InitializeCandidate>,
                                        _poll_id:u64,
                                        candidate_name:String) -> Result<()> {

        msg!("Initilizing candidate with id: {}", candidate_name);
        ctx.accounts.candidate.candidate_name = candidate_name;
        ctx.accounts.poll_account.poll_option_index += 1;
        Ok(())
    }

    pub fn vote(ctx:Context<Vote>, _poll_id: u64, candidate_name: String) -> Result<()>{
        msg!("Voting for candidate: {}", candidate_name);
        let current_time = Clock::get()?.unix_timestamp;

        let start_date = ctx.accounts.poll_account.start_date as i64;
        let end_date = ctx.accounts.poll_account.end_date as i64;

        if current_time < start_date {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        if current_time > end_date {
            return Err(ErrorCode::VotingEnded.into());
        }

        let candidate = &mut ctx.accounts.canidate_account;
        candidate.candidate_votes += 1;

        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(poll_id: u64, candidate_name: String)]
    pub struct Vote<'info>{
        #[account(mut)]
        pub user: Signer<'info>,


        #[account(
            mut,
            seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
            bump,
        )]
        pub poll_account: Account<'info, PollAccount>, 

        #[account(
            mut,
            seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
            bump,
        )]
        pub canidate_account: Account<'info, CandidateAccount>, 
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

    #[derive(Accounts)]
    #[instruction(poll_id: u64, candidate_name: String)]
    pub struct InitializeCandidate<'info>{
        #[account(mut)]
        pub user: Signer<'info>,

        pub poll_account: Account<'info, PollAccount>,

        #[account(
            init_if_needed,
            payer = user,
            space = 8 + CandidateAccount::INIT_SPACE,
            seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
            bump,
        )]
        pub candidate: Account<'info, CandidateAccount>,

        pub system_program: Program<'info, System>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct CandidateAccount{
        #[max_len(50)]
        pub candidate_name: String,
        pub candidate_votes: u64,
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

    #[error_code]
    pub enum ErrorCode {
        #[msg("Voting has not started yet")]
        VotingNotStarted,
        #[msg("Voting has ended")]
        VotingEnded,
    }
}

