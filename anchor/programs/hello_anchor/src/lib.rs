use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("CFzoprd9wxj64pZKCMQaRtpZWeJPgZ5WFEzhewDLjZH4");

#[program]
mod hello_anchor {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        let payer = &mut ctx.accounts.payer.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        msg!(
            "The payer is {:?} and the system program is {:?}",
            payer,
            system_program
        );

        msg!("Hello Solana");
        msg!("Our program's Program ID: {}", &id());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}
