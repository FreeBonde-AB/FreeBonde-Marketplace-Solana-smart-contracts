rust
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Failed,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum PetitionStatus {
    Active,
    Succeeded,
    Failed,
}

#[derive(Accounts)]
pub struct InitializeCouncil<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 1 + 8 + 4 + (32 * 15) + 8 + 1, // Assuming max 15 council members
    )]
    pub council: Account<'info, Council>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NominateCouncilMember<'info> {
    #[account(mut)]
    pub nominator: Signer<'info>,
    #[account(mut)]
    pub nominee: AccountInfo<'info>,
    #[account(
        mut,
        constraint = nominee_token_account.owner == nominee.key(),
    )]
    pub nominee_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = nominator,
        space = 8 + 32 + 32 + 4 + 100 + 4 + 500 + 8 + 1,
    )]
    pub nomination: Account<'info, Nomination>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteForCouncilMember<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        constraint = voter_token_account.owner == voter.key(),
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,
    #[account(
        init,
        payer = voter,
        space = 8 + 1 + 32 + 32 + 32 + 8,
    )]
    pub voter_record: Account<'info, VoterRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeCouncilElection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(
        seeds = [b"nomination".as_ref(), council.key().as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nominations: UncheckedAccount<'info>,
    
}

#[derive(Accounts)]
pub struct CreateEmergencyAction<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 32 + 4 + 100 + 4 + 500 + 1 + 4 + 150 + 8 + 4 + 1 + 1,
    )]
    pub emergency_action: Account<'info, EmergencyAction>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnEmergencyAction<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct ExecuteEmergencyAction<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct CancelEmergencyAction<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct InitializeCrisisManagement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 8 + 8 + 1 + 1,
    )]
    pub crisis_manager: Account<'info, CrisisManager>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeclareCrisis<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub crisis_manager: Account<'info, CrisisManager>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 4 + 500 + 8 + 8 + 1 + 1 + 1,
    )]
    pub crisis_record: Account<'info, CrisisRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveCrisis<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub crisis_manager: Account<'info, CrisisManager>,
    #[account(mut)]
    pub crisis_record: Account<'info, CrisisRecord>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(
        mut,
        constraint = proposer_token_account.owner == proposer.key(),
    )]
    pub proposer_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 4 + 100 + 4 + 1000 + 1 + 8 + 8 + 1 + 32 + 4 + 200 + 8, // Adjusted space
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        constraint = voter_token_account.owner == voter.key(),
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"token-account-data", voter.key().as_ref()],
        bump,
    )]
    pub voter_token_data_mut: Account<'info, TokenAccountData>,
        #[account(
        
        seeds = [b"token-account-data", voter.key().as_ref()],
        bump,
    )]
    pub voter_token_data: Account<'info, TokenAccountData>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + 1 + 32 + 32 + 1 + 8,
    )]
    pub voter_record: Account<'info, VoterRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CreatePetition<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 4 + 100 + 4 + 1000 + 4 + 8 + 8 + 1 + 1,
    )]
    pub petition: Account<'info, Petition>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignPetition<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub petition: Account<'info, Petition>,
    #[account(
        init,
        payer = signer,
        space = 8 + 1 + 32 + 32 + 8,
    )]
    pub signature: Account<'info, Signature>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConvertPetitionToProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub petition: Account<'info, Petition>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 100 + 4 + 1000 + 1 + 8 + 8 + 1 + 32 + 4 + 200 + 8, // Adjusted space
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Council {
    pub authority: Pubkey,
    pub council_size: u8,
    pub quorum_percentage: u8,
    pub term_length: i64,
    pub members: Vec<Pubkey>,
    pub created_at: i64,
    pub last_election: i64,
    pub active: bool,
}

#[account]
pub struct Nomination {
    pub nominee: Pubkey,
    pub council: Pubkey,
    pub name: String,
    pub statement: String,
    pub vote_count: u64,
    pub nominated_at: i64,
    pub elected: bool,
}

#[account]
pub struct VoterRecord {
    pub has_voted: bool,
    pub council: Pubkey,
    pub voter: Pubkey,
    pub nomination: Pubkey,
    pub voting_power: u64,
    pub proposal: Pubkey,
    pub option_index: u8,
}

#[account]
pub struct EmergencyAction {
    pub proposer: Pubkey,
    pub council: Pubkey,
    pub title: String,
    pub description: String,
    pub action_type: u8, // 1=Fee Change, 2=Contract Pause, 3=Fund Reallocation
    pub parameters: Vec<u8>,
    pub created_at: i64,
    pub approvals: Vec<Pubkey>,
    pub rejections: Vec<Pubkey>,
    pub executed: bool,
    pub cancelled: bool,
}

#[account]
pub struct CrisisManager {
    pub authority: Pubkey,
    pub council: Pubkey,
    pub active_crisis: bool,
    pub crisis_start_time: i64,
    pub crisis_end_time: i64,
    pub zero_fee_enabled: bool,
    pub emergency_fund_unlocked: bool,
}

#[account]
pub struct CrisisRecord {
    pub authority: Pubkey,
    pub crisis_type: u8, // 1=Natural Disaster, 2=Supply Chain Disruption, 3=Economic Crisis
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub zero_fee_enabled: bool,
    pub emergency_fund_unlocked: bool,
    pub resolved: bool,
}

#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: u8,
    pub created_at: i64,
    pub voting_end_time: i64,
    pub executed: bool,
    pub cancelled: bool,
    pub status: u8,
    pub dao: Pubkey,
    pub options: Vec<String>,
    pub votes: Vec<u64>,
    pub total_votes_cast:u64,
}

#[account]
pub struct Dao {
    
}

#[account]
pub struct Petition {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub required_signatures: u32,
    pub signature_count: u32,
    pub created_at: i64,
    pub expires_at: i64,
    pub converted_to_proposal: bool,
    pub status: u8,
}

#[account]
pub struct Signature {
    pub has_signed: bool,
    pub petition: Pubkey,
    pub signer: Pubkey,
    pub timestamp: i64,
}
#[account]
pub struct TokenAccountData {
    pub acquisition_time: i64,
    pub governance_participation_count: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid council size")]
    InvalidCouncilSize,
    #[msg("Invalid quorum percentage")]
    InvalidQuorumPercentage,
    #[msg("Invalid term length")]
    InvalidTermLength,
    #[msg("Nomination period is closed")]
    NominationPeriodClosed,
    #[msg("Insufficient tokens for nomination")]
    InsufficientTokensForNomination,
    #[msg("Election period is closed")]
    ElectionPeriodClosed,
    #[msg("Already voted in this election")]
    AlreadyVotedInElection,
    #[msg("Election period too short")]
    ElectionPeriodTooShort,
    #[msg("No nominations found")]
    NoNominations,
    #[msg("Council is not active")]
    CouncilNotActive,
    #[msg("Not a council member")]
    NotCouncilMember,
    #[msg("Invalid action type")]
    InvalidActionType,
    #[msg("Action already executed")]
    ActionAlreadyExecuted,
    #[msg("Action already cancelled")]
    ActionAlreadyCancelled,
    #[msg("Already voted on this action")]
    AlreadyVotedOnAction,
    #[msg("Quorum not reached")]
    QuorumNotReached,
    #[msg("Not authorized to cancel action")]
    NotAuthorizedToCancelAction,
    #[msg("Invalid crisis type")]
    InvalidCrisisType,
    #[msg("Invalid crisis duration")]
    InvalidCrisisDuration,
    #[msg("No crisis is currently active")]
    NoCrisisActive,
    #[msg("Invalid proposal type")]
    InvalidProposalType,
    #[msg("Invalid voting period")]
    InvalidVotingPeriod,
    #[msg("Insufficient options")]
    InsufficientOptions,
    #[msg("Insufficient tokens to create proposal")]
    InsufficientTokens,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Proposal has been cancelled")]
    ProposalCancelled,
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    #[msg("You have already voted")]
    AlreadyVoted,
    #[msg("Voting period has not ended yet")]
    VotingPeriodNotEnded,
    #[msg("No votes were cast")]
    NoVotesCast,
    #[msg("Insufficient approval")]
    InsufficientApproval,
    #[msg("This proposal has already been executed")]
    ProposalAlreadyExecuted,
    #[msg("You are not authorized to cancel this proposal")]
    NotAuthorizedToCancelProposal,
    #[msg("Invalid signature requirement")]
    InvalidSignatureRequirement,
    #[msg("Petition has expired")]
    PetitionExpired,
    #[msg("Petition has already been converted to a proposal")]
    PetitionAlreadyConverted,
    #[msg("You have already signed this petition")]
    AlreadySigned,
    #[msg("Insufficient signatures collected")]
    InsufficientSignatures,
}

#[event]
pub struct CouncilNominationEvent {
    pub nomination_id: Pubkey,
    pub nominee: Pubkey,
    pub council_id: Pubkey,
}

#[event]
pub struct CouncilVoteEvent {
    pub nomination_id: Pubkey,
    pub voter: Pubkey,
    pub voting_power: u64,
}

#[event]
pub struct CouncilElectedEvent {
    pub council_id: Pubkey,
    pub member_count: u8,
    pub election_time: i64,
}

#[event]
pub struct EmergencyActionCreatedEvent {
    pub action_id: Pubkey,
    pub proposer: Pubkey,
    pub action_type: u8,
}

#[event]
pub struct EmergencyActionVoteEvent {
    pub action_id: Pubkey,
    pub voter: Pubkey,
    pub approved: bool,
}

#[event]
pub struct EmergencyActionExecutedEvent {
    pub action_id: Pubkey,
    pub executor: Pubkey,
    pub approval_count: u8,
}

#[event]
pub struct EmergencyActionCancelledEvent {
    pub action_id: Pubkey,
    pub cancelled_by: Pubkey,
}

#[event]
pub struct CrisisDeclaredEvent {
    pub crisis_id: Pubkey,
    pub declarer: Pubkey,
    pub crisis_type: u8,
    pub duration: i64,
}

#[event]
pub struct CrisisResolvedEvent {
    pub crisis_id: Pubkey,
    pub resolver: Pubkey,
    pub resolution_time: i64,
}

#[event]
pub struct ProposalCreatedEvent {
    pub proposal_id: Pubkey,
    pub proposer: Pubkey,
    pub proposal_type: u8,
    pub voting_end_time: i64,
}

#[event]
pub struct VoteCastEvent {
    pub proposal_id: Pubkey,
    pub voter: Pubkey,
    pub option_index: u8,
    pub voting_power: u64,
}

#[event]
pub struct ProposalExecutedEvent {
    pub proposal_id: Pubkey,
    pub executor: Pubkey,
    pub winning_option: u8,
    pub approval_percentage: u8,
}

#[event]
pub struct ProposalCancelledEvent {
    pub proposal_id: Pubkey,
    pub cancelled_by: Pubkey,
}

#[event]
pub struct PetitionCreatedEvent {
    pub petition_id: Pubkey,
    pub creator: Pubkey,
    pub required_signatures: u32,
    pub expires_at: i64,
}

#[event]
pub struct PetitionSignedEvent {
    pub petition_id: Pubkey,
    pub signer: Pubkey,
    pub signature_count: u32,
}

#[event]
pub struct PetitionConvertedEvent {
    pub petition_id: Pubkey,
    pub proposal_id: Pubkey,
}

#[program]
pub mod freebonde_governance {
    use super::*;

    // ---------- Council Management ----------

    // Initialize governance council
    pub fn initialize_council(
        ctx: Context<InitializeCouncil>,
        council_size: u8,
        quorum_percentage: u8,
        term_length: i64,
    ) -> Result<()> {
        // Validate council parameters
        require!(council_size >= 3 && council_size <= 15, ErrorCode::InvalidCouncilSize);
        require!(quorum_percentage >= 51 && quorum_percentage <= 100, ErrorCode::InvalidQuorumPercentage);
        require!(term_length >= 90 * 24 * 60 * 60, ErrorCode::InvalidTermLength); // Minimum 90 days
        
        // Initialize council state
        let council = &mut ctx.accounts.council;
        council.authority = ctx.accounts.authority.key();
        council.council_size = council_size;
        council.quorum_percentage = quorum_percentage;
        council.term_length = term_length;
        council.members = Vec::new();
        council.created_at = Clock::get()?.unix_timestamp;
        council.last_election = 0;
        council.active = false;
        
        Ok(())
    }
    
    // Nominate council member
    pub fn nominate_council_member(
        ctx: Context<NominateCouncilMember>,
        name: String,
        statement: String,
    ) -> Result<()> {
        // Verify nomination period is active
        let council = &ctx.accounts.council;
        let current_time = Clock::get()?.unix_timestamp;
        let election_period_active = council.last_election == 0 || 
                                    (current_time - council.last_election) >= council.term_length;
        
        require!(election_period_active, ErrorCode::NominationPeriodClosed);
        
        // Verify nominee has sufficient tokens (minimum 50,000 FARM)
        let required_tokens = 50_000 * 10u64.pow(9);
        let token_balance = ctx.accounts.nominee_token_account.amount;
        require!(token_balance >= required_tokens, ErrorCode::InsufficientTokensForNomination);
        
        // Create nomination
        let nomination = &mut ctx.accounts.nomination;
        nomination.nominee = ctx.accounts.nominee.key();
        nomination.council = ctx.accounts.council.key();
        nomination.name = name;
        nomination.statement = statement;
        nomination.vote_count = 0;
        nomination.nominated_at = current_time;
        nomination.elected = false;
        
        // Emit nomination event
        emit!(CouncilNominationEvent {
            nomination_id: nomination.key(),
            nominee: nomination.nominee,
            council_id: council.key(),
        });
        
        Ok(())
    }
    
    // Vote for council member
    pub fn vote_for_council_member(ctx: Context<VoteForCouncilMember>) -> Result<()> {
        // Verify election period is active
        let council = &ctx.accounts.council;
        let current_time = Clock::get()?.unix_timestamp;
        let election_period_active = council.last_election == 0 || 
                                    (current_time - council.last_election) >= council.term_length;
        
        require!(election_period_active, ErrorCode::ElectionPeriodClosed);
        
        // Verify voter hasn't already voted for this position
        let voter_record = &mut ctx.accounts.voter_record;
        require!(!voter_record.has_voted, ErrorCode::AlreadyVotedInElection);
        
        // Calculate voting power based on token balance
        let token_balance = ctx.accounts.voter_token_account.amount;
        
        // Record vote
        let nomination = &mut ctx.accounts.nomination;
        nomination.vote_count += token_balance;
        
        // Update voter record
        voter_record.has_voted = true;
        voter_record.council = council.key();
        voter_record.voter = ctx.accounts.voter.key();
        voter_record.nomination = nomination.key();
        voter_record.voting_power = token_balance;
        
        // Emit vote event
        emit!(CouncilVoteEvent {
            nomination_id: nomination.key(),
            voter: voter_record.voter,
            voting_power: token_balance,
        });
        
        Ok(())
    }
    
    // Finalize council election
    pub fn finalize_council_election(ctx: Context<FinalizeCouncilElection>) -> Result<()> {
        // Verify election period has ended (minimum 14 days after first nomination)
        let council = &mut ctx.accounts.council;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Get all nominations
        let nominations = ctx.accounts.nominations.to_vec();
        require!(!nominations.is_empty(), ErrorCode::NoNominations);
        
        let first_nomination_time = nominations
            .iter()
            .map(|n| n.nominated_at)
            .min()
            .unwrap_or(current_time);
            
        let election_duration = current_time - first_nomination_time;
        require!(election_duration >= 14 * 24 * 60 * 60, ErrorCode::ElectionPeriodTooShort);
        
        // Sort nominations by vote count (descending)
        let mut sorted_nominations = nominations.clone();
        sorted_nominations.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));
        
        // Select top candidates based on council size
        let elected_count = std::cmp::min(council.council_size as usize, sorted_nominations.len());
        let elected_nominations = &sorted_nominations[0..elected_count];
        
        // Update council members
        council.members.clear();
        for nomination in elected_nominations {
            council.members.push(nomination.nominee);
            
            // Mark nomination as elected
            let mut nom = nomination.clone();
            nom.elected = true;
            // In a real implementation, we would update the nomination account
        }
        
        // Update council state
        council.last_election = current_time;
        council.active = true;
        
        // Emit council elected event
        emit!(CouncilElectedEvent {
            council_id: council.key(),
            member_count: elected_count as u8,
            election_time: current_time,
        });
        
        Ok(())
    }
    
    // ---------- Emergency Actions ----------

    // Create emergency action proposal (council only)
    pub fn create_emergency_action(
        ctx: Context<CreateEmergencyAction>,
        title: String,
        description: String,
        action_type: u8, // 1=Fee Change, 2=Contract Pause, 3=Fund Reallocation
        parameters: Vec<u8>, // Serialized parameters for the action
    ) -> Result<()> {
        // Verify caller is a council member
        let council = &ctx.accounts.council;
        require!(council.active, ErrorCode::CouncilNotActive);
        
        let is_council_member = council.members.contains(&ctx.accounts.proposer.key());
        require!(is_council_member, ErrorCode::NotCouncilMember);
        
        // Validate action type
        require!(action_type >= 1 && action_type <= 3, ErrorCode::InvalidActionType);
        
        // Create emergency action
        let action = &mut ctx.accounts.emergency_action;
        action.proposer = ctx.accounts.proposer.key();
        action.council = council.key();
        action.title = title;
        action.description = description;
        action.action_type = action_type;
        action.parameters = parameters;
        action.created_at = Clock::get()?.unix_timestamp;
        action.approvals = Vec::new();
        action.rejections = Vec::new();
        action.executed = false;
        action.cancelled = false;
        
        // Emit emergency action created event
        emit!(EmergencyActionCreatedEvent {
            action_id: action.key(),
            proposer: action.proposer,
            action_type,
        });
        
        Ok(())
    }
    
    // Vote on emergency action (council members only)
    pub fn vote_on_emergency_action(
        ctx: Context<VoteOnEmergencyAction>,
        approve: bool,
    ) -> Result<()> {
        // Verify caller is a council member
        let council = &ctx.accounts.council;
        require!(council.active, ErrorCode::CouncilNotActive);
        
        let is_council_member = council.members.contains(&ctx.accounts.voter.key());
        require!(is_council_member, ErrorCode::NotCouncilMember);
        
        // Verify action hasn't been executed or cancelled
        let action = &mut ctx.accounts.emergency_action;
        require!(!action.executed, ErrorCode::ActionAlreadyExecuted);
        require!(!action.cancelled, ErrorCode::ActionAlreadyCancelled);
        
        // Verify council member hasn't already voted
        let already_approved = action.approvals.contains(&ctx.accounts.voter.key());
        let already_rejected = action.rejections.contains(&ctx.accounts.voter.key());
        require!(!already_approved && !already_rejected, ErrorCode::AlreadyVotedOnAction);
        
        // Record vote
        if approve {
            action.approvals.push(ctx.accounts.voter.key());
        } else {
            action.rejections.push(ctx.accounts.voter.key());
        }
        
        // Emit vote event
        emit!(EmergencyActionVoteEvent {
            action_id: action.key(),
            voter: ctx.accounts.voter.key(),
            approved: approve,
        });
        
        Ok(())
    }
    
    // Execute emergency action if quorum reached
    pub fn execute_emergency_action(ctx: Context<ExecuteEmergencyAction>) -> Result<()> {
        // Verify action hasn't been executed or cancelled
        let action = &mut ctx.accounts.emergency_action;
        require!(!action.executed, ErrorCode::ActionAlreadyExecuted);
        require!(!action.cancelled, ErrorCode::ActionAlreadyCancelled);
        
        // Verify quorum has been reached
        let council = &ctx.accounts.council;
        let approval_count = action.approvals.len() as u8;
        let total_council_size = council.council_size;
        let quorum_required = (total_council_size * council.quorum_percentage) / 100;
        
        require!(approval_count >= quorum_required, ErrorCode::QuorumNotReached);
        
        // Execute the emergency action based on action type
        match action.action_type {
            1 => {
                // Fee Change
                // Implementation would include fee change logic
            },
            2 => {
                // Contract Pause
                // Implementation would include contract pause logic
            },
            3 => {
                // Fund Reallocation
                // Implementation would include fund reallocation logic
            },
            _ => return Err(ErrorCode::InvalidActionType.into()),
        }
        
        // Mark action as executed
        action.executed = true;
        
        // Emit action executed event
        emit!(EmergencyActionExecutedEvent {
            action_id: action.key(),
            executor: ctx.accounts.executor.key(),
            approval_count,
        });
        
        Ok(())
    }
    
    // Cancel emergency action (proposer or majority rejection)
    pub fn cancel_emergency_action(ctx: Context<CancelEmergencyAction>) -> Result<()> {
        // Verify action hasn't been executed or cancelled
        let action = &mut ctx.accounts.emergency_action;
        require!(!action.executed, ErrorCode::ActionAlreadyExecuted);
        require!(!action.cancelled, ErrorCode::ActionAlreadyCancelled);
        
        // Verify caller is the proposer or majority of council rejected
        let is_proposer = action.proposer == ctx.accounts.authority.key();
        let rejection_count = action.rejections.len() as u8;
        let council = &ctx.accounts.council;
        let majority_rejected = rejection_count > council.council_size / 2;
        
        require!(is_proposer || majority_rejected, ErrorCode::NotAuthorizedToCancelAction);
        
        // Mark action as cancelled
        action.cancelled = true;
        
        // Emit action cancelled event
        emit!(EmergencyActionCancelledEvent {
            action_id: action.key(),
            cancelled_by: ctx.accounts.authority.key(),
        });
        
        Ok(())
    }
    
    // ---------- Crisis Management ----------

    // Add crisis management functionality
    pub fn initialize_crisis_management(ctx: Context<InitializeCrisisManagement>) -> Result<()> {
        let crisis_manager = &mut ctx.accounts.crisis_manager;
        crisis_manager.authority = ctx.accounts.authority.key();
        crisis_manager.council = ctx.accounts.council.key();
        crisis_manager.active_crisis = false;
        crisis_manager.crisis_start_time = 0;
        crisis_manager.crisis_end_time = 0;
        crisis_manager.zero_fee_enabled = false;
        crisis_manager.emergency_fund_unlocked = false;
        
        Ok(())
    }
    
    // Declare crisis situation (requires council approval)
    pub fn declare_crisis(
        ctx: Context<DeclareCrisis>,
        crisis_type: u8, // 1=Natural Disaster, 2=Supply Chain Disruption, 3=Economic Crisis
        description: String,
        duration: i64, // Crisis duration in seconds
        zero_fee_enabled: bool,
        emergency_fund_unlocked: bool,
    ) -> Result<()> {
        // Verify caller is a council member
        let council = &ctx.accounts.council;
        require!(council.active, ErrorCode::CouncilNotActive);
        
        let is_council_member = council.members.contains(&ctx.accounts.authority.key());
        require!(is_council_member, ErrorCode::NotCouncilMember);
        
        // Validate crisis parameters
        require!(crisis_type >= 1 && crisis_type <= 3, ErrorCode::InvalidCrisisType);
        require!(duration >= 24 * 60 * 60 && duration <= 30 * 24 * 60 * 60, ErrorCode::InvalidCrisisDuration);
        
        // Update crisis manager state
        let crisis_manager = &mut ctx.accounts.crisis_manager;
        crisis_manager