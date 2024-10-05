// // Import statement
// use anchor_lang::prelude::*;

// // RUST MACRO : macro fn that writes more code , takes a arg i.e. program address from keypair , PUBLIC KEY
// // Anchor will read this thing and expand it more into the Rust Code
// declare_id!("7wWfAXexLPgeUs9F543Ag92qyxxdU4NacDkDqQaY2BWa");

// // Body of function
// // The [program] is another MACRO
// #[program]
// pub mod learnv1 {
//     use super::*;

//     // Core body of the Program
//     // It takes is something known as Context
//     // This Initialize actually Initializes the whole Solana program, just like setting things up
//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(()) // Returns an okay which is just like 200 of the normal HTTp status Codes
//         // example of vending machine: setup item layout
//     }

//     // function to actually add the items
// }

// // Type declaration, declares what the type of Initialize is gonna be
// // The #derive stuff in Anchor magic to like a MACRO
// // Initialize here is simple a data type
// #[derive(Accounts)]
// pub struct Initialize {}
















// Import statement
use anchor_lang::prelude::*;

// RUST MACRO : macro fn that writes more code , takes a arg i.e. program address from keypair , PUBLIC KEY
// Anchor will read this thing and expand it more into the Rust Code
declare_id!("7wWfAXexLPgeUs9F543Ag92qyxxdU4NacDkDqQaY2BWa");

// Body of function
// The [program] is another MACRO
#[program]
pub mod learnv1 {
    use super::*;

    // Core body of the Program
    // It takes is something known as Context
    // This Initialize actually Initializes the whole Solana program, just like setting things up
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; // store bump seed in `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Create and initialize `Counter` account using a PDA as the address
    #[account(
        init,
        seeds = [b"counter"], // optional seeds for pda
        bump,                 // bump seed for pda
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    #[account(
        mut, //specify that the account in mutable
        seeds = [b"counter"], // optional seeds for pda, 
        bump = counter.bump,  // bump seed for pda stored in `Counter` account
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, // 8 bytes
    pub bump: u8,   // 1 byte
} 