use anchor_lang::prelude::*;

declare_id!("Cffb3HKJ5mFR6NvKP82tabnGooT4Hjc2us8LWmYqxg8L");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
