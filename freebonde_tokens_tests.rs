rust
// This file is to test the freebonde_tokens_program.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use freebonde_tokens::*;
use freebonde_tokens::ErrorCode;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;
    use anchor_lang::system_program::ID as system_program_id;
    use anchor_spl::token::ID as token_program_id;
    use solana_program::instruction::AccountMeta;
    use solana_program::pubkey::Pubkey;
    use solana_program_test::*;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_sdk::account::Account as SolanaAccount;
    use solana_sdk::transaction::Transaction;
    use std::mem::size_of;

    #[tokio::test]
    async fn test_initialize_council() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();

        // Fund accounts
        program_test.add_account(
            authority.pubkey(),
            solana_sdk::account::Account::new(10000000000, 0, &system_program_id),
        );

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test invalid council size
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    2,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x0");

        // Test invalid quorum percentage
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    50,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x1");

        // Test invalid term length
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    89 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x2");
        
        // Test council already active (First, initialize the council correctly)
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Now try to initialize again
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_initialize_token() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
    
        // Fund accounts
        program_test.add_account(
            authority.pubkey(),
            solana_sdk::account::Account::new(10000000000, 0, &system_program_id),
        );
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_token(
                    &program_id,
                    &authority.pubkey(),
                    &farm_token_mint.pubkey(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }


    #[tokio::test]
    async fn test_nominate_council_member() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        
        // Fund accounts
        program_test.add_account(
            authority.pubkey(),
            solana_sdk::account::Account::new(10000000000, 0, &system_program_id),
        );
        program_test.add_account(
            nominee.pubkey(),
            solana_sdk::account::Account::new(10000000000, 0, &system_program_id),
        );

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_token(
                    &program_id,
                    &authority.pubkey(),
                    &farm_token_mint.pubkey(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Nominee Token Account
        let accounts = vec![
            AccountMeta::new(nominee_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(nominee.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to nominee_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &nominee_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::nominate_council_member(
                    &program_id,
                    &nominee.pubkey(),
                    &council.pubkey(),
                    &nomination.pubkey(),
                    &nominee_token_account.pubkey(),
                    &farm_token_mint.pubkey(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_vote_for_council_member() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let voter_record = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let voter_token_account = Keypair::new();
        
        // Fund accounts
        program_test.add_account(
            authority.pubkey(),
            SolanaAccount::new(10000000000, 0, &system_program_id),
        );
        program_test.add_account(
            nominee.pubkey(),
            SolanaAccount::new(10000000000, 0, &system_program_id),
        );
        program_test.add_account(
            voter.pubkey(),
            SolanaAccount::new(10000000000, 0, &system_program_id),
        );
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_council(
                    &program_id,
                    &authority.pubkey(),
                    &council.pubkey(),
                    3,
                    51,
                    90 * 24 * 60 * 60,
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::initialize_token(
                    &program_id,
                    &authority.pubkey(),
                    &farm_token_mint.pubkey(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Create Nominee Token Account
        let accounts = vec![
            AccountMeta::new(nominee_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(nominee.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Mint tokens to nominee_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &nominee_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();
    
        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(
            &[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Voter Token Account
        let accounts = vec![
            AccountMeta::new(voter_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(voter.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &voter], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to voter_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &voter_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Vote for a member
        let mut transaction = Transaction::new_with_payer(
            &[instruction::vote_for_council_member(&program_id, &voter.pubkey(), &council.pubkey(), &nomination.pubkey(), &voter_record.pubkey(), &voter_token_account.pubkey(), &farm_token_mint.pubkey())],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &voter, &voter_record], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_finalize_council_election() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let voter_record = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let voter_token_account = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Nominee Token Account
        let accounts = vec![AccountMeta::new(nominee_token_account.pubkey(), false), AccountMeta::new_readonly(farm_token_mint.pubkey(), false), AccountMeta::new_readonly(nominee.pubkey(), true), AccountMeta::new_readonly(token_program_id, false), AccountMeta::new_readonly(system_program_id, false), AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false)];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Mint tokens to nominee_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(&token_program_id, &farm_token_mint.pubkey(), &nominee_token_account.pubkey(), &authority.pubkey(), &[], 1000).unwrap();
        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_emergency_action() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let voter_record = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let voter_token_account = Keypair::new();
        let proposer = Keypair::new();
        let emergency_action = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        //Create emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::create_emergency_action(&program_id, &proposer.pubkey(), &council.pubkey(), &emergency_action.pubkey(), "test".to_string(), "test".to_string(), 0, vec![])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &emergency_action], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_vote_on_emergency_action() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let voter_record = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let voter_token_account = Keypair::new();
        let proposer = Keypair::new();
        let emergency_action = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        //Create emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::create_emergency_action(&program_id, &proposer.pubkey(), &council.pubkey(), &emergency_action.pubkey(), "test".to_string(), "test".to_string(), 0, vec![])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &emergency_action], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Vote on emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(
            &[instruction::vote_on_emergency_action(&program_id, &voter.pubkey(), &council.pubkey(), &emergency_action.pubkey())],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &voter], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Vote on emergency action (error case - not a council member)
        let not_council_member = Keypair::new();
        program_test.add_account(not_council_member.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        let mut transaction = Transaction::new_with_payer(
            &[instruction::vote_on_emergency_action(&program_id, &not_council_member.pubkey(), &council.pubkey(), &emergency_action.pubkey())],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &not_council_member], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x9");
    
    }

    #[tokio::test]
    async fn test_execute_emergency_action() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let voter_record = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let voter_token_account = Keypair::new();
        let proposer = Keypair::new();
        let emergency_action = Keypair::new();
        let executor = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(executor.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        //Create emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::create_emergency_action(&program_id, &proposer.pubkey(), &council.pubkey(), &emergency_action.pubkey(), "test".to_string(), "test".to_string(), 0, vec![])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &emergency_action], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Vote on emergency action
        let mut transaction = Transaction::new_with_payer(&[instruction::vote_on_emergency_action(&program_id, &voter.pubkey(), &council.pubkey(), &emergency_action.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &voter], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Execute emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::execute_emergency_action(&program_id, &executor.pubkey(), &council.pubkey(), &emergency_action.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &executor], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_cancel_emergency_action() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let voter = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let proposer = Keypair::new();
        let emergency_action = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Create emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::create_emergency_action(&program_id, &proposer.pubkey(), &council.pubkey(), &emergency_action.pubkey(), "test".to_string(), "test".to_string(), 0, vec![])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &emergency_action], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Cancel emergency action (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::cancel_emergency_action(&program_id, &authority.pubkey(), &emergency_action.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_initialize_crisis_management() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let crisis_manager = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_crisis_management(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &crisis_manager], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_declare_crisis() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let crisis_manager = Keypair::new();
        let crisis_record = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        //Initialize Crisis manager
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_crisis_management(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &crisis_manager], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::declare_crisis(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey(), &crisis_record.pubkey(), 0, "Test description".to_string()),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority, &crisis_record], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_resolve_crisis() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let council = Keypair::new();
        let nominee = Keypair::new();
        let nomination = Keypair::new();
        let farm_token_mint = Keypair::new();
        let nominee_token_account = Keypair::new();
        let crisis_manager = Keypair::new();
        let crisis_record = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(nominee.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize the council correctly
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_council(&program_id, &authority.pubkey(), &council.pubkey(), 3, 51, 90 * 24 * 60 * 60)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Nominate a member
        let mut transaction = Transaction::new_with_payer(&[instruction::nominate_council_member(&program_id, &nominee.pubkey(), &council.pubkey(), &nomination.pubkey(), &nominee_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &nominee, &nomination], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Finalize council election
        let mut transaction = Transaction::new_with_payer(&[instruction::finalize_council_election(&program_id, &authority.pubkey(), &council.pubkey(), &vec![nomination.pubkey()])], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Initialize Crisis manager
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_crisis_management(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &crisis_manager], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Declare Crisis
        let mut transaction = Transaction::new_with_payer(
            &[instruction::declare_crisis(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey(), &crisis_record.pubkey(), 0, "Test description".to_string())],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority, &crisis_record], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(&[instruction::resolve_crisis(&program_id, &authority.pubkey(), &council.pubkey(), &crisis_manager.pubkey(), &crisis_record.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_proposal() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let dao = Keypair::new();
        let proposal = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create DAO
        let space = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + size_of::<i64>() + size_of::<u64>();
        let create_account_ix = system_program::create_account(
            &payer.pubkey(),
            &dao.pubkey(),
            banks_client.get_rent().await.unwrap().minimum_balance(space),
            space as u64,
            &program_id,
        );
        let mut transaction = Transaction::new_with_payer(&[create_account_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &dao], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::create_proposal(&program_id, &proposer.pubkey(), &proposal.pubkey(), &dao.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "test".to_string(), "test".to_string(), 0, vec![], vec![], 0),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &proposer, &proposal], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_cast_vote() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let dao = Keypair::new();
        let proposal = Keypair::new();
        let voter = Keypair::new();
        let voter_token_account = Keypair::new();
        let voter_record = Keypair::new();
        let voter_token_data = Keypair::new();
        let voter_token_data_mut = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Voter Token Account
        let accounts = vec![
            AccountMeta::new(voter_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(voter.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &voter], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to voter_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &voter_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create DAO
        let space = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + size_of::<i64>() + size_of::<u64>();
        let create_account_ix = system_program::create_account(&payer.pubkey(), &dao.pubkey(), banks_client.get_rent().await.unwrap().minimum_balance(space), space as u64, &program_id);
        let mut transaction = Transaction::new_with_payer(&[create_account_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &dao], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposal
        let mut transaction = Transaction::new_with_payer(&[instruction::create_proposal(&program_id, &proposer.pubkey(), &proposal.pubkey(), &dao.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "test".to_string(), "test".to_string(), 0, vec![], vec![], 0)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &proposal], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        //Test cast_vote
        let mut transaction = Transaction::new_with_payer(&[instruction::cast_vote(&program_id, &voter.pubkey(), &proposal.pubkey(), &voter_record.pubkey(), &voter_token_account.pubkey(), &voter_token_data.pubkey(), &voter_token_data_mut.pubkey(), &farm_token_mint.pubkey(), 0)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &voter, &voter_record], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_execute_proposal() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let dao = Keypair::new();
        let proposal = Keypair::new();
        let voter = Keypair::new();
        let voter_token_account = Keypair::new();
        let voter_record = Keypair::new();
        let voter_token_data = Keypair::new();
        let voter_token_data_mut = Keypair::new();
        let executor = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(executor.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Voter Token Account
        let accounts = vec![
            AccountMeta::new(voter_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(voter.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &voter], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to voter_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &voter_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create DAO
        let space = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + size_of::<i64>() + size_of::<u64>();
        let create_account_ix = system_program::create_account(&payer.pubkey(), &dao.pubkey(), banks_client.get_rent().await.unwrap().minimum_balance(space), space as u64, &program_id);
        let mut transaction = Transaction::new_with_payer(&[create_account_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &dao], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposal
        let mut transaction = Transaction::new_with_payer(&[instruction::create_proposal(&program_id, &proposer.pubkey(), &proposal.pubkey(), &dao.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "test".to_string(), "test".to_string(), 0, vec![], vec![], 0)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &proposal], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Execute Proposal
        let mut transaction = Transaction::new_with_payer(&[instruction::execute_proposal(&program_id, &executor.pubkey(), &proposal.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &executor], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_cancel_proposal() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let dao = Keypair::new();
        let proposal = Keypair::new();
        let voter = Keypair::new();
        let voter_token_account = Keypair::new();
        let voter_record = Keypair::new();
        let voter_token_data = Keypair::new();
        let voter_token_data_mut = Keypair::new();
        let executor = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(voter.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(executor.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create DAO
        let space = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + size_of::<i64>() + size_of::<u64>();
        let create_account_ix = system_program::create_account(&payer.pubkey(), &dao.pubkey(), banks_client.get_rent().await.unwrap().minimum_balance(space), space as u64, &program_id);
        let mut transaction = Transaction::new_with_payer(&[create_account_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &dao], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposal
        let mut transaction = Transaction::new_with_payer(&[instruction::create_proposal(&program_id, &proposer.pubkey(), &proposal.pubkey(), &dao.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "test".to_string(), "test".to_string(), 0, vec![], vec![], 0)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &proposal], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test cancel proposal (happy path)
        let mut transaction = Transaction::new_with_payer(&[instruction::cancel_proposal(&program_id, &proposer.pubkey(), &proposal.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test cancel proposal (error case - not authorized)
        let not_proposer = Keypair::new();
        program_test.add_account(not_proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        let mut transaction = Transaction::new_with_payer(&[instruction::cancel_proposal(&program_id, &not_proposer.pubkey(), &proposal.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &not_proposer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x21");
    }

    #[tokio::test]
    async fn test_create_petition() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let petition = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::create_petition(&program_id, &proposer.pubkey(), &petition.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "Test Petition".to_string(), "Test Description".to_string(), 10),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &proposer, &petition], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Test error condition: invalid signature requirement
        let mut transaction = Transaction::new_with_payer(&[instruction::create_petition(&program_id, &proposer.pubkey(), &petition.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "Test Petition".to_string(), "Test Description".to_string(), 0)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &petition], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x22");
    }

    #[tokio::test]
    async fn test_sign_petition() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let petition = Keypair::new();
        let signer = Keypair::new();
        let signer_token_account = Keypair::new();
        let petition_signature = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(signer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Signer Token Account
        let accounts = vec![
            AccountMeta::new(signer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(signer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &signer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to signer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &signer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create petition
        let mut transaction = Transaction::new_with_payer(&[instruction::create_petition(&program_id, &proposer.pubkey(), &petition.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "Test Petition".to_string(), "Test Description".to_string(), 10)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &petition], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(&[instruction::sign_petition(&program_id, &signer.pubkey(), &petition.pubkey(), &petition_signature.pubkey(), &signer_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &signer, &petition_signature], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_convert_petition_to_proposal() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));

        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let proposer = Keypair::new();
        let proposer_token_account = Keypair::new();
        let petition = Keypair::new();
        let signer = Keypair::new();
        let signer_token_account = Keypair::new();
        let petition_signature = Keypair::new();
        let dao = Keypair::new();
        let proposal = Keypair::new();

        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(proposer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(signer.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));

        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create Proposer Token Account
        let accounts = vec![
            AccountMeta::new(proposer_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(proposer.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Mint tokens to proposer_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &proposer_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();

        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create petition
        let mut transaction = Transaction::new_with_payer(&[instruction::create_petition(&program_id, &proposer.pubkey(), &petition.pubkey(), &proposer_token_account.pubkey(), &farm_token_mint.pubkey(), "Test Petition".to_string(), "Test Description".to_string(), 1)], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &petition], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Sign petition
        let mut transaction = Transaction::new_with_payer(&[instruction::sign_petition(&program_id, &signer.pubkey(), &petition.pubkey(), &petition_signature.pubkey(), &signer_token_account.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &signer, &petition_signature], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Create DAO
        let space = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + size_of::<i64>() + size_of::<u64>();
        let create_account_ix = system_program::create_account(&payer.pubkey(), &dao.pubkey(), banks_client.get_rent().await.unwrap().minimum_balance(space), space as u64, &program_id);
        let mut transaction = Transaction::new_with_payer(&[create_account_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &dao], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Test happy path
        let mut transaction = Transaction::new_with_payer(&[instruction::convert_petition_to_proposal(&program_id, &proposer.pubkey(), &petition.pubkey(), &proposal.pubkey(), &dao.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &proposer, &proposal], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_transfer_tokens() {
        let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let mut program_test = ProgramTest::new("freebonde_tokens", program_id, processor!(process_instruction));
    
        // Create accounts
        let authority = Keypair::new();
        let farm_token_mint = Keypair::new();
        let sender = Keypair::new();
        let sender_token_account = Keypair::new();
        let receiver = Keypair::new();
        let receiver_token_account = Keypair::new();
    
        // Fund accounts
        program_test.add_account(authority.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(sender.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
        program_test.add_account(receiver.pubkey(), SolanaAccount::new(10000000000, 0, &system_program_id));
    
        // Start the test
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
        // Initialize Token
        let mut transaction = Transaction::new_with_payer(&[instruction::initialize_token(&program_id, &authority.pubkey(), &farm_token_mint.pubkey())], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority, &farm_token_mint], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Create Sender Token Account
        let accounts = vec![
            AccountMeta::new(sender_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(sender.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &sender], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Mint tokens to sender_token_account
        let mint_to_ix = token::spl_token::instruction::mint_to(
            &token_program_id,
            &farm_token_mint.pubkey(),
            &sender_token_account.pubkey(),
            &authority.pubkey(),
            &[],
            1000,
        ).unwrap();
    
        let mut transaction = Transaction::new_with_payer(&[mint_to_ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Create Receiver Token Account
        let accounts = vec![
            AccountMeta::new(receiver_token_account.pubkey(), false),
            AccountMeta::new_readonly(farm_token_mint.pubkey(), false),
            AccountMeta::new_readonly(receiver.pubkey(), true),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ];
        let data = vec![]; // No data for this instruction
        let ix = solana_program::instruction::Instruction::new_with_bincode(token_program_id, &data, accounts);
        let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &receiver], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Test happy path
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::transfer_tokens(&program_id, &sender.pubkey(), &sender_token_account.pubkey(), &receiver_token_account.pubkey(), &farm_token_mint.pubkey(), 500),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &sender], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    
        // Test error case: insufficient funds
        let mut transaction = Transaction::new_with_payer(
            &[
                instruction::transfer_tokens(&program_id, &sender.pubkey(), &sender_token_account.pubkey(), &receiver_token_account.pubkey(), &farm_token_mint.pubkey(), 1000),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &sender], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap().to_string(), "Custom program error: 0x26");
    }
}