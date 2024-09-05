use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

declare_id!("HcxtR55Ec4XQPt47SckxG6RMoWyFAEbSYn5BTmVX7DEE");

#[program]
pub mod kamino_deposit {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn execute_kamino_operations(
        ctx: Context<ExecuteKaminoOperations>,
        amount: u64,
    ) -> Result<()> {
        let accounts = &ctx.remaining_accounts;

        // Step 1: Init Obligation
        invoke(
            &init_obligation_instruction(&ctx.accounts.kamino_program.key(), accounts, amount),
            &[
                accounts[0].clone(),
                accounts[1].clone(),
                accounts[2].clone(),
                accounts[3].clone(),
                accounts[4].clone(),
                accounts[5].clone(),
                accounts[6].clone(),
                accounts[7].clone(),
                accounts[8].clone(),
            ],
        )?;

        // Step 2: Init Obligation Farms for Reserve
        invoke(
            &init_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_farm.key(),
                accounts,
                0,
            ),
            &[
                accounts[9].clone(),
                accounts[10].clone(),
                accounts[11].clone(),
                accounts[12].clone(),
                accounts[13].clone(),
                accounts[14].clone(),
                accounts[15].clone(),
                accounts[16].clone(),
                accounts[17].clone(),
                accounts[18].clone(),
                accounts[19].clone(),
            ],
        )?;

        // Step 3: Refresh Reserve
        invoke(
            &refresh_reserve_instruction(&ctx.accounts.kamino_program.key(), accounts),
            &[
                accounts[20].clone(),
                accounts[21].clone(),
                accounts[22].clone(),
                accounts[23].clone(),
                accounts[24].clone(),
            ],
        )?;

        // Step 4: Refresh Obligation
        invoke(
            &refresh_obligation_instruction(&ctx.accounts.kamino_program.key(), accounts),
            &[accounts[25].clone(), accounts[26].clone()],
        )?;

        // Step 5: Deposit Reserve Liquidity and Obligation Collateral
        invoke(
            &deposit_reserve_liquidity_and_obligation_collateral_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                amount,
            ),
            &[
                accounts[27].clone(),
                accounts[28].clone(),
                accounts[29].clone(),
                accounts[30].clone(),
                accounts[31].clone(),
                accounts[32].clone(),
                accounts[33].clone(),
                accounts[34].clone(),
                accounts[35].clone(),
                accounts[36].clone(),
                accounts[37].clone(),
                accounts[38].clone(),
                accounts[39].clone(),
                accounts[40].clone(),
            ],
        )?;

        // Step 6: Refresh Obligation Farms for Reserve (Only after deposit)
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_farm.key(),
                accounts,
                0,
            ),
            &[
                accounts[41].clone(),
                accounts[42].clone(),
                accounts[43].clone(),
                accounts[44].clone(),
                accounts[45].clone(),
                accounts[46].clone(),
                accounts[47].clone(),
                accounts[48].clone(),
                accounts[49].clone(),
                accounts[50].clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoOperations<'info> {
    // Kamino Lending Program
    pub kamino_program: AccountInfo<'info>,

    // Kamino Farm Program
    pub kamino_farm: AccountInfo<'info>,
}

fn init_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), true),
            AccountMeta::new(accounts[2].key(), false),
            AccountMeta::new(accounts[3].key(), false),
            AccountMeta::new(accounts[4].key(), false),
            AccountMeta::new(accounts[5].key(), false),
            AccountMeta::new(accounts[6].key(), false),
            AccountMeta::new_readonly(accounts[7].key(), false),
            AccountMeta::new_readonly(accounts[8].key(), false),
        ],
        data: {
            let mut data = vec![0x00];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn init_obligation_farms_for_reserve_instruction(
    kamino_farm_id: &Pubkey,
    accounts: &[AccountInfo],
    mode: u8,
) -> Instruction {
    Instruction {
        program_id: *kamino_farm_id,
        accounts: vec![
            AccountMeta::new(accounts[9].key(), true),
            AccountMeta::new(accounts[10].key(), true),
            AccountMeta::new(accounts[11].key(), false),
            AccountMeta::new(accounts[12].key(), false),
            AccountMeta::new(accounts[13].key(), false),
            AccountMeta::new(accounts[14].key(), false),
            AccountMeta::new(accounts[15].key(), false),
            AccountMeta::new(accounts[16].key(), false),
            AccountMeta::new(accounts[17].key(), false),
            AccountMeta::new_readonly(accounts[18].key(), false),
            AccountMeta::new_readonly(accounts[19].key(), false),
        ],
        data: {
            let mut data = vec![0x01];
            data.push(mode);
            data
        },
    }
}

fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[20].key(), false),
            AccountMeta::new(accounts[21].key(), false),
            AccountMeta::new(accounts[22].key(), false),
            AccountMeta::new(accounts[23].key(), false),
            AccountMeta::new(accounts[24].key(), false),
        ],
        data: vec![0x02],
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[25].key(), false),
            AccountMeta::new(accounts[26].key(), false),
        ],
        data: vec![0x03],
    }
}

fn deposit_reserve_liquidity_and_obligation_collateral_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[27].key(), true),
            AccountMeta::new(accounts[28].key(), false),
            AccountMeta::new(accounts[29].key(), false),
            AccountMeta::new(accounts[30].key(), false),
            AccountMeta::new(accounts[31].key(), false),
            AccountMeta::new(accounts[32].key(), false),
            AccountMeta::new(accounts[33].key(), false),
            AccountMeta::new(accounts[34].key(), false),
            AccountMeta::new(accounts[35].key(), false),
            AccountMeta::new(accounts[36].key(), false),
            AccountMeta::new(accounts[37].key(), false),
            AccountMeta::new_readonly(accounts[38].key(), false),
            AccountMeta::new_readonly(accounts[39].key(), false),
            AccountMeta::new_readonly(accounts[40].key(), false),
        ],
        data: {
            let mut data = vec![0x04];
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    }
}

fn refresh_obligation_farms_for_reserve_instruction(
    kamino_farm_id: &Pubkey,
    accounts: &[AccountInfo],
    mode: u8,
) -> Instruction {
    Instruction {
        program_id: *kamino_farm_id,
        accounts: vec![
            AccountMeta::new(accounts[41].key(), true),
            AccountMeta::new(accounts[42].key(), false),
            AccountMeta::new(accounts[43].key(), false),
            AccountMeta::new(accounts[44].key(), false),
            AccountMeta::new(accounts[45].key(), false),
            AccountMeta::new(accounts[46].key(), false),
            AccountMeta::new(accounts[47].key(), false),
            AccountMeta::new(accounts[48].key(), false),
            AccountMeta::new_readonly(accounts[49].key(), false),
            AccountMeta::new_readonly(accounts[50].key(), false),
        ],
        data: {
            let mut data = vec![0x05];
            data.push(mode);
            data
        },
    }
}
