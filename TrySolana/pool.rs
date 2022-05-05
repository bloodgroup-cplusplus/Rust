// in state.rs 
//

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::pubkey::Pubkey;

pub struct Pool 
{
    pub mint_a : Pubkey,
    pub mint_b : Pubkey,
    pub mint :Pubkey,
    pub fee:u64,
    pub fee_decimals :u64,
}

//code inside of process_instruction 
//pool_account of is of type " AccountInfo" retrived previously 
//
let pool = Pool::try_from_slice (&pool_account.try_borrow_data()?)?;
msg!("ppl fee:{}", pool.fee);



