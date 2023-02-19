// Code journal for Anchor program uses PDA feuture

// Every Anchor program must include this crate
// Crate contain all comonly Anchor components - Accounts, Errors, serialisation etc..
use anchor_lang::prelude::*;

// Macros to declare "constant" program ID - just program account public key
declare_id!("FFKtnYFyzPj1qFjE9epkrfYHJwZMdh8CvJrB6XsKeFVz");

// Program module annotation - main logic goes here
#[program]
pub mod anchor_program_example {
      // the same ^^^^ name as workspace created by anchor init

    // for "import" parent modules stuff
    use super::*;

    // And we gonna declare all program "endpoints" - 
    // functions, that represents any possible program instructions

    // anchor endpoint for instruction create_page_visits
    pub fn create_page_visits(
        // first input - "context" - a wrapper arround all nessesary program data
        // including accountss passed with this ix and some more stuff like program id etc..
        ctx: Context<CreatePageVisits>
    ) -> Result<()> {   // - note - anchor programs return just Result<()>, not wrapped enum <Result, Err> as like native one does
        // and just pass our context to create_page_visits method
        // We going to initialize smth
        create_page_visits(ctx)
    }

    // same things here, but another ix - increment_page_visits
    // so here we do some action++ on our initialized state
    pub fn increment_page_visits(
        // just a context here, but we can aslo get some instruction data with second input parameter
        ctx: Context<IncrementPageVisits>
    ) -> Result<()> {  
        // passing context included IncrementPageVisits stuff
        increment_page_visits(ctx)
    }
}

// business logic implementation for first instruction - create_page_visits
// simple initializing smth
pub fn create_page_visits(
  ctx: Context<CreatePageVisits>
) -> Result<()> {
  // as we know the Context (ctx) include all nessesary data such as accounts and program id
  // we can get instruction .accounts like a ctx struct field
  // so here we going to work with page_visits account
  ctx.accounts.page_visits.set_inner(
                          // ^^^ ??? not yet realized this ...
      // Create new PageVisits instance 
      // upd - looks like this is PDA account derived for every users 
      PageVisits::new(
          // counter's inital value
          0,  
          // a bump for PDA
          *ctx.bumps.get(PageVisits::SEED_PREFIX).expect("Bump not found."),
      )
  );

  // if everything above went well - return Ok(())
  Ok(())
}

// business logic implementation for second instruction - increment_page_visits
// to modify existing state
pub fn increment_page_visits(
  ctx: Context<IncrementPageVisits>
) -> Result<()> {
  // get page_visits account (as mutable - to make some changes)
  let page_visits = &mut ctx.accounts.page_visits;
  // and yeah - making smth with account - apply increment method
  page_visits.increment();

  // if everything above went well - return Ok(())
  Ok(())
}

/// Ands structs down below is a stuff like native Instruction module
/// here we should describe all accounts that corresponded ix must include
/// Also anchor give us an abbilities to make some magic like constrains, additional data etc..

#[derive(Accounts)] // this annotate that struct describes accounts
pub struct CreatePageVisits<'info> {
  // and here anchor help us with some magic
  #[account(                                              
      init,                                             // this account must be initialized
      space = PageVisits::ACCOUNT_SPACE,                // memory allocation for account we going to initialize
      payer = payer,                                    // who will be a fee payer
      seeds = [                                         // array of PDA seeds
          PageVisits::SEED_PREFIX.as_bytes().as_ref(),  // [0]
          user.key().as_ref(),                          // [1]
      ],
      bump,                                             // bump
  )] 
  // and here is the account (T = PageVisits)
  page_visits: Account<'info, PageVisits>, 

  // here is a System account wrapper  - just user's system account / keypair
  user: SystemAccount<'info>,

  #[account(mut)]         // here we annotate that account below must be mutable
  payer: Signer<'info>,   // signer

  // and system program account must also be present here
  system_program: Program<'info, System>,
}

// struct for increment ix
#[derive(Accounts)]
pub struct IncrementPageVisits<'info> {
  #[account(
      mut,                                              // mutable account
      seeds = [                                         // array of PDA seeds
          PageVisits::SEED_PREFIX.as_bytes().as_ref(),  // [0]
          user.key().as_ref(),                          // [1]
      ],
      bump,                                             // bump
  )]
  // state account (T = PageVisits)
  page_visits: Account<'info, PageVisits>,
  // Accnt wrapper^^^             ^^^ account struct/type declared below

  user: SystemAccount<'info>,

  #[account(mut)]         // mutable accnt
  payer: Signer<'info>,   // signer

  // system program account must also be present here
  system_program: Program<'info, System>,
}

// Describe state account here
#[account]    // and anchor under the hood check if account owner == program ID we declered above
pub struct PageVisits {
    pub page_visits: u32,       // state field (counter of visits)
    pub bump: u8,               // PDA bump
}

// PageVisits methods implementations
impl PageVisits {

    // const to store amount of bytes of account data (bump - 8 bit, counter - 32 bit)
    pub const ACCOUNT_SPACE: usize = 8 + 32;

    // const to store seed preffix - str slice
    pub const SEED_PREFIX: &'static str = "page_visits";

    // Constructor - returns an PageVisits instance with given counter and bump
    pub fn new(page_visits: u32, bump: u8) -> Self {
        PageVisits {
            page_visits,      // counter
            bump,             // bump
        }
    }

    // method implements "counter++
    pub fn increment(&mut self) {
        self.page_visits += 1;  // increment page_visits
    }
}