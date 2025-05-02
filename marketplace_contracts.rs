rust
#![allow(unused)]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    borsh::try_from_slice_unchecked,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
    instruction::{AccountMeta, Instruction},
    hash::Hash,
};

// Define the error codes for the program
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum ErrorCode {
    InvalidInstruction,
    NotEnoughTokens,
    InvalidAccountData,
    AccountNotInitialized,
    Unauthorized,
    ListingNotFound,
    PurchaseNotFound,
    CrisisModeAlreadyActive,
    CrisisModeNotActive,
    InvalidTokenAmount,
    InvalidFeeAmount,
    InvalidNFTAmount,
    NFTNotFound,
    ProposalNotFound,
    StakingNotFound,
    InvalidStakingDuration,
    InsufficientFunds,
    InvalidTimestamp,
    InvalidAccountOwner,
    InvalidAccountSigner,
    InvalidProposalTier,
    InvalidVote,
    InvalidNFTType,
    // New Error Codes
    InvalidMarketplaceFee,
    InvalidDisputePeriod,
    InvalidRatingPeriod,
    UnauthorizedAccess,
    InvalidPrice,
    InvalidQuantity,
    InvalidDescription,
    MarketplacePaused,
    ListingNotActive,
    InsufficientQuantity,
    MathOverflow,
    InvalidOrderStatus,
    InvalidRatingValue,
    AlreadyRated,
    TokenMintMismatch,
    InvalidMintAuthority,
    InvalidTokenAccountOwner,
    RatingPeriodEnded,
}

// Define the crisis state
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum CrisisState {
    Active,
    Inactive,
}

// Define the priority group
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum PriorityGroup {
    Low,
    Medium,
    High,
}

// Define the NFT types
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum NFTType {
    DigitalFarm,
    GrowingSlot,
    HarvestProduct,
}

// Define the proposal tiers
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum ProposalTier {
    Level1, // Community Proposals
    Level2, // Enhancement Proposals
    Level3, // Core Proposals
}

// Define the order status
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum OrderStatus {
    Pending = 0,
    Completed = 1,
    Disputed = 2,
}

// Helper function to check marketplace account access
fn check_marketplace_account_access(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    check_admin: bool,
) -> Result<(AccountInfo, Option<AccountInfo>), ProgramError> {
    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let marketplace_state_account = next_account_info(accounts_iter)?;
    let admin_account = if check_admin {
        Some(next_account_info(accounts_iter)?)
    } else {
        None
    };

    // Check account ownership
    if marketplace_state_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if admin signed if needed
    if check_admin && !admin_account.unwrap().is_signer {
        msg!("Admin signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok((marketplace_state_account.clone(), admin_account))
}

// Define the struct for managing the marketplace state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MarketplaceState {
    pub authority: Pubkey, // Changed from admin to authority
    pub fee: u16,         // Added fee
    pub dispute_period: i64, // Added dispute period
    pub min_rating_period: i64, // Added rating period
    pub name: String,       // Added marketplace name
    pub created_at: i64,    // Added created at
    pub updated_at: i64,    // Added updated at
    pub paused: bool,      // Added paused state
    pub total_listings: u64,   // Added listings count
    pub total_transactions: u64, // Added transactions count
    pub priorityGroup: PriorityGroup,
    pub maxListing: u32,
    pub currentListing: u32,
    pub crisisState: CrisisState,
    pub crisisTreasury: u64,
}

// Define the struct for managing GROW tokens
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GrowToken {
    pub mint: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
}

// Define the struct for managing FARM tokens
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FarmToken {
    pub mint: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
}

// Define the struct for a listing in the marketplace
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Listing {
    pub id: u32,
    pub seller: Pubkey,
    pub marketplace: Pubkey, // Added marketplace
    pub mint: Pubkey,         // Added mint
    pub price: u64,           // Price in GROW
    pub quantity: u64,
    pub quantity_remaining: u64, // Added remaining quantity
    pub description: String, // Added description
    pub is_organic: bool,   // Added is_organic
    pub nutritional_data: String, // Added nutritional data
    pub harvest_nft: Option<Pubkey>, // Added harvest NFT
    pub created_at: i64,       // Added created at
    pub updated_at: i64,       // Added updated at
    pub active: bool,         // Added active flag
    pub timestamp: u64,
}

// Define the struct for a purchase in the marketplace
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Purchase {
    pub id: u32,
    pub buyer: Pubkey,
    pub listing_id: u32,
    pub price: u64, // Price in GROW
    pub quantity: u32,
    pub timestamp: u64,
}
// Struct for a transaction
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Transaction {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub total_price: u64,
    pub marketplace_fee: u64,
    pub net_price: u64,
    pub quantity: u64,
    pub created_at: i64,
    pub updated_at: i64,
}
// Struct for an order
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Order {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub status: u8,         // Using u8 for OrderStatus enum
    pub completed_at: Option<i64>,
    pub seller_rated: bool,
    pub updated_at: i64,
    pub transaction: Pubkey,
}
// Struct for a dispute
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Dispute {
    pub resolved_at: Option<i64>,
}
// Struct for a buyer rating
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyerRating {
    pub rating: u8,
    pub comment: String,
    pub rated_at: i64,
}
// Define the struct for Digital Farm NFT
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DigitalFarmNFT {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub tier: u8, // Standard, Premium, Elite
    pub daily_rewards_multiplier: f32,
}

// Define the struct for Growing Slot NFT
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GrowingSlotNFT {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub farm_nft_mint: Pubkey,
    pub crop_type: String,
}

// Define the struct for Harvest Product NFT
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HarvestProductNFT {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub farm_nft_mint: Pubkey,
    pub growing_slot_nft_mint: Pubkey,
    pub harvest_data: String,
    pub quality_score: u32,
}

// Define the struct for governance proposal
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GovernanceProposal {
    pub id: u32,
    pub proposer: Pubkey,
    pub description: String,
    pub tier: ProposalTier,
    pub yes_votes: u64,
    pub no_votes: u64,    
    pub voting_start_time: u64,
    pub voting_end_time: u64,
    pub has_voted: Vec<Pubkey>,
    pub executed: bool, 
}

// Define the struct for stakers
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Staking {
    pub staker: Pubkey,
    pub amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub reward_multiplier: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Unstaking {
    pub unstaker: Pubkey,
    pub amount: u64,
    pub start_time: u64,
    pub end_time: u64,
}

// Define the struct for activate crisis mode
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ActivateCrisisMode {
    pub admin: Pubkey,
}

// Define the struct for deactivate crisis mode
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DeactivateCrisisMode {
    pub admin: Pubkey,
}

// Define the struct for update priority group
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdatePriorityGroup {
    pub admin: Pubkey,
    pub new_priority_group: PriorityGroup,
}

// Define the struct for daily check-in rewards
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DailyCheckIn {
    pub user: Pubkey,
    pub last_check_in: u64,
    pub last_check_in_day: u64,
    pub streak: u32,
}

// Define the struct for base insurance fund
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BaseInsuranceFund {
    pub treasury: u64,
}

// Define the struct for premium protection
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PremiumProtection {
    pub user: Pubkey,
    pub amount: u64,
}

// Marketplace Initialized Event
pub struct MarketplaceInitializedEvent {
    marketplace_id: Pubkey,
    authority: Pubkey,
    fee: u16,
    dispute_period: i64,
    rating_period: i64,
    created_at: i64,
}
// Marketplace Updated Event
pub struct MarketplaceUpdatedEvent {
    marketplace_id: Pubkey,
    updated_at: i64,
}

// Marketplace Paused Event
pub struct MarketplacePausedEvent {
    marketplace_id: Pubkey,
    updated_at: i64,
}
// Marketplace Unpaused Event
pub struct MarketplaceUnpausedEvent {
    marketplace_id: Pubkey,
    updated_at: i64,
}

// Listing Created Event
pub struct ListingCreatedEvent {
    listing_id: Pubkey,
    seller: Pubkey,
    mint: Pubkey,
    price: u64,
    quantity: u64,
    created_at: i64,
    harvest_nft: Option<Pubkey>,
}

// Listing Updated Event
pub struct ListingUpdatedEvent {
    listing_id: Pubkey,
    updated_at: i64,
}

// Listing Deactivated Event
pub struct ListingDeactivatedEvent {
    listing_id: Pubkey,
    updated_at: i64,
}

// Purchase Event
pub struct PurchaseEvent {
    transaction_id: Pubkey,
    buyer: Pubkey,
    seller: Pubkey,
    mint: Pubkey,
    total_price: u64,
    quantity: u64,
    created_at: i64,
}

// Define the main entry point for the program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Extract the instruction code from the instruction data
    let instruction_code: u8 = instruction_data[0];

    // Match the instruction code to the corresponding function
    //SPL_TOKEN id
    let spl_token_id: [u8; 32] = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    match instruction_code {
        0 => init_marketplace(program_id, accounts, &instruction_data[1..]),             // Initialize marketplace
        1 => create_listing(program_id, accounts, &instruction_data[1..]),               // Create listing
        2 => update_listing(program_id, accounts, &instruction_data[1..]),             // Update listing
        3 => delete_listing(program_id, accounts, &instruction_data[1..]),             // Delete listing
        4 => buy_listing(program_id, accounts, &instruction_data[1..]),                // Buy listing
        5 => mint_grow(program_id, accounts, &instruction_data[1..]),                    // Mint GROW
        6 => mint_farm(program_id, accounts, &instruction_data[1..]),                     // Mint FARM
        7 => transfer_grow(program_id, accounts, &instruction_data[1..]),                // Transfer GROW
        8 => transfer_farm(program_id, accounts, &instruction_data[1..]),                // Transfer FARM
        9 => burn_grow(program_id, accounts, &instruction_data[1..]),                     // Burn GROW
        10 => burn_farm(program_id, accounts, &instruction_data[1..]),                    // Burn FARM
        11 => calculate_fees(program_id, accounts, &instruction_data[1..]),               // Calculate fees
        12 => activate_crisis_mode(program_id, accounts, &instruction_data[1..]),        // Activate crisis mode
        13 => deactivate_crisis_mode(program_id, accounts, &instruction_data[1..]),      // Deactivate crisis mode
        14 => mint_digital_farm_nft(program_id, accounts, &instruction_data[1..]),       // Mint Digital Farm NFT
        15 => mint_growing_slot_nft(program_id, accounts, &instruction_data[1..]),       // Mint Growing Slot NFT
        16 => mint_harvest_product_nft(program_id, accounts, &instruction_data[1..]),   // Mint Harvest Product NFT
        17 => transfer_nft(program_id, accounts, &instruction_data[1..]),                // Transfer NFT
        18 => burn_nft(program_id, accounts, &instruction_data[1..]),                    // Burn NFT
        19 => redeem_harvest_product_nft(program_id, accounts, &instruction_data[1..]), // Redeem Harvest Product NFT
        20 => compost_harvest_product_nft(program_id, accounts, &instruction_data[1..]), // Compost Harvest Product NFT
        21 => submit_proposal(program_id, accounts, &instruction_data[1..]),             // Submit proposal
        22 => vote_on_proposal(program_id, accounts, &instruction_data[1..]),             // Vote on proposal
        23 => stake_grow(program_id, accounts, &instruction_data[1..]),                  // Stake GROW
        24 => unstake_grow(program_id, accounts, &instruction_data[1..]),                // Unstake GROW
        25 => daily_check_in_reward(program_id, accounts, &instruction_data[1..]),       // Daily Check In Reward
        26 => update_priority_group(program_id, accounts, &instruction_data[1..]),       // Update priority group
        27 => add_insurance_fund(program_id, accounts, &instruction_data[1..]),          // add insurance fund
        28 => add_premium_protection(program_id, accounts, &instruction_data[1..]),    // add premium protection
        100 => initialize_marketplace(program_id, accounts, &instruction_data[1..]), // Initialize marketplace
        101 => update_marketplace(program_id, accounts, &instruction_data[1..]),     // Update marketplace
        102 => pause_marketplace(program_id, accounts),                               // Pause marketplace
        103 => unpause_marketplace(program_id, accounts),                             // Unpause marketplace
        104 => deactivate_listing(program_id, accounts, &instruction_data[1..]),    // Deactivate listing
        105 => rate_buyer(program_id, accounts, &instruction_data[1..]),          // Rate buyer
            msg!("Invalid Instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}

// Add Insurance Fund
pub fn add_insurance_fund(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Adding to insurance fund");
    let accounts_iter = &mut accounts.iter();
    let user_token_account = next_account_info(accounts_iter)?;
    let insurance_fund_token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check owner is signer
    if !owner_account.is_signer {
        msg!("Owner account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check account owner
    if user_token_account.owner != owner_account.owner {
        msg!("Token account owner does not match owner account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Get the amount to transfer from the instruction data
    let amount = {
        let mut data = instruction_data.iter();
        let mut amount_bytes = [0u8; 8];
        for i in 0..8 {
            amount_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
        }
        u64::from_le_bytes(amount_bytes)
    };
    // Create transfer instruction
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        user_token_account.key,
        insurance_fund_token_account.key,
        owner_account.key,
        &[&owner_account.key],
        amount,
    )?;
    // Invoke transfer instruction
    invoke_signed(
        &transfer_instruction,
        &[
            user_token_account.clone(),
            insurance_fund_token_account.clone(),
            owner_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    msg!("Transfered {} tokens to insurance fund from {}", amount, user_token_account.key);
    Ok(())
}

// Add Premium Protection
pub fn add_premium_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Adding to premium protection");
    let accounts_iter = &mut accounts.iter();
    let user_token_account = next_account_info(accounts_iter)?;
    let premium_protection_token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Get the amount to transfer from the instruction data
    let amount = {
        let mut data = instruction_data.iter();
        let mut amount_bytes = [0u8; 8];
        for i in 0..8 {
            amount_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
        }
        u64::from_le_bytes(amount_bytes)
    };
    transfer_grow(_program_id, accounts, instruction_data)
}

// Daily Check-In Reward
pub fn daily_check_in_reward(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Daily Check-In Reward");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let daily_check_in_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if daily_check_in_account.owner != program_id {
        msg!("Daily Check-In account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if user signed
    if !user_account.is_signer {
        msg!("User signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check daily_check_in_account is writable
    if !daily_check_in_account.is_writable {
        msg!("Daily Check-In account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Get the current time
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;
    let current_day = current_timestamp / 86400; // Get the current day

    // Deserialize the daily check-in data
    let mut check_in_data = match DailyCheckIn::try_from_slice(&daily_check_in_account.data.borrow()) {
        Ok(data) => data,
        Err(_) => {
            // If the data cannot be deserialized, initialize a new entry
            DailyCheckIn {
                user: *user_account.key,
                last_check_in: 0,
                last_check_in_day: 0,
                streak: 0,
            }
        },
    };

    // Check if the user has already checked in today
    if check_in_data.last_check_in_day == current_day {
        msg!("User has already checked in today");
        return Err(ProgramError::Custom(ErrorCode::AlreadyRated as u32));
    }

    // Update check-in data
    check_in_data.last_check_in = current_timestamp;
    check_in_data.last_check_in_day = current_day;
    check_in_data.serialize(&mut &mut daily_check_in_account.data.borrow_mut()[..])?;
    Ok(())
}

// Update the priority group
pub fn update_priority_group(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Updating priority group");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let marketplace_state_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if marketplace_state_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if admin signed
    if !admin_account.is_signer {
        msg!("Admin signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Deserialize the marketplace state
    let mut marketplace = MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;
    // Validate caller is authorized
    if marketplace.authority != *admin_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }
    // Get the new priority group
    let new_priority_group = PriorityGroup::try_from_slice(instruction_data)?;
    // Update priority group
    marketplace.priorityGroup = new_priority_group;
    // Serialize the updated marketplace data
    marketplace.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Update check-in data
    check_in_data.last_check_in = current_timestamp;
    check_in_data.last_check_in_day = current_day;
    check_in_data.serialize(&mut &mut daily_check_in_account.data.borrow_mut()[..])?;
    Ok(())
}


// Mint GROW tokens
pub fn mint_grow(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Minting GROW tokens");
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check mint authority is signer
    if !mint_authority_account.is_signer {
        msg!("Mint authority account is not a signer");
        return Err(ProgramError::Custom(ErrorCode::InvalidMintAuthority as u32));
    }
    // Check account owner
    if token_account.owner != mint_authority_account.owner {
        msg!("Token account owner does not match mint authority account owner");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Get the amount to mint from the instruction data
    let amount = {
        let mut data = instruction_data.iter();
        let mut amount_bytes = [0u8; 8];
        for i in 0..8 {
            amount_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
        }
        u64::from_le_bytes(amount_bytes)
    };
    // Create mint instruction
    let mint_instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint_account.key,
        token_account.key,
        mint_authority_account.key,
        &[&mint_authority_account.key],
        amount,
    )?;
    // Invoke mint instruction
    invoke_signed(
        &mint_instruction,
        &[
            mint_account.clone(),
            token_account.clone(),
            mint_authority_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    msg!("Minted {} GROW tokens to {}", amount, token_account.key);
    Ok(())
}

// Mint FARM tokens
pub fn mint_farm(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Minting FARM tokens");
    mint_grow(_program_id, accounts, instruction_data)
}

// Transfer GROW tokens
pub fn transfer_grow(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transfering GROW tokens");
    let accounts_iter = &mut accounts.iter();
    let source_token_account = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check owner is signer
    if !owner_account.is_signer {
        msg!("Owner account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check account owner
    if source_token_account.owner != owner_account.owner {
        msg!("Token account owner does not match owner account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Get the amount to transfer from the instruction data
    let amount = {
        let mut data = instruction_data.iter();
        let mut amount_bytes = [0u8; 8];
        for i in 0..8 {
            amount_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
        }
        u64::from_le_bytes(amount_bytes)
    };
    // Create transfer instruction
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        source_token_account.key,
        destination_token_account.key,
        owner_account.key,
        &[&owner_account.key],
        amount,
    )?;
    // Invoke transfer instruction
    invoke_signed(
        &transfer_instruction,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            owner_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    msg!("Transfered {} GROW tokens from {} to {}", amount, source_token_account.key, destination_token_account.key);
    Ok(())
}

// Transfer FARM tokens
pub fn transfer_farm(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transfering FARM tokens");
    transfer_grow(_program_id, accounts, instruction_data)
}

// Burn GROW tokens
pub fn burn_grow(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Burning GROW tokens");
    let accounts_iter = &mut accounts.iter();
    let _token_account = next_account_info(accounts_iter)?;
    let _mint_account = next_account_info(accounts_iter)?;
    let _owner_account = next_account_info(accounts_iter)?;
    Ok(())
}

// Burn FARM tokens
pub fn burn_farm(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Burning FARM tokens");
    let accounts_iter = &mut accounts.iter();
    let _token_account = next_account_info(accounts_iter)?;
    let _mint_account = next_account_info(accounts_iter)?;
    let _owner_account = next_account_info(accounts_iter)?;
    Ok(())
}

// Stake GROW tokens
pub fn stake_grow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Staking GROW tokens");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let staking_account = next_account_info(accounts_iter)?;
    let staker_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if staking_account.owner != program_id {
        msg!("Staking account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if staker signed
    if !staker_account.is_signer {
        msg!("Staker signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check account owner
    if token_account.owner != staker_account.owner {
        msg!("Token account owner does not match staker account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the amount to stake from the instruction data
    let (amount, duration) = {
        let mut data = instruction_data.iter();
        let amount = {
            let mut amount_bytes = [0u8; 8];
            for i in 0..8 {
                amount_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(amount_bytes)
        };
        let duration = {
            let mut duration_bytes = [0u8; 8];
            for i in 0..8 {
                duration_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(duration_bytes)
        };
        (amount, duration)
    };

    // Check duration is more than 1 month
    if duration < 2592000 { // 30 days in seconds
        msg!("Invalid duration, it must be more than 1 month");
        return Err(ProgramError::Custom(ErrorCode::InvalidStakingDuration as u32));
    }

    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp as u64;

    // Check if the user has enough tokens
    let token_account_data = try_from_slice_unchecked::<spl_token::state::Account>(&token_account.try_borrow_data()?)?;
    if token_account_data.amount < amount {
        msg!("Insufficient GROW tokens to stake");
        return Err(ProgramError::Custom(ErrorCode::InsufficientFunds as u32));
    }

    // Create a new staking entry
    let staking = Staking {
        staker: *staker_account.key,
        amount,
        start_time: timestamp,
        end_time: timestamp + duration,
        reward_multiplier: 1.0,
    };

    // Serialize the staking data
    staking.serialize(&mut &mut staking_account.data.borrow_mut()[..])?;
    msg!("Staked {} GROW tokens", amount);
    Ok(())
}

// Unstake GROW tokens
pub fn unstake_grow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Unstaking GROW tokens");
    // Get account iterator
    let accounts_iter = &mut accounts.iter();
    // Get accounts
    let staking_account = next_account_info(accounts_iter)?;
    let staker_account = next_account_info(accounts_iter)?;
    // Check account ownership
    if staking_account.owner != program_id {
        msg!("Staking account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check if staker signed
    if !staker_account.is_signer {
        msg!("Staker signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Deserialize the staking data
    let staking = Staking::try_from_slice(&staking_account.data.borrow())?;
    // Check if user has staked tokens
    if staking.staker != *staker_account.key {
        msg!("User has not staked tokens");
        return Err(ProgramError::Custom(ErrorCode::StakingNotFound as u32));
    }
    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp as u64;

    if staking.end_time > timestamp {
        msg!("Time not finished, applying penalty");
        //Apply penalty
    }
    Ok(())
}
// Mint Digital Farm NFT
pub fn mint_digital_farm_nft(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Minting Digital Farm NFT");
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check mint authority is signer
    if !mint_authority_account.is_signer {
        msg!("Mint authority account is not a signer");
        return Err(ProgramError::Custom(ErrorCode::InvalidMintAuthority as u32));
    }
    // Check account owner
    if token_account.owner != mint_authority_account.owner {
        msg!("Token account owner does not match mint authority account owner");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Create mint instruction
    let mint_instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint_account.key,
        token_account.key,
        mint_authority_account.key,
        &[&mint_authority_account.key],
        1,
    )?;
    // Invoke mint instruction
    invoke_signed(
        &mint_instruction,
        &[
            mint_account.clone(),
            token_account.clone(),
            mint_authority_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    msg!("Minted 1 Digital Farm NFT to {}", token_account.key);
    Ok(())
}

// Mint Growing Slot NFT
pub fn mint_growing_slot_nft(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Minting Growing Slot NFT");
    mint_digital_farm_nft(_program_id, accounts, instruction_data)
}

// Mint Harvest Product NFT
pub fn mint_harvest_product_nft(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Minting Harvest Product NFT");
    mint_digital_farm_nft(_program_id, accounts, instruction_data)
}

// Transfer NFT
pub fn transfer_nft(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transfering NFT");
    let accounts_iter = &mut accounts.iter();
    let source_token_account = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check owner is signer
    if !owner_account.is_signer {
        msg!("Owner account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check account owner
    if source_token_account.owner != owner_account.owner {
        msg!("Token account owner does not match owner account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }
    // Create transfer instruction
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        source_token_account.key,
        destination_token_account.key,
        owner_account.key,
        &[&owner_account.key],
        1,
    )?;
    // Invoke transfer instruction
    invoke_signed(
        &transfer_instruction,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            owner_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    msg!("Transfered 1 NFT from {} to {}", source_token_account.key, destination_token_account.key);
    Ok(())
}

// Burn NFT
pub fn burn_nft(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Burning NFT");
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
        // Burn instruction
    let burn_instruction = spl_token::instruction::burn(
        &spl_token::id(),
        token_account.key,
        mint_account.key,
        owner_account.key,
        &[&owner_account.key],
        1,
    )?;
    // Invoke burn instruction
    invoke_signed(
        &burn_instruction,
        &[token_account.clone(), mint_account.clone(), owner_account.clone(), token_program_account.clone()],
        &[] // No seeds for signer
    )?;
    msg!("NFT burned successfully");
    Ok(())
}
// Redeem Harvest Product NFT
pub fn redeem_harvest_product_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Redeeming Harvest Product NFT");
    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check account ownership
    if token_account.owner != owner_account.owner {
        msg!("Token account owner does not match owner account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }

    // Check if owner signed
    if !owner_account.is_signer {
        msg!("Owner signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // burn NFT
    burn_nft(program_id, accounts, instruction_data)
}
// Compost Harvest Product NFT
pub fn compost_harvest_product_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Composting Harvest Product NFT");
    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    // Check token program
    if token_program_account.key.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check account ownership
    if token_account.owner != owner_account.owner {
        msg!("Token account owner does not match owner account");
        return Err(ProgramError::Custom(ErrorCode::InvalidTokenAccountOwner as u32));
    }

    // Check if owner signed
    if !owner_account.is_signer {
        msg!("Owner signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }
    // burn NFT
    burn_nft(program_id, accounts, instruction_data)

    Ok(())
}

// Submit a proposal
pub fn submit_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Submitting Proposal");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let proposal_account = next_account_info(accounts_iter)?;
    let proposer_account = next_account_info(accounts_iter)?;
    let proposer_token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if proposal_account.owner != program_id {
        msg!("Proposal account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if proposer signed
    if !proposer_account.is_signer {
        msg!("Proposer signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check proposal_account is writable
    if !proposal_account.is_writable {
        msg!("Proposal account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp as u64;

    // Deserialize the proposal data from the instruction data
    let (id, description, tier) = {
        let mut data = instruction_data.iter();
        let id = {
            let mut id_bytes = [0u8; 4];
            for i in 0..4 {
                id_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u32::from_le_bytes(id_bytes)
        };
        
        let mut description_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            if *byte == 0 {
                break;
            }
            description_bytes.push(*byte);
        }

        let mut tier_bytes = [0u8; 1];
        tier_bytes[0] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
        let description = String::from_utf8(description_bytes).unwrap_or_default();
        let tier = match tier_bytes[0] {
            0 => ProposalTier::Level1,
            1 => ProposalTier::Level2,
            2 => ProposalTier::Level3,
            _ => {
                msg!("Invalid proposal tier");
                return Err(ProgramError::Custom(ErrorCode::InvalidProposalTier as u32));
            }
        };
        (id, description, tier)
    };

    // Validate that the user has enough FARM tokens
    let required_farm_tokens = match tier {
        ProposalTier::Level1 => 10,
        ProposalTier::Level2 => 100,
        ProposalTier::Level3 => 1000,
    };

    let proposer_token_account_data = try_from_slice_unchecked::<spl_token::state::Account>(&proposer_token_account.try_borrow_data()?)?;
    if proposer_token_account_data.amount < required_farm_tokens {
        msg!("Insufficient FARM tokens to submit this proposal");
        return Err(ProgramError::Custom(ErrorCode::NotEnoughTokens as u32));
    }

    // Create a new proposal
    let proposal = GovernanceProposal {
        id,
        proposer: *proposer_account.key,
        description,
        tier,
        yes_votes: 0,
        no_votes: 0,
        voting_start_time: timestamp,
        voting_end_time: timestamp + 7*86400, // 7 days duration
        executed: false,
        has_voted: Vec::new(),
    };

    // Serialize the proposal data
    proposal.serialize(&mut &mut proposal_account.data.borrow_mut()[..])?;

    msg!("Proposal submitted successfully");
    Ok(())
}

// Vote on a proposal
pub fn vote_on_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Voting on Proposal");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let proposal_account = next_account_info(accounts_iter)?;
    let voter_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if proposal_account.owner != program_id {
        msg!("Proposal account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if voter signed
    if !voter_account.is_signer {
        msg!("Voter signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check proposal_account is writable
    if !proposal_account.is_writable {
        msg!("Proposal account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the proposal data
    let mut proposal = GovernanceProposal::try_from_slice(&proposal_account.data.borrow())?;

    // Verify the proposal exists
    if proposal.proposer == Pubkey::default() {
        msg!("Proposal does not exist");
        return Err(ProgramError::Custom(ErrorCode::ProposalNotFound as u32));
    }
    // Verify user has not already voted
    if proposal.has_voted.contains(voter_account.key) {
        msg!("User has already voted");
        return Err(ProgramError::Custom(ErrorCode::InvalidVote as u32));
    }

    // Verify that the proposal is active
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;
    if current_timestamp < proposal.voting_start_time || current_timestamp > proposal.voting_end_time {
        msg!("Proposal voting period is not active");
        return Err(ProgramError::Custom(ErrorCode::InvalidTimestamp as u32));
    }

    // Get the vote type from the instruction data (0 for no, 1 for yes)
    let vote_type = instruction_data[0];

    // Update the vote count
    match vote_type {
        0 => proposal.no_votes = proposal.no_votes.checked_add(1).ok_or(ProgramError::Custom(ErrorCode::MathOverflow as u32))?,
        1 => proposal.yes_votes = proposal.yes_votes.checked_add(1).ok_or(ProgramError::Custom(ErrorCode::MathOverflow as u32))?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    // add voter to has_voted
    proposal.has_voted.push(*voter_account.key);
    // Serialize the updated proposal data
    proposal.serialize(&mut &mut proposal_account.data.borrow_mut()[..])?;

    msg!("Vote recorded successfully");
    Ok(())
}

// Initialize the marketplace state
pub fn init_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Initializing Marketplace");

    // Check accounts access
    let (marketplace_state_account, admin_account) =
        check_marketplace_account_access(program_id, accounts, true)?;
    if !marketplace_state_account.is_writable {
            return Err(ProgramError::InvalidAccountData);
    }  

    // Get the current time    
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    // Create and initialize the marketplace state
    let mut marketplace_state = MarketplaceState {
        authority: *admin_account.unwrap().key,
        fee: 0,
        dispute_period: 86400,
        min_rating_period: 86400,
        name: String::from("FreeBonde Marketplace"),
        created_at: timestamp,
        updated_at: timestamp,
        paused: false,
        total_listings: 0,
        total_transactions: 0,
        priorityGroup: PriorityGroup::Low,
        maxListing: 100,
        currentListing: 0,
        crisisState: CrisisState::Inactive,
        crisisTreasury: 0,
    };

    // Serialize the marketplace state
    marketplace_state.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Marketplace Initialized event
    msg!("Event: MarketplaceInitialized");
    msg!("marketplace_id: {}", marketplace_state_account.key);
    msg!("authority: {}", marketplace_state.authority);
    msg!("fee: {}", marketplace_state.fee);
    msg!("dispute_period: {}", marketplace_state.dispute_period);
    msg!("rating_period: {}", marketplace_state.min_rating_period);
    msg!("created_at: {}", marketplace_state.created_at);

    msg!("Marketplace initialized successfully");
    Ok(())
}

// Update the marketplace configuration
pub fn update_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Updating Marketplace");

    // Check accounts access
    let (marketplace_state_account, admin_account) =
        check_marketplace_account_access(program_id, accounts, true)?;

        // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Deserialize the marketplace state
    let mut marketplace = MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;
    // Validate caller is authorized
    if marketplace.authority != *admin_account.unwrap().key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }

    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    // Deserialize the new values from the instruction data
    let (new_fee, new_dispute_period, new_rating_period, new_name) = {
        let mut data = instruction_data.iter();

        let new_fee = {
            let mut fee_bytes = [0u8; 2];
            for i in 0..2 {
                fee_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u16::from_le_bytes(fee_bytes)
        };

        let new_dispute_period = {
            let mut period_bytes = [0u8; 8];
            for i in 0..8 {
                period_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            i64::from_le_bytes(period_bytes)
        };

        let new_rating_period = {
            let mut period_bytes = [0u8; 8];
            for i in 0..8 {
                period_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            i64::from_le_bytes(period_bytes)
        };

        let mut name_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            name_bytes.push(*byte);
        }
        let new_name = String::from_utf8(name_bytes).unwrap_or_default();

        (new_fee, new_dispute_period, new_rating_period, new_name)
    };
    // Update fields if provided
    if new_fee <= 1000 {
        marketplace.fee = new_fee;
    } else {
        msg!("Invalid marketplace fee");
        return Err(ProgramError::Custom(ErrorCode::InvalidMarketplaceFee as u32));
    }

    if new_dispute_period >= 86400 {
        marketplace.dispute_period = new_dispute_period;
    } else {
        msg!("Invalid dispute period");
        return Err(ProgramError::Custom(ErrorCode::InvalidDisputePeriod as u32));
    }

    if new_rating_period >= 86400 {
        marketplace.min_rating_period = new_rating_period;
    } else {
        msg!("Invalid rating period");
        return Err(ProgramError::Custom(ErrorCode::InvalidRatingPeriod as u32));
    }
    marketplace.name = new_name;
    // Update updated_at
    marketplace.updated_at = timestamp;

    // Serialize the updated marketplace data
    marketplace.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Marketplace Updated event
    msg!("Event: MarketplaceUpdated");
    msg!("marketplace_id: {}", marketplace_state_account.key);
    msg!("updated_at: {}", marketplace.updated_at);

    msg!("Marketplace updated successfully");
    Ok(())
}

// Pause the marketplace
pub fn pause_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {    
    msg!("Pausing Marketplace");

    // Check accounts access
    let (marketplace_state_account, admin_account) =
        check_marketplace_account_access(program_id, accounts, true)?;

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the marketplace state
    let mut marketplace = MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;

    // Validate caller is authorized    
    if marketplace.authority != *admin_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }

    // Pause the marketplace
    marketplace.paused = true;

    // Update updated_at
    let clock = Clock::get()?;
    marketplace.updated_at = clock.unix_timestamp;

    // Serialize the updated marketplace data
    marketplace.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Marketplace Paused event
    msg!("Event: MarketplacePaused");
    msg!("marketplace_id: {}", marketplace_state_account.key);
    msg!("updated_at: {}", marketplace.updated_at);

    msg!("Marketplace paused successfully");
    Ok(())
}

// Unpause the marketplace
pub fn unpause_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {    
    msg!("Unpausing Marketplace");

    // Check accounts access
    let (marketplace_state_account, admin_account) =
        check_marketplace_account_access(program_id, accounts, true)?;

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the marketplace state
    let mut marketplace = MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;

    // Validate caller is authorized
    if marketplace.authority != *admin_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }

    // Unpause the marketplace
    marketplace.paused = false;

    // Update updated_at
    let clock = Clock::get()?;
    marketplace.updated_at = clock.unix_timestamp;

    // Serialize the updated marketplace data
    marketplace.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Marketplace Unpaused event
    msg!("Event: MarketplaceUnpaused");
    msg!("marketplace_id: {}", marketplace_state_account.key);
    msg!("updated_at: {}", marketplace.updated_at);

    msg!("Marketplace unpaused successfully");
    Ok(())
}

// Create a new listing in the marketplace
pub fn create_listing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Creating Listing");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let listing_account = next_account_info(accounts_iter)?;
    let marketplace_state_account = next_account_info(accounts_iter)?;
    let seller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let harvest_nft_account = next_account_info(accounts_iter).ok();

    // Check account ownership
    if marketplace_state_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if seller signed
    if !seller_account.is_signer {
        msg!("Seller signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Check listing_account is writable
    if !listing_account.is_writable {
        msg!("Listing account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the marketplace state
    let mut marketplace_state =
        MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;

    // Check if the maximum number of listings has been reached
    if marketplace_state.currentListing >= marketplace_state.maxListing {
        msg!("Maximum number of listings reached");
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp as i64;

    // Deserialize the listing data from the instruction data
    let (id, price, quantity, description, is_organic, nutritional_data) = {
        let mut data = instruction_data.iter();
        let id = {
            let mut id_bytes = [0u8; 4];
            for i in 0..4 {
                id_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u32::from_le_bytes(id_bytes)
        };
        let price = {
            let mut price_bytes = [0u8; 8];
            for i in 0..8 {
                price_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(price_bytes)
        };
        let quantity = {
            let mut quantity_bytes = [0u8; 8];
            for i in 0..8 {
                quantity_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(quantity_bytes)
        };
        let mut description_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            if *byte == 0 {
                break;
            }
            description_bytes.push(*byte);
        }

        let mut is_organic_bytes = [0u8; 1];
        is_organic_bytes[0] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;

        let mut nutritional_data_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            nutritional_data_bytes.push(*byte);
        }
        let description = String::from_utf8(description_bytes).unwrap_or_default();
        let is_organic = is_organic_bytes[0] != 0;
        let nutritional_data = String::from_utf8(nutritional_data_bytes).unwrap_or_default();
        (id, price, quantity, description, is_organic, nutritional_data)
    };

    // Validate parameters
    if price <= 0 {
        msg!("Invalid price");
        return Err(ProgramError::Custom(ErrorCode::InvalidPrice as u32));
    }
    if quantity <= 0 {
        msg!("Invalid quantity");
        return Err(ProgramError::Custom(ErrorCode::InvalidQuantity as u32));
    }
    if description.is_empty() {
        msg!("Invalid description");
        return Err(ProgramError::Custom(ErrorCode::InvalidDescription as u32));
    }

    // Create a new listing
    let listing = Listing {
        id,
        seller: *seller_account.key,
        marketplace: *marketplace_state_account.key,
        mint: *mint_account.key,
        price,
        quantity,
        quantity_remaining: quantity,
        description,
        is_organic,
        nutritional_data,
        harvest_nft: harvest_nft_account.map(|account| *account.key),
        created_at: timestamp,
        updated_at: timestamp,
        active: true,
        timestamp: clock.unix_timestamp as u64,
    };

    // Serialize the listing data
    listing.serialize(&mut &mut listing_account.data.borrow_mut()[..])?;

    // Increment the current number of listings
    marketplace_state.currentListing += 1;
    marketplace_state.total_listings += 1;
    // Serialize the updated marketplace state
    marketplace_state.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Listing Created event    
    msg!("Event: ListingCreated");
    msg!("listing_id: {}", listing_account.key);
    msg!("seller: {}", listing.seller);
    msg!("mint: {}", listing.mint);
    msg!("price: {}", listing.price);
    msg!("quantity: {}", listing.quantity);
    msg!("created_at: {}", listing.created_at);
    if let Some(nft_key) = listing.harvest_nft {
        msg!("harvest_nft: {}", nft_key);
    }

    msg!("Listing created successfully");
    Ok(())
}

// Update an existing listing in the marketplace
pub fn update_listing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Updating Listing");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let listing_account = next_account_info(accounts_iter)?;    
    )?;

    // Check account ownership
    if listing_account.owner != program_id {
        msg!("Listing account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if seller signed
    if !seller_account.is_signer {
        msg!("Seller signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check listing_account is writable
    if !listing_account.is_writable {
        msg!("Listing account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the listing data
    let mut listing = Listing::try_from_slice(&listing_account.data.borrow())?;

    // Validate caller is authorized
    if listing.seller != *seller_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }

    // Deserialize the new values from the instruction data
    let (new_price, new_quantity, new_description, new_is_organic, new_nutritional_data) = {
        let mut data = instruction_data.iter();
        let new_price = {
            let mut price_bytes = [0u8; 8];
            for i in 0..price_bytes.len() {
                price_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(price_bytes)
        };

        let new_quantity = {
            let mut quantity_bytes = [0u8; 8];
            for i in 0..quantity_bytes.len() {
                quantity_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(quantity_bytes)
        };
        let mut description_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            if *byte == 0 {
                break;
            }
            description_bytes.push(*byte);
        }

        let mut is_organic_bytes = [0u8; 1];
        is_organic_bytes[0] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;

        let mut nutritional_data_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            nutritional_data_bytes.push(*byte);
        }
        let new_description = String::from_utf8(description_bytes).unwrap_or_default();
        let new_is_organic = is_organic_bytes[0] != 0;
        let new_nutritional_data = String::from_utf8(nutritional_data_bytes).unwrap_or_default();

        (new_price, new_quantity, new_description, new_is_organic, new_nutritional_data)
    };

    // Update fields if provided
    if new_price > 0 {
        listing.price = new_price;
    }else {
        msg!("Invalid price");
        return Err(ProgramError::Custom(ErrorCode::InvalidPrice as u32));
    }
    if new_quantity > 0 {
        listing.quantity = new_quantity;
    }else{
        msg!("Invalid quantity");
        return Err(ProgramError::Custom(ErrorCode::InvalidQuantity as u32));
    }
    if !new_description.is_empty() {
        listing.description = new_description;
    }else{
        msg!("Invalid description");
        return Err(ProgramError::Custom(ErrorCode::InvalidDescription as u32));
    }
    listing.is_organic = new_is_organic;
    listing.nutritional_data = new_nutritional_data;

    // Update updated_at
    let clock = Clock::get()?;
    listing.updated_at = clock.unix_timestamp;

    // Serialize the updated listing data
    listing.serialize(&mut &mut listing_account.data.borrow_mut()[..])?;
    // Emit Listing Updated event
    msg!("Event: ListingUpdated");
    msg!("listing_id: {}", listing_account.key);
    msg!("updated_at: {}", listing.updated_at);
    msg!("Listing updated successfully");
    Ok(())
}


// Deactivate an existing listing in the marketplace
pub fn delete_listing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Deactivating Listing");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let listing_account = next_account_info(accounts_iter)?;
    let seller_account = next_account_info(accounts_iter)?;
    let marketplace_state_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if marketplace_state_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if seller signed
    if !seller_account.is_signer {
        msg!("Seller signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check listing_account is writable
    if !listing_account.is_writable {
        msg!("Listing account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the listing data
    let mut listing = Listing::try_from_slice(&listing_account.data.borrow())?;
    // Deserialize the marketplace state
    let mut marketplace_state =
        MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;

    // Validate caller is authorized
    if listing.seller != *seller_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }

    // Deactivate the listing
    listing.active = false;

    // Update updated_at
    let clock = Clock::get()?;
    listing.updated_at = clock.unix_timestamp;

    // Serialize the updated listing data
    listing.serialize(&mut &mut listing_account.data.borrow_mut()[..])?;
    marketplace_state.currentListing -= 1;
    // Serialize the updated marketplace state
    marketplace_state.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;

    // Emit Listing Deactivated event
    msg!("Event: ListingDeactivated");
    msg!("listing_id: {}", listing_account.key);
    msg!("updated_at: {}", listing.updated_at);

    msg!("Listing deactivated successfully");
    Ok(())
}

// Buy a listing in the marketplace
pub fn buy_listing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Buying Listing");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let listing_account = next_account_info(accounts_iter)?;
    let marketplace_state_account = next_account_info(accounts_iter)?;
    let buyer_account = next_account_info(accounts_iter)?;
    let buyer_token_account = next_account_info(accounts_iter)?;
    let seller_token_account = next_account_info(accounts_iter)?;
    let dao_treasury_account = next_account_info(accounts_iter)?;
    let transaction_account = next_account_info(accounts_iter)?;
    let order_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    // Check account ownership
    if marketplace_state_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    if token_program_account.owner.to_bytes() != [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] { //SPL_TOKEN
        msg!("Token program account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // Check if buyer signed
    if !buyer_account.is_signer {
        msg!("Buyer signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check marketplace_state_account is writable
    if !marketplace_state_account.is_writable {
        msg!("Marketplace state account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check listing_account is writable
    if !listing_account.is_writable {
        msg!("Listing account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check buyer_token_account is writable
    if !buyer_token_account.is_writable {
        msg!("Buyer token account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check seller_token_account is writable
    if !seller_token_account.is_writable {
        msg!("Seller token account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check dao_treasury_account is writable
    if !dao_treasury_account.is_writable {
        msg!("DAO treasury account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check transaction_account is writable
    if !transaction_account.is_writable {
        msg!("Transaction account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Deserialize the listing data
    if listing_account.owner != program_id {
        msg!("Listing account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut listing = Listing::try_from_slice(&listing_account.data.borrow())?;
    // Deserialize the marketplace state
    let mut marketplace_state =
        MarketplaceState::try_from_slice(&marketplace_state_account.data.borrow())?;
    // Get the current time
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    //Get quantity from instruction data
    let quantity = {
        let mut data = instruction_data.iter();        
        let mut quantity_bytes = [0u8; 8];
            for i in 0..quantity_bytes.len() {
                quantity_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            u64::from_le_bytes(quantity_bytes)
    };
    //Check if token accounts are the same mint
    let buyer_token_account_data = try_from_slice_unchecked::<spl_token::state::Account>(&buyer_token_account.try_borrow_data()?)?;
    let seller_token_account_data = try_from_slice_unchecked::<spl_token::state::Account>(&seller_token_account.try_borrow_data()?)?;

    if buyer_token_account_data.mint != seller_token_account_data.mint {
        msg!("Buyer and Seller token accounts are not the same mint");
        return Err(ProgramError::Custom(ErrorCode::TokenMintMismatch as u32));
    }
    if buyer_token_account_data.mint != listing.mint {
        msg!("Buyer token account is not the same mint than the listing");
        return Err(ProgramError::Custom(ErrorCode::TokenMintMismatch as u32));
    }
    // Validate parameters
    if listing.price <= 0 {
        msg!("Invalid price");
        return Err(ProgramError::Custom(ErrorCode::InvalidPrice as u32));
    }
    if listing.quantity <= 0 {
        msg!("Invalid quantity");
        return Err(ProgramError::Custom(ErrorCode::InvalidQuantity as u32));
    if quantity <= 0 {
        msg!("Invalid quantity");
        return Err(ProgramError::Custom(ErrorCode::InvalidQuantity as u32));
    }

    // Verify marketplace is not paused
    if marketplace_state.paused {
        msg!("Marketplace is paused");
        return Err(ProgramError::Custom(ErrorCode::MarketplacePaused as u32));
    }
    // Verify listing is active and has sufficient quantity
    if !listing.active {
        msg!("Listing is not active");
        return Err(ProgramError::Custom(ErrorCode::ListingNotActive as u32));
    }
    if listing.quantity_remaining < quantity {
        msg!("Insufficient quantity");
        return Err(ProgramError::Custom(ErrorCode::InsufficientQuantity as u32));
    }
    // Calculate the total price and fees
    let total_price = listing.price
        .checked_mul(quantity)
        .ok_or(ProgramError::Custom(ErrorCode::MathOverflow as u32))?;
    let marketplace_fee = total_price
        .checked_mul(marketplace_state.fee as u64)
        .and_then(|val| val.checked_div(1000)) // Assuming fee is per mille
        .ok_or(ProgramError::Custom(ErrorCode::MathOverflow as u32))?;
    let net_price = total_price
        .checked_sub(marketplace_fee)
        .ok_or(ProgramError::Custom(ErrorCode::MathOverflow as u32))?;

    // Transfer tokens from buyer to seller (net price)
    let transfer_to_seller_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        buyer_token_account.key,
        seller_token_account.key,
        buyer_account.key,
        &[&buyer_account.key],
        net_price
    )?;
    invoke_signed(
        &transfer_to_seller_instruction,
        &[
            buyer_token_account.clone(),
            seller_token_account.clone(),
            buyer_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;

    // Transfer marketplace fee to DAO treasury
    let transfer_to_dao_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        buyer_token_account.key,
        dao_treasury_account.key,
        buyer_account.key,
        &[&buyer_account.key],
        marketplace_fee
    )?;
    invoke_signed(
        &transfer_to_dao_instruction,
        &[
            buyer_token_account.clone(),
            dao_treasury_account.clone(),
            buyer_account.clone(),
            token_program_account.clone(),
        ],
        &[], // No seeds for the signer in this case
    )?;
    // Update listing quantity remaining
    listing.quantity_remaining -= quantity;

    // Update transaction details
    let transaction = Transaction{
        buyer: *buyer_account.key,
        seller: listing.seller,
        mint: listing.mint,
        total_price,
        marketplace_fee,
        net_price,
        quantity,
        created_at: timestamp,
        updated_at: timestamp,
    };
    // Update order details
    let order = Order{
        seller: listing.seller,
        buyer: *buyer_account.key,
        status: OrderStatus::Pending as u8,
        completed_at: None,
        seller_rated: false,
        updated_at: timestamp,
        transaction: *transaction_account.key, };
    transaction.serialize(&mut &mut transaction_account.data.borrow_mut()[..])?;
    //Create order account
    order.serialize(&mut &mut order_account.data.borrow_mut()[..])?;
    
    // Update marketplace stats
    marketplace_state.total_transactions += 1;
    // Serialize the updated listing data
    listing.serialize(&mut &mut listing_account.data.borrow_mut()[..])?;
    marketplace_state.serialize(&mut &mut marketplace_state_account.data.borrow_mut()[..])?;
    // Emit Purchase event
    msg!("Event: PurchaseEvent");
    msg!("transaction_id: {}", transaction_account.key);
    msg!("buyer: {}", transaction.buyer);
    msg!("seller: {}", transaction.seller);
    msg!("mint: {}", transaction.mint);
    msg!("total_price: {}", transaction.total_price);
    msg!("quantity: {}", transaction.quantity);
    msg!("created_at: {}", transaction.created_at);

    msg!("Listing buyed successfully");
    Ok(())
}

// Rate a buyer
pub fn rate_buyer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Rating Buyer");
    
    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let order_account = next_account_info(accounts_iter)?;    
    let marketplace_account = next_account_info(accounts_iter)?;
    let rater_account = next_account_info(accounts_iter)?;    
    let buyer_rating_account = next_account_info(accounts_iter)?;
    let _dispute_account = next_account_info(accounts_iter).ok();

    // Check account ownership
    if marketplace_account.owner != program_id {
        msg!("Marketplace state account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if rater signed
    if !rater_account.is_signer {
        msg!("Rater signature is missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check marketplace_account is writable   
    if !marketplace_account.is_writable {
        msg!("Marketplace account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }
    // Check order_account is writable
    if !order_account.is_writable {
        msg!("Order account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }   

    // Check buyer_rating_account is writable
    if !buyer_rating_account.is_writable {
        msg!("Buyer rating account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the order data    
    let mut order = Order::try_from_slice(&order_account.data.borrow())?;
    // Deserialize the marketplace data
    let marketplace =
    MarketplaceState::try_from_slice(&marketplace_account.data.borrow())?;

    
    // Get the rating and comment from the instruction data
    let (rating, comment) = {
        let mut data = instruction_data.iter();
        let rating = {
            let mut rating_bytes = [0u8; 1];
            for i in 0..1 {
                rating_bytes[i] = *data.next().ok_or(ProgramError::InvalidInstructionData)?;
            }
            rating_bytes[0]
        };

        let mut comment_bytes: Vec<u8> = Vec::new();
        while let Some(byte) = data.next() {
            comment_bytes.push(*byte);
        }
        let comment = String::from_utf8(comment_bytes).unwrap_or_default();

        (rating, comment)
    };
    // Validate parameters
    if rating < 1 || rating > 5 {
        msg!("Invalid rating");
        return Err(ProgramError::Custom(ErrorCode::InvalidRatingValue as u32));
    }
    // Verify order status
    if order.status != OrderStatus::Completed as u8 && order.status != OrderStatus::Disputed as u8 {
        msg!("Invalid order status");
        return Err(ProgramError::Custom(ErrorCode::InvalidOrderStatus as u32));
    }
    // Verify rater is the seller
    if order.seller != *rater_account.key {
        msg!("Unauthorized access");
        return Err(ProgramError::Custom(ErrorCode::UnauthorizedAccess as u32));
    }
    // Verify seller hasn't already rated
    if order.seller_rated {
        msg!("Already rated");
        return Err(ProgramError::Custom(ErrorCode::AlreadyRated as u32));
    }

    // Verify rating period hasn't expired
    let clock = Clock::get()?;
    if order.updated_at + marketplace.min_rating_period < clock.unix_timestamp {
        msg!("Rating period ended");
        return Err(ProgramError::Custom(ErrorCode::RatingPeriodEnded as u32));
    }
    // Initialize buyer rating
    let buyer_rating = BuyerRating {
        rating,
        comment,
        rated_at: clock.unix_timestamp,
    };

    // Update order status
    order.seller_rated = true;
    order.updated_at = clock.unix_timestamp;
    // Serialize the updated data
    buyer_rating.serialize(&mut &mut buyer_rating_account.data.borrow_mut()[..])?;
    order.serialize(&mut &mut order_account.data.borrow_mut()[..])?;

    Ok(())
}
