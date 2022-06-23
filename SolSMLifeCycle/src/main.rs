//On Chain Code - Rust Smart Contract (C,C++)
//off chain code - Rust SDK, Javascipt or third party sdks
//Transaction has two things - Signatures and Message
//Message consist of 4 things - Headers, Account Adddresses, Recent Blockhash and Instruction
//Header - Information about signatures
//Account address - Array of addresses that we are interacting with
//Hash of the last observed blockchain ledger
//Instruction is the only thing we should care about - Its the array of data that is used by smart contract to complete transaction

//Instruction
//Instruction has three part - Program ID, Account, Data

//Program ID - The smart contract the transaction will interact with 

//Account - Account keeps the track of state on a solana account
//Account is stored on the blockchain
//Account - Signer , Read only, Executable , Rent and Data
//Signer - The account that signs the transaction
//Read only - The account that is read only - Cannot modify
//Executable - Executable account are created at the time of deployment. Only executable account can process the information.All addition user account must be owned by the excutable account.
//Rent - Amount in Lamport that user have to pay to keep the account active or else account will be removed. Account can also be rent exempted
//Data - Serialise metadata that allows user to store custom data in the account on the blockchain


fn main() {
    println!("Hello, world!");
}
