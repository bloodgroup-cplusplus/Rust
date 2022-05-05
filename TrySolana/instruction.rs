// don't forget to import !

use solana_program ::{
    account_info::{AccountInfo, next_account_info}

};


// code inside of process_instruction 


let accounts_iter = & mut accounts iter();// Accounts here refers to the argument denoting the array of 'AccountInfo's'
let account_one = next_account_info(accounts_iter)?;
let account_two = next_account_info(accounts_iter)?;
let account_three = next_account_info(accounts_iter)?;


