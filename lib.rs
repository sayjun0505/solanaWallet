use anchor_lang::prelude::*;
use solana_program::system_instruction;
declare_id!("BgnU5WiDbNDgHQBqu6d4YpnrUChtJkTuo2Wzfi19USdR");
#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[program]
pub mod solana_lamport_transfer {    
    use super::*;    
    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {        
        let from_account = &ctx.accounts.from;        
        let to_account = &ctx.accounts.to;        // Create the transfer instruction        
        let transfer_instruction = system_instruction::transfer(
            from_account.key, 
            to_account.key, 
            amount
        ); // Invoke the transfer instruction        
        anchor_lang::solana_program::program::invoke_signed(            
            &transfer_instruction,            
            &[                
                from_account.to_account_info(),                
                to_account.clone(),                
                ctx.accounts.system_program.to_account_info(),            
            ],            
            &[],        
        )?;        
        Ok(())    
    }
}