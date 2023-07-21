use  borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::PrintProgramError,
    program_pack::{IsInitialized,Pack,Sealed},
    pubkey::Pubkey,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct NameRecordHeader {
    
}