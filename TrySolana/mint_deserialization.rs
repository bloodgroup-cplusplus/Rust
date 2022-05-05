//import Pack trait 

use solana_program::{
    program_pack::Pack;

};

//import type 

use spl_token ::{
    state::Mint,
}


// code inside of 'process_instruction '

//mint_account is of type ' AccountInfo' retrived previously 


let mint = Mint::unpack_from_slice ( &mint_account try_borrow_data()?)?;

msg!("mint decimals : {}", mint.decimals);



