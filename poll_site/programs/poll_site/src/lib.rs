use anchor_lang::prelude::{*, borsh::{BorshSerialize, BorshDeserialize}};

declare_id!("8WtoeFycM6CribYGy1URCYBrUg9XNMYBsqbewNY3xggr");

#[program]
pub mod poll_site {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, title: String, choices: Vec<Choices>) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.title = title;
        poll.choices = choices;
        poll.is_closed = false;
        Ok(())
    }
    // Add_vote function for the program
    pub fn add_vote(ctx: Context<AddVote>, choice_name: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
    
        if let Some(choice_index) = poll.choices.iter().position(|c| c.choice_name == choice_name) {
            poll.choices[choice_index].votes += 1;
            Ok(())
        } else {
            Err(ProgramError::Custom(0).into())
        }
    }
    
    pub fn close_poll(ctx: Context<CloseVote>) -> Result<()>{
        let poll = &mut ctx.accounts.poll;
        poll.is_closed = true;
        Ok(())
    }
}

   

#[account]
pub struct Poll{
    title: String,
    choices: Vec<Choices>,
    is_closed: bool,

}
#[derive(BorshSerialize, BorshDeserialize,Clone, Default)]
pub struct Choices {
    choice_name: String,
    votes: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
}
#[derive(Accounts)]
pub struct CloseVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
}
