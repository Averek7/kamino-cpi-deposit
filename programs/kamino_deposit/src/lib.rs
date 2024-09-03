use anchor_lang::{prelude::*, solana_program::instruction::Instruction};
use anchor_spl::token::{Token, TokenAccount};

declare_id!("EQ4ZfkAaGPuuVGkaBUhdSi4QQ6RdkxNWQ8eCU96x2dQU");

#[program]
pub mod kamino_deposit {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn execute_kamino_operations(
        ctx: Context<ExecuteKaminoOperations>,
        amount: u64,
    ) -> Result<()> {
        // Step 1: Init Obligation
        invoke(
            &init_obligation_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.obligation_owner.to_account_info(),
                ctx.accounts.fee_payer.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.system_program.to_account_info(), // Seed 1 Account
                ctx.accounts.system_program.to_account_info(), // Seed 2 Account
                ctx.accounts.owner_user_metadata.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Step 2: Init Obligation Farms for Reserve
        invoke(
            &init_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_farm.key(),
                &ctx.accounts,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
                ctx.accounts.lending_market_authority.to_account_info(),
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.reserve_farm_state.to_account_info(),
                ctx.accounts.obligation_farm.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.farms_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Step 3: Refresh Reserve
        invoke(
            &refresh_reserve_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.pyth_oracle.to_account_info(),
                ctx.accounts.switchboard_price_oracle.to_account_info(),
                ctx.accounts.switchboard_twap_oracle.to_account_info(),
                ctx.accounts.scope_prices.to_account_info(),
            ],
        )?;

        // Step 4: Refresh Obligation
        invoke(
            &refresh_obligation_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
            ],
        )?;

        // Step 5: Deposit Reserve Liquidity and Obligation Collateral
        invoke(
            &deposit_reserve_liquidity_and_obligation_collateral_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.accounts,
                amount,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.lending_market_authority.to_account_info(),
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.reserve_liquidity_mint.to_account_info(),
                ctx.accounts.reserve_liquidity_supply.to_account_info(),
                ctx.accounts.reserve_collateral_mint.to_account_info(),
                ctx.accounts
                    .reserve_destination_deposit_collateral
                    .to_account_info(),
                ctx.accounts.user_source_liquidity.to_account_info(),
                ctx.accounts
                    .placeholder_user_destination_collateral
                    .to_account_info(),
                ctx.accounts.collateral_token_program.to_account_info(),
                ctx.accounts.liquidity_token_program.to_account_info(),
                ctx.accounts.instruction_sysvar.to_account_info(),
            ],
        )?;

        // Step 6: Refresh Obligation Farms for Reserve (Only after deposit)
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_farm.key(),
                &ctx.accounts,
            ),
            &[
                ctx.accounts.crank.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
                ctx.accounts.lending_market_authority.to_account_info(),
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.reserve_farm_state.to_account_info(),
                ctx.accounts.obligation_farm_user_state.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.farms_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoOperations<'info> {
    // Init Obligation Accounts
    #[account(mut)]
    pub obligation_owner: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    ///CHECK
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    pub owner_user_metadata: AccountInfo<'info>,
    #[account(mut)]
    pub rent: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    // Init Obligation Farms for Reserve Accounts
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_farm_state: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_farm: AccountInfo<'info>,
    pub farms_program: AccountInfo<'info>,

    // Refresh Reserve Accounts
    #[account(mut)]
    pub pyth_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub switchboard_price_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub switchboard_twap_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub scope_prices: AccountInfo<'info>,

    // Deposit Reserve Liquidity and Obligation Collateral Accounts
    #[account(mut)]
    pub reserve_liquidity_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_destination_deposit_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub user_source_liquidity: Account<'info, TokenAccount>,
    #[account(mut)]
    pub placeholder_user_destination_collateral: AccountInfo<'info>,
    pub collateral_token_program: Program<'info, Token>,
    pub liquidity_token_program: Program<'info, Token>,
    pub instruction_sysvar: AccountInfo<'info>,

    // Kamino Lending Program
    pub kamino_program: AccountInfo<'info>,

    // Kamino Farm Program
    pub kamino_farm: AccountInfo<'info>,

    pub crank: AccountInfo<'info>,
    pub obligation_farm_user_state: AccountInfo<'info>,
}

fn init_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.obligation_owner.key(), true),
            AccountMeta::new(accounts.fee_payer.key(), true),
            AccountMeta::new(accounts.obligation.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.system_program.key(), false),
            AccountMeta::new(accounts.system_program.key(), false),
            AccountMeta::new(accounts.owner_user_metadata.key(), false),
            AccountMeta::new_readonly(accounts.rent.key(), false),
            AccountMeta::new_readonly(accounts.system_program.key(), false),
        ],
        data: vec![0, 0],
    }
}

fn init_obligation_farms_for_reserve_instruction(
    kamino_farm_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
) -> Instruction {
    Instruction {
        program_id: *kamino_farm_id,
        accounts: vec![
            AccountMeta::new(accounts.payer.key(), true),
            AccountMeta::new(accounts.owner.key(), true),
            AccountMeta::new(accounts.obligation.key(), false),
            AccountMeta::new(accounts.lending_market_authority.key(), false),
            AccountMeta::new(accounts.reserve.key(), false),
            AccountMeta::new(accounts.reserve_farm_state.key(), false),
            AccountMeta::new(accounts.obligation_farm.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.farms_program.key(), false),
            AccountMeta::new_readonly(accounts.rent.key(), false),
            AccountMeta::new_readonly(accounts.system_program.key(), false),
        ],
        data: vec![0, 1],
    }
}

fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.reserve.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.pyth_oracle.key(), false),
            AccountMeta::new(accounts.switchboard_price_oracle.key(), false),
            AccountMeta::new(accounts.switchboard_twap_oracle.key(), false),
            AccountMeta::new(accounts.scope_prices.key(), false),
        ],
        data: vec![1],
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.obligation.key(), false),
        ],
        data: vec![2],
    }
}

fn refresh_obligation_farms_for_reserve_instruction(
    kamino_farm_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
) -> Instruction {
    Instruction {
        program_id: *kamino_farm_id,
        accounts: vec![
            AccountMeta::new(accounts.crank.key(), true),
            AccountMeta::new(accounts.obligation.key(), false),
            AccountMeta::new(accounts.lending_market_authority.key(), false),
            AccountMeta::new(accounts.reserve.key(), false),
            AccountMeta::new(accounts.reserve_farm_state.key(), false),
            AccountMeta::new(accounts.obligation_farm_user_state.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.farms_program.key(), false),
            AccountMeta::new_readonly(accounts.rent.key(), false),
            AccountMeta::new_readonly(accounts.system_program.key(), false),
        ],
        data: vec![3],
    }
}

fn deposit_reserve_liquidity_and_obligation_collateral_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoOperations,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.owner.key(), true),
            AccountMeta::new(accounts.obligation.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.lending_market_authority.key(), false),
            AccountMeta::new(accounts.reserve.key(), false),
            AccountMeta::new(accounts.reserve_liquidity_mint.key(), false),
            AccountMeta::new(accounts.reserve_liquidity_supply.key(), false),
            AccountMeta::new(accounts.reserve_collateral_mint.key(), false),
            AccountMeta::new(accounts.reserve_destination_deposit_collateral.key(), false),
            AccountMeta::new(accounts.user_source_liquidity.key(), false),
            AccountMeta::new(
                accounts.placeholder_user_destination_collateral.key(),
                false,
            ),
            AccountMeta::new_readonly(accounts.collateral_token_program.key(), false),
            AccountMeta::new_readonly(accounts.liquidity_token_program.key(), false),
            AccountMeta::new_readonly(accounts.instruction_sysvar.key(), false),
        ],
        data: vec![4, amount as u8],
    }
}
