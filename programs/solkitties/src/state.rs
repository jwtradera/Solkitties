use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub authority: Pubkey,
    pub total_tickets: u64,
    pub ticket_price: u64,
    pub end_date: i64,
    pub sold_tickets: u64,
}

#[account]
#[derive(Default)]
pub struct SaleState {
    pub authority: Pubkey,
    pub tickets: u64,
}
