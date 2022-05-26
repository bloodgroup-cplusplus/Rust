// import
use solana_program::{
    program_pack::Pack,
};
use spl_token::{
	error::TokenError, // custom errors defined in token program
	state::{Account as TokenAccount} // alias
}
// code inside of an instruction implementation
// get accounts
let accounts_iter = &mut accounts.iter();
// often you'll see people denote the AccountInfo with an `_ai` suffix to differentiate the account info and the deserialized account
let user_ai = next_account_info(accounts_iter)?;
let user_token_ai = next_account_info(accounts_iter)?;
// deserialize the token account
// if this account is not a token account, then the deserializing will fail and you're transaction will revert. This is what we want.
let user_token_account = TokenAccount::unpack_from_slice(&user_token_ai.try_borrow_data()?)?;
// check that the token account's owner is the user's public key
// throw an error if it doesn't
//
 if user_token_account.owner != *user.key {
	return Err(TokenError::OwnerMismatch.into()) // note the `.into` is used to convert the type from `TokenError` into whatever the `Err` function expects, in this case it's a `ProgramError` type, and as long as the conversion is possible it will go through.
}

