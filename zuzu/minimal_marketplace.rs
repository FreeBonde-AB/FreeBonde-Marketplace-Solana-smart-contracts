rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod minimal_marketplace {
    use super::*;

    // Initialize the marketplace with default values
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = *ctx.accounts.authority.key;
        marketplace.data_count = 0;
        marketplace.listing_count = 0;
        marketplace.transaction_count = 0;
        
        // Emit initialization event
        emit!(MarketplaceInitializedEvent {
            marketplace: marketplace.key(),
            authority: marketplace.authority,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Add simulated plant data to the blockchain
    pub fn add_data(ctx: Context<AddData>, data: SimulatedDataInput) -> Result<()> {
        let data_entry = &mut ctx.accounts.data_entry;
        data_entry.data = SimulatedData {
            temperature: data.temperature,
            humidity: data.humidity,
            ph: data.ph,
            ec: data.ec,
            plant_type: data.plant_type
        };
        data_entry.plant_id = data.plant_id;
        data_entry.minted = false;
        data_entry.data_id = ctx.accounts.marketplace.data_count;
        ctx.accounts.marketplace.data_count += 1;

        // Emit data added event
        emit!(DataAddedEvent {
            data_entry: data_entry.key(),
            plant_id: data_entry.plant_id.clone(),
            data_id: data_entry.data_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Mint NFT based on plant data
    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        // Check if the data has a nft minted
        require!(!ctx.accounts.data_entry.minted, ErrorCode::AlreadyMinted);
        // Check if the data_entry is valid
        require!(ctx.accounts.data_entry.data_id <= ctx.accounts.marketplace.data_count, ErrorCode::InvalidDataId);

        let nft = &mut ctx.accounts.nft;
        nft.owner = *ctx.accounts.minter.key;
        nft.data_id = ctx.accounts.data_entry.data_id;
        nft.plant_id = ctx.accounts.data_entry.plant_id.clone();
        nft.last_claimed = Clock::get()?.unix_timestamp;

        // Set data_entry as minted
        ctx.accounts.data_entry.minted = true;

        // Emit NFT minted event
        emit!(NftMintedEvent {
            nft: nft.key(),
            owner: nft.owner,
            data_id: nft.data_id,
            plant_id: nft.plant_id.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Distribute tokens to NFT holders as rewards
    pub fn distribute_tokens(ctx: Context<DistributeTokens>) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        let receiver = &mut ctx.accounts.receiver;
        let reward_amount = 10; // Example reward amount

        // Distribute tokens to NFT owner
        **receiver.try_borrow_mut_lamports()? += reward_amount;

        // Store the last claim time
        nft.last_claimed = Clock::get()?.unix_timestamp;

        // Emit token distribution event
        emit!(TokensDistributedEvent {
            nft: nft.key(),
            receiver: receiver.key(),
            amount: reward_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
    
    // Create listing to sell NFT
    pub fn create_listing(
        ctx: Context<CreateListing>,
        price: u64,
    ) -> Result<()> {
        // Validate price is greater than zero
        require!(price > 0, ErrorCode::InvalidPrice);
        
        let listing = &mut ctx.accounts.listing;
        listing.seller = *ctx.accounts.seller.key;
        listing.price = price;
        listing.nft = *ctx.accounts.nft.to_account_info().key;
        ctx.accounts.marketplace.listing_count += 1;

        // Emit listing created event
        emit!(ListingCreatedEvent {
            listing: listing.key(),
            seller: listing.seller,
            nft: listing.nft,
            price: listing.price,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Buy NFT from a listing
    pub fn buy_listing(ctx: Context<BuyListing>) -> Result<()> {
        // Check if the listing exists
        let listing = &ctx.accounts.listing;
        
        // Check if buyer has sufficient funds
        require!(
            ctx.accounts.buyer.lamports() >= listing.price,
            ErrorCode::InsufficientFunds
        );
        
        // Transfer lamports from buyer to seller
        **ctx.accounts.buyer.try_borrow_mut_lamports()? -= listing.price;
        **ctx.accounts.seller.try_borrow_mut_lamports()? += listing.price;

        // Transfer the NFT
        ctx.accounts.nft.owner = *ctx.accounts.buyer.to_account_info().key;
        ctx.accounts.marketplace.transaction_count += 1;

        // Emit listing purchased event
        emit!(ListingPurchasedEvent {
            listing: listing.key(),
            buyer: ctx.accounts.buyer.key(),
            seller: ctx.accounts.seller.key(),
            nft: ctx.accounts.nft.key(),
            price: listing.price,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Stake tokens for rewards
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Validate stake amount
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        let stake_account = &mut ctx.accounts.stake;
        stake_account.owner = *ctx.accounts.user.to_account_info().key;
        stake_account.amount += amount;
        stake_account.last_claimed = Clock::get()?.unix_timestamp;
        
        // Emit stake event
        emit!(StakeEvent {
            stake: stake_account.key(),
            user: stake_account.owner,
            amount: amount,
            total_staked: stake_account.amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    // Unstake tokens with potential penalty
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake;
        require!(stake_account.owner == *ctx.accounts.user.to_account_info().key, ErrorCode::Unauthorized);
        
        // Calculate the penalty
        let current_time = Clock::get()?.unix_timestamp;
        let time_since_last_claim = current_time - stake_account.last_claimed;
        let penalty_percentage = if time_since_last_claim < 30 {
            5 // 5% penalty
        } else {
            0
        };

        let penalty_amount = stake_account.amount * penalty_percentage / 100;
        let unstake_amount = stake_account.amount - penalty_amount;
        
        // Reset stake account
        let old_amount = stake_account.amount;
        stake_account.amount = 0;
        stake_account.owner = Pubkey::default();

        // Emit unstake event
        emit!(UnstakeEvent {
            stake: stake_account.key(),
            user: ctx.accounts.user.key(),
            amount: unstake_amount,
            penalty: penalty_amount,
            timestamp: current_time,
        });

        Ok(())
    }
}

// Data structure for the simulated data
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SimulatedData {
    pub temperature: Option<u8>,
    pub humidity: Option<u8>,
    pub ph: Option<u8>,
    pub ec: Option<u8>,
    pub plant_type: Option<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SimulatedDataInput {
    pub temperature: Option<u8>,
    pub humidity: Option<u8>,
    pub ph: Option<u8>,
    pub ec: Option<u8>,
    pub plant_type: Option<u8>,
    pub plant_id: String,
}

// Data entry account
#[account]
pub struct DataEntry {
    pub data: SimulatedData,
    pub minted: bool,
    pub data_id: u64,
    pub plant_id: String,
}

// NFT account
#[account]
pub struct Nft {
    pub owner: Pubkey,
    pub data_id: u64,
    pub last_claimed: i64,
    pub plant_id: String,
}

// Listing Account
#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub nft: Pubkey,
}

// Stake Account
#[account]
pub struct Stake {
    pub owner: Pubkey,
    pub amount: u64,
    pub last_claimed: i64,
}

// Marketplace Account
#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub data_count: u64,
    pub listing_count: u64,
    pub transaction_count: u64,
}

// Event definitions
#[event]
pub struct MarketplaceInitializedEvent {
    pub marketplace: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DataAddedEvent {
    pub data_entry: Pubkey,
    pub plant_id: String,
    pub data_id: u64,
    pub timestamp: i64,
}

#[event]
pub struct NftMintedEvent {
    pub nft: Pubkey,
    pub owner: Pubkey,
    pub data_id: u64,
    pub plant_id: String,
    pub timestamp: i64,
}

#[event]
pub struct TokensDistributedEvent {
    pub nft: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct ListingCreatedEvent {
    pub listing: Pubkey,
    pub seller: Pubkey,
    pub nft: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct ListingPurchasedEvent {
    pub listing: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub nft: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct StakeEvent {
    pub stake: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub total_staked: u64,
    pub timestamp: i64,
}

#[event]
pub struct UnstakeEvent {
    pub stake: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub penalty: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddData<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(init, payer = payer, space = 8 + 200 + 1 + 8 + 32)] // Increased space to 200
    pub data_entry: Account<'info, DataEntry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub data_entry: Account<'info, DataEntry>,
    #[account(init, payer = minter, space = 8 + 32 + 8 + 8 + 32)] // Add 32 for plant_id
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub minter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeTokens<'info> {
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(init, payer = seller, space = 8 + 32 + 8 + 32)]
    pub listing: Account<'info, Listing>,
    #[account(mut, constraint = nft.owner == seller.key() @ ErrorCode::Unauthorized)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyListing<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, constraint = listing.seller == seller.key() @ ErrorCode::Unauthorized)]
    pub seller: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 32 + 8 + 8)]
    pub stake: Account<'info, Stake>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub stake: Account<'info, Stake>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The data has already minted a nft")]
    AlreadyMinted,
    #[msg("Invalid data id")]
    InvalidDataId,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid amount")]
    InvalidAmount,
}
