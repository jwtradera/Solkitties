use anchor_lang::prelude::*;

#[error_code]
pub enum SolkittiesError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Sale ended")]
    SaleEnded,
    #[msg("Ticket amount is out of range")]
    OutTickets,
    #[msg("Tokens already sold")]
    AlreadySold,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Insufficient balance")]
    Insufficient,
}
