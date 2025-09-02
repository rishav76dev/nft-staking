pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("1aEajZaHNq5xnWKRCJ6QAoiysfFim7yCGkFhNoaZ59i");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_nft_stake: u8, // 1000/10000
        points_per_sol_stake: u8, // 100/10000
        points_per_spl_stake: u8, // 10/10000
        min_freeze_period: i64,
        annual_percentage_rate: u16,
    ) -> Result<()> {
        ctx.accounts.initialize_config(
            points_per_nft_stake,
            points_per_sol_stake,
            points_per_spl_stake,
            min_freeze_period,
            annual_percentage_rate,
            &ctx.bumps,
        )
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake_nft(ctx: Context<StakeNFT>, seed: u64, locked_stackers: bool, lock_period:i64)  -> Result<()>{
        ctx.accounts.stake_nft(seed, locked_stackers, lock_period, &ctx.bumps)
    }

    
}

