#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("A9apbLdvLHj322EZxpShv77qgybArWJ7MRgKjDirCKKf");

#[program]
pub mod crud {
    use super::*;

    pub fn init_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()>{

      let journal_entry= &mut ctx.accounts.journal_entry;
      journal_entry.owner= *ctx.accounts.owner.key;
      journal_entry.title= title;
      journal_entry.message= message;
      Ok(())
    }


    pub fn update_entry(ctx: Context<UpdateJournal>, title: String, message: String) -> Result<()> {

      let journal_entry= &mut ctx.accounts.journal_entry;
      journal_entry.message= message;
      Ok(())

    } 

    pub fn delete_entry(ctx: Context<DeleteEntry>, title: String) -> Result<()> {
      Ok(())
    }
  
  }


  #[derive(Accounts)]
  #[instruction(title: String)]
  pub struct CreateEntry<'info>{

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      init,
      payer= owner,
      space= 8 + JournalEntryState:: INIT_SPACE,
      seeds= [title.as_bytes(), owner.key().as_ref()],
      bump,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,

  }

  #[derive(Accounts)]
  #[instruction(title: String)]
  pub struct UpdateJournal<'info>{

    #[account(
      mut,
      seeds= [title.as_bytes(), owner.key().as_ref()],
      bump,
      realloc= 8 + JournalEntryState::INIT_SPACE,
      realloc :: payer= owner,
      realloc :: zero= true,


    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
  }

  #[derive(Accounts)]
  #[instruction(title: String)]
  pub struct DeleteEntry<'info> {
    #[account(
      mut,
      seeds= [title.as_bytes(), owner.key().as_ref()],
      bump,
      close= owner,
      )]
       pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,

  }


#[account]
#[derive(InitSpace)]
 pub struct JournalEntryState{
  pub owner: Pubkey,
  #[max_len(16)]
  pub title: String,
  #[max_len(200)]
  pub message: String,
 }