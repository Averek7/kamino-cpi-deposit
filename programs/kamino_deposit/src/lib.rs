use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

declare_id!("HcxtR55Ec4XQPt47SckxG6RMoWyFAEbSYn5BTmVX7DEE");

#[program]
pub mod kamino_deposit {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn execute_kamino_operations(
        ctx: Context<ExecuteKaminoOperations>,
        data: Vec<Vec<u8>>,
    ) -> Result<()> {
        let accounts = &ctx.remaining_accounts;

        invoke(
            &initiate_ixns(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[0].clone(),
            ),
            &[
                accounts[0].clone(),
                accounts[1].clone(),
                accounts[2].clone(),
                accounts[3].clone(),
                accounts[4].clone(),
                accounts[5].clone(),
            ],
        )?;

        // Step 1: Init Obligation
        invoke(
            &init_obligation_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[1].clone(),
            ),
            &[
                accounts[6].clone(),
                accounts[7].clone(),
                accounts[8].clone(),
                accounts[9].clone(),
                accounts[10].clone(),
                accounts[11].clone(),
                accounts[12].clone(),
                accounts[13].clone(),
                accounts[14].clone(),
            ],
        )?;

        // Step 2: Init Obligation Farms for Reserve
        invoke(
            &init_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[2].clone(),
            ),
            &[
                accounts[15].clone(),
                accounts[16].clone(),
                accounts[17].clone(),
                accounts[18].clone(),
                accounts[19].clone(),
                accounts[20].clone(),
                accounts[21].clone(),
                accounts[22].clone(),
                accounts[23].clone(),
                accounts[24].clone(),
                accounts[25].clone(),
            ],
        )?;

        // Step 3: Refresh Reserve
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[3].clone(),
            ),
            &[
                accounts[26].clone(),
                accounts[27].clone(),
                accounts[28].clone(),
                accounts[29].clone(),
                accounts[30].clone(),
                accounts[31].clone(),
            ],
        )?;

        // Step 4: Refresh Obligation
        invoke(
            &refresh_obligation_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[4].clone(),
            ),
            &[accounts[32].clone(), accounts[33].clone()],
        )?;

        // Step 5: Refresh Obligation Farms for Reserve (before deposit)
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[5].clone(),
            ),
            &[
                accounts[34].clone(),
                accounts[35].clone(),
                accounts[36].clone(),
                accounts[37].clone(),
                accounts[38].clone(),
                accounts[39].clone(),
                accounts[40].clone(),
                accounts[41].clone(),
                accounts[42].clone(),
                accounts[43].clone(),
            ],
        )?;

        // Step 6: Deposit Reserve Liquidity and Obligation Collateral
        invoke(
            &deposit_reserve_liquidity_and_obligation_collateral_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[6].clone(),
            ),
            &[
                accounts[44].clone(),
                accounts[45].clone(),
                accounts[46].clone(),
                accounts[47].clone(),
                accounts[48].clone(),
                accounts[49].clone(),
                accounts[50].clone(),
                accounts[51].clone(),
                accounts[52].clone(),
                accounts[53].clone(),
                accounts[54].clone(),
                accounts[55].clone(),
                accounts[56].clone(),
                accounts[57].clone(),
            ],
        )?;

        // Step 7: Refresh Obligation Farms for Reserve (after deposit)
        invoke(
            &refresh_obligation_farms_for_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                accounts,
                data[7].clone(),
            ),
            &[
                accounts[58].clone(),
                accounts[59].clone(),
                accounts[60].clone(),
                accounts[61].clone(),
                accounts[62].clone(),
                accounts[63].clone(),
                accounts[64].clone(),
                accounts[65].clone(),
                accounts[66].clone(),
                accounts[67].clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoOperations<'info> {
    pub kamino_program: AccountInfo<'info>,
}

fn initiate_ixns(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {

    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), true),
            AccountMeta::new(accounts[2].key(), false),
            AccountMeta::new_readonly(accounts[3].key(), false),
            AccountMeta::new_readonly(accounts[4].key(), false),
            AccountMeta::new_readonly(accounts[5].key(), false),
        ],
        data,
    }
}

fn init_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {

    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(accounts[6].key(), true),
            AccountMeta::new(accounts[7].key(), true),
            AccountMeta::new(accounts[8].key(), false),
            AccountMeta::new_readonly(accounts[9].key(), false),
            AccountMeta::new_readonly(accounts[10].key(), false),
            AccountMeta::new_readonly(accounts[11].key(), false),
            AccountMeta::new_readonly(accounts[12].key(), false),
            AccountMeta::new_readonly(accounts[13].key(), false),
            AccountMeta::new_readonly(accounts[14].key(), false),
        ],
        data,
    }
}

fn init_obligation_farms_for_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {

    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[15].key(), true),
            AccountMeta::new_readonly(accounts[16].key(), true),
            AccountMeta::new(accounts[17].key(), false),
            AccountMeta::new(accounts[18].key(), false),
            AccountMeta::new(accounts[19].key(), false),
            AccountMeta::new(accounts[20].key(), false),
            AccountMeta::new(accounts[21].key(), false),
            AccountMeta::new_readonly(accounts[22].key(), false),
            AccountMeta::new_readonly(accounts[23].key(), false),
            AccountMeta::new_readonly(accounts[24].key(), false),
            AccountMeta::new_readonly(accounts[25].key(), false),
        ],
        data,
    }
}

fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {

    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[26].key(), false),
            AccountMeta::new_readonly(accounts[27].key(), false),
            AccountMeta::new_readonly(accounts[28].key(), false),
            AccountMeta::new_readonly(accounts[29].key(), false),
            AccountMeta::new_readonly(accounts[30].key(), false),
            AccountMeta::new_readonly(accounts[31].key(), false),
        ],
        data,
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {

    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(accounts[32].key(), false),
            AccountMeta::new(accounts[33].key(), false),
        ],
        data,
    }
}

fn deposit_reserve_liquidity_and_obligation_collateral_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[44].key(), true),
            AccountMeta::new(accounts[45].key(), false),
            AccountMeta::new_readonly(accounts[46].key(), false),
            AccountMeta::new_readonly(accounts[47].key(), false),
            AccountMeta::new(accounts[48].key(), false),
            AccountMeta::new(accounts[49].key(), false),
            AccountMeta::new(accounts[50].key(), false),
            AccountMeta::new(accounts[51].key(), false),
            AccountMeta::new(accounts[52].key(), false),
            AccountMeta::new(accounts[53].key(), false),
            AccountMeta::new_readonly(accounts[54].key(), false),
            AccountMeta::new_readonly(accounts[55].key(), false),
            AccountMeta::new_readonly(accounts[56].key(), false),
            AccountMeta::new_readonly(accounts[57].key(), true),
        ],
        data,
    }
}

fn refresh_obligation_farms_for_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts[34].key(), false),
            AccountMeta::new_readonly(accounts[35].key(), false),
            AccountMeta::new(accounts[36].key(), false),
            AccountMeta::new_readonly(accounts[37].key(), false),
            AccountMeta::new(accounts[38].key(), false),
            AccountMeta::new(accounts[39].key(), false),
            AccountMeta::new_readonly(accounts[40].key(), false),
            AccountMeta::new_readonly(accounts[41].key(), false),
            AccountMeta::new_readonly(accounts[42].key(), false),
            AccountMeta::new_readonly(accounts[43].key(), false),
        ],
        data,
    }
}
