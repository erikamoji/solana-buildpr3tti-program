use anchor_lang::prelude::*;

declare_id!("4pcvRiBAAzooGJjavJ1xUXpzKxzf5ATrjDFAJwXQjmUK");

#[program]
pub mod myepicproject {
  use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
      Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}