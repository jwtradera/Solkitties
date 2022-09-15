use anchor_lang::{
    prelude::*,
    solana_program::{clock, program::invoke, system_instruction},
};

declare_id!("Bb73evuaitYgZNXotqaF2UsNZsoLaAyGmdNwYR4Wmg6X");

pub mod constant;
pub mod error;
pub mod state;
pub mod util;

use crate::{constant::*, error::*, state::*, util::*};

#[program]
pub mod solkitties {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        ticket_price: u64,
        total_tickets: u64,
        end_date: i64,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        global_state.authority = ctx.accounts.authority.key();
        global_state.ticket_price = ticket_price;
        global_state.total_tickets = total_tickets;
        global_state.end_date = end_date;
        global_state.sold_tickets = 0;

        Ok(())
    }

    pub fn update_state(
        ctx: Context<UpdateState>,
        ticket_price: Option<u64>,
        total_tickets: Option<u64>,
        end_date: Option<i64>,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        if let Some(_ticket_price) = ticket_price {
            global_state.ticket_price = _ticket_price;
        }
        if let Some(_total_tickets) = total_tickets {
            global_state.total_tickets = _total_tickets;
        }
        if let Some(_end_date) = end_date {
            global_state.end_date = _end_date;
        }

        Ok(())
    }

    pub fn sale(ctx: Context<Sale>, amount: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        // Validate global state
        let now = clock::Clock::get()?.unix_timestamp.try_into().unwrap();
        require!(global_state.end_date >= now, SolkittiesError::SaleEnded);

        let sold_tickets = global_state
            .sold_tickets
            .checked_add(amount)
            .ok_or(SolkittiesError::MathOverflow)?;
        require!(
            sold_tickets <= global_state.total_tickets,
            SolkittiesError::OutTickets
        );

        // Validate user state
        let sale_state = &mut ctx.accounts.sale_state;
        let is_sale_zero = is_zero_account(&sale_state.to_account_info());
        if is_sale_zero {
            sale_state.authority = ctx.accounts.buyer.key();
            sale_state.tickets = 0;
        }

        let user_tickets = sale_state
            .tickets
            .checked_add(amount)
            .ok_or(SolkittiesError::MathOverflow)?;

        // Transfer SOL
        let sol_amount = global_state.ticket_price
            .checked_mul(amount)
            .ok_or(SolkittiesError::MathOverflow)?;

        require!(
            ctx.accounts.buyer.lamports() >= sol_amount,
            SolkittiesError::Insufficient
        );
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.buyer.key(),
                &ctx.accounts.authority.key(),
                sol_amount,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Store state
        global_state.sold_tickets = sold_tickets;
        sale_state.tickets = user_tickets;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [PREFIX],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<GlobalState>(),
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct UpdateState<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PREFIX],
        bump,
        has_one = authority
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Sale<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECKED: Validated in global state
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [PREFIX],
        bump,
        has_one = authority
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init_if_needed,
        seeds = [PREFIX, buyer.key().as_ref(), SALE],
        bump,
        payer = buyer,
        space = 8 + std::mem::size_of::<SaleState>(),
    )]
    pub sale_state: Box<Account<'info, SaleState>>,

    pub system_program: Program<'info, System>,
}
