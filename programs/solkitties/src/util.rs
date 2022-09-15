use anchor_lang::prelude::*;


pub fn is_zero_account(account_info: &AccountInfo) -> bool {
    account_info.data.borrow().iter().all(|byte| byte.eq(&0))
}