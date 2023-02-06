// Ok, Let's think why we need the imports below...

/// tools for (de)serialize data 
use borsh::{ BorshDeserialize, BorshSerialize };
// solana
use solana_program::{
    /// while we need to process passed accounts,
    /// we can use this next_account_info fn to iterate by acconts and get data from each one.
    /// Also, Solana give us a type AccountInfo
    account_info::{
        next_account_info, AccountInfo  
    },

    // Solana's macros to point "fn main" of smart-contract
    entrypoint, 
    // .. and Solana's Result wrapper 
    entrypoint::ProgramResult, 
    // this macros we're using for output messages 
    msg, 
    // we can make cross-program invocation using this fn
    program::invoke,
    // Solana's Error enum
    program_error::ProgramError,
    // Pubkey type
    pubkey::Pubkey,
    // Rent type  
    rent::Rent,
    // 
    system_instruction,
    // this help us get cluster data
    sysvar::Sysvar,
};


#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);     // Here we declare process_instruction as a "main" fn


pub fn process_instruction(
    program_id: &Pubkey,        // smart-cantract public key
    accounts: &[AccountInfo],   // passed accounts
    instruction_data: &[u8],    // serialized instruction data
) -> ProgramResult {

    // Trying to deserialize instruction data using Borch
    match PowerStatus::try_from_slice(&instruction_data) {
        /// On Success - goto initialize
        /// ...and looks like here is simple on/off boolean ...
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {}, // deserialization f#c&up?
    }

    /// if we got another ix ...
    /// deserealization...
    match SetPowerStatus::try_from_slice(&instruction_data) {
        // ok, now we have to change stored power status
        // on success - goto switch_power where we'll specify status (string?...wtf)
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        // is here^^^ a string?
        Err(_) => {}, // deserialization f#c&up?
    }

    /// in case previous two match statements didn't find as we expect 
    /// - seems ix is incorrect - return err...
    Err(ProgramError::InvalidInstructionData)
}



pub fn initialize(
    program_id: &Pubkey,        // smart-cantract public key
    accounts: &[AccountInfo],   // Accounts set
    power_status: PowerStatus,  // power_status 
) -> ProgramResult {
    // make an iterator for accounts
    let accounts_iter = &mut accounts.iter();
    // Looks like first account contain data we should store?
    let power = next_account_info(accounts_iter)?;
    // user here (signer?)
    let user = next_account_info(accounts_iter)?;
    // system program account
    let system_program = next_account_info(accounts_iter)?;
    // looks like here we need to calculate data allocation
    let account_span = (power_status.try_to_vec()?).len();
    // ok, make runtime query to get minimum rent for account above
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // invoke system program to create account to store some state...
    // ...upd - this account for store status 
    invoke(
        &system_instruction::create_account(  // create account ix
            &user.key,                        // signer?
            &power.key,                       // state account pubkey?
            lamports_required,                // rent we calculated before
            account_span as u64,              // data len? (mean pre-allocated space)
            program_id,                       // account owner?
        ),
        &[
            user.clone(), power.clone(), system_program.clone()
          // signer?^^^   state?^^^         invoke to^^
        ]
    )?;

    // serialize status. (why?)
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;   // didn't realize this

    // if all above worked fine - return ok
    Ok(())
}

// And yes, here we'll change power state
pub fn switch_power(
    /// Account set (not clear for me)
    /// I expect here: 
    /// 1) state account,
    /// 2) ...
    accounts: &[AccountInfo],

    // we'll change status to this? Or it's just a some notification ...
    // ...UPD (after read some code below) - ha) ok, looks like this is username (who change a status)
    name: String,
) -> ProgramResult {
    // make an iterator for acoounts
    let accounts_iter = &mut accounts.iter();
    // yeah, this account stored power state (made via cpi in initilize fn)
    let power = next_account_info(accounts_iter)?;
    
    // trying to deserialize from current account
    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    // just reverse bool 
    power_status.is_on = !power_status.is_on;

    // ...and serialize changed data
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    // make output info who is making changes
    msg!("{} is pulling the power switch!", &name);

    // print current power status
    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    
    // if all above worked fine - return ok
    Ok(())
}


// Set attributes to be able (de)serialize and debug struct  
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    // Person who turn power on/off
    pub name: String,
}

// Set attributes to be able (de)serialize and debug struct 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    // power status boolean
    pub is_on: bool,
}