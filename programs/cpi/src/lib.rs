use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke, system_instruction};
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("6NCfKM3jcHudXu25GFoFNJgwsCpAsTdxsGpmrfNX9dbU");

#[program]
pub mod cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn sol_transfer_one(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey: AccountInfo = ctx.accounts.sender.to_account_info();
        let to_pubkey:   AccountInfo = ctx.accounts.receipient.to_account_info();
        let program_id:  AccountInfo = ctx.accounts.system_program.to_account_info();

        let cpi_context: CpiContext<Transfer> = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn sol_transfer_two(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey: AccountInfo = ctx.accounts.sender.to_account_info(); 
        let to_pubkey:   AccountInfo = ctx.accounts.receipient.to_account_info();
        let program_id:  AccountInfo = ctx.accounts.system_program.to_account_info();

        let instruction: &Instruction = &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);

        invoke(instruction, &[from_pubkey, to_pubkey, program_id])?;
        Ok(())
    }

    pub fn sol_transfer_three(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey: AccountInfo = ctx.accounts.sender.to_account_info(); 
        let to_pubkey:   AccountInfo = ctx.accounts.receipient.to_account_info();
        let program_id:  AccountInfo = ctx.accounts.system_program.to_account_info();

        let account_metas: Vec<AccountMeta> = vec![
            AccountMeta::new(from_pubkey.key(), true),
            AccountMeta::new(to_pubkey.key(), false),
        ];

        let instruction_discriminator: u32 = 2;

        let mut instruction_data: Vec<u8> = Vec::with_capacity(4 + 8);
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction: Instruction = Instruction {
            program_id: program_id.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&instruction, &[from_pubkey, to_pubkey, program_id])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    receipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
