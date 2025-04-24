rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer as TokenTransfer};
use std::cmp;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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

#[program]
pub mod freebonde_tokens {
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
        crisis_manager.active_crisis = true;
        crisis_manager.crisis_start_time = Clock::get()?.unix_timestamp;
        crisis_manager.crisis_end_time = crisis_manager.crisis_start_time + duration;
        crisis_manager.zero_fee_enabled = zero_fee_enabled;
        crisis_manager.emergency_fund_unlocked = emergency_fund_unlocked;

        // Create crisis record
        let crisis_record = &mut ctx.accounts.crisis_record;
        crisis_record.authority = ctx.accounts.authority.key();
        crisis_record.crisis_type = crisis_type;
        crisis_record.description = description;
        crisis_record.start_time = crisis_manager.crisis_start_time;
        crisis_record.end_time = crisis_manager.crisis_end_time;
        crisis_record.zero_fee_enabled = zero_fee_enabled;
        crisis_record.emergency_fund_unlocked = emergency_fund_unlocked;
        crisis_record.resolved = false;

        // Emit crisis declared event
        emit!(CrisisDeclaredEvent {
            crisis_id: crisis_record.key(),
            declarer: ctx.accounts.authority.key(),
            crisis_type,
            duration,
        });

        Ok(())
    }

    // Resolve crisis situation
    pub fn resolve_crisis(ctx: Context<ResolveCrisis>) -> Result<()> {
        // Verify caller is a council member
        let council = &ctx.accounts.council;
        require!(council.active, ErrorCode::CouncilNotActive);

        let is_council_member = council.members.contains(&ctx.accounts.authority.key());
        require!(is_council_member, ErrorCode::NotCouncilMember);

        // Update crisis manager state
        let crisis_manager = &mut ctx.accounts.crisis_manager;
        require!(crisis_manager.active_crisis, ErrorCode::NoCrisisActive);

        crisis_manager.active_crisis = false;
        crisis_manager.zero_fee_enabled = false;
        crisis_manager.emergency_fund_unlocked = false;

        // Update crisis record
        let crisis_record = &mut ctx.accounts.crisis_record;
        crisis_record.resolved = true;
        crisis_record.end_time = Clock::get()?.unix_timestamp;

        // Emit crisis resolved event
        emit!(CrisisResolvedEvent {
            crisis_id: crisis_record.key(),
            resolver: ctx.accounts.authority.key(),
            resolution_time: crisis_record.end_time,
        });

        Ok(())
    }
    // ---------- Core Governance ----------

    // Create a new governance proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: u8, // 1=Level 1, 2=Level 2, 3=Level 3
        options: Vec<String>,
        voting_period: i64,
    ) -> Result<()> {
        // Validate proposal type
        require!(
            proposal_type >= 1 && proposal_type <= 3,
            ErrorCode::InvalidProposalType
        );

        // Validate voting period (minimum 3 days, maximum 30 days)
        require!(
            voting_period >= 3 * 24 * 60 * 60 && voting_period <= 30 * 24 * 60 * 60,
            ErrorCode::InvalidVotingPeriod
        );

        // Validate options (at least 2 options required)
        require!(options.len() >= 2, ErrorCode::InsufficientOptions);

        // Verify proposer has sufficient tokens based on proposal type
        let required_tokens = match proposal_type {
            1 => 5_000 * 10u64.pow(9),    // Level 1: 5,000 FARM
            2 => 25_000 * 10u64.pow(9),   // Level 2: 25,000 FARM
            3 => 75_000 * 10u64.pow(9),   // Level 3: 75,000 FARM
            _ => return Err(ErrorCode::InvalidProposalType.into()),
        };

        // Check token balance
        let token_balance = ctx.accounts.proposer_token_account.amount;
        require!(token_balance >= required_tokens, ErrorCode::InsufficientTokens);

        // Create proposal
        let proposal = &mut ctx.accounts.proposal;
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.voting_end_time = Clock::get()?.unix_timestamp + voting_period;
        proposal.executed = false;
        proposal.cancelled = false;
        proposal.status = ProposalStatus::Draft as u8;
        proposal.dao = ctx.accounts.dao.key();
        proposal.options = options;
        proposal.votes = vec![0; proposal.options.len()];
        proposal.total_votes_cast = 0;

        // Emit proposal created event
        emit!(ProposalCreatedEvent {
            proposal_id: proposal.key(),
            proposer: proposal.proposer,
            proposal_type,
            voting_end_time: proposal.voting_end_time,
        });

        Ok(())
    }

    // Cast a vote on a proposal
    pub fn cast_vote(ctx: Context<CastVote>, option_index: u8) -> Result<()> {
        // Verify proposal is still in voting period
        let current_time = Clock::get()?.unix_timestamp;
        let proposal = &mut ctx.accounts.proposal;
        require!(
            current_time <= proposal.voting_end_time,
            ErrorCode::VotingPeriodEnded
        );

        // Verify proposal is not cancelled
        require!(!proposal.cancelled, ErrorCode::ProposalCancelled);

        // Verify option index is valid
        require!(
            (option_index as usize) < proposal.options.len(),
            ErrorCode::InvalidOptionIndex
        );

        // Verify voter hasn't already voted
        let voter_record = &mut ctx.accounts.voter_record;
        require!(!voter_record.has_voted, ErrorCode::AlreadyVoted);

        // Calculate voting power
        let token_balance = ctx.accounts.voter_token_account.amount;

        // Calculate time multiplier (max 2x for 1+ year holders)
        let token_account_data = &ctx.accounts.voter_token_data;
        let holding_duration = current_time - token_account_data.acquisition_time;
        let time_multiplier = if holding_duration >= 365 * 24 * 60 * 60 {
            200 // 2x
        } else {
            100 + (holding_duration * 100 / (365 * 24 * 60 * 60))
        };

        // Calculate participation multiplier (max 1.5x)
        let participation_count = token_account_data.governance_participation_count;
        let participation_multiplier = 100 + (participation_count.min(10) * 5); // Max 1.5x

        // Calculate final voting power
        let voting_power = token_balance * time_multiplier as u64 * participation_multiplier as u64 / 10000;

        // Record vote
        proposal.votes[option_index as usize] += voting_power;
        proposal.total_votes_cast += voting_power;

        // Update voter record
        voter_record.has_voted = true;
        voter_record.proposal = proposal.key();
        voter_record.voter = ctx.accounts.voter.key();
        voter_record.option_index = option_index;
        voter_record.voting_power = voting_power;

        // Update voter participation count
        let mut token_account_data = &mut ctx.accounts.voter_token_data_mut;
        token_account_data.governance_participation_count += 1;

        // Emit vote cast event         
        emit!(VoteCastEvent {
            proposal_id: proposal.key(),
            voter: voter_record.voter,
            option_index,
            voting_power,
        });

        Ok(())
    }

    // Execute a proposal after voting period ends
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        // Verify voting period has ended
        let current_time = Clock::get()?.unix_timestamp;
        let proposal = &mut ctx.accounts.proposal;
        require!(
            current_time > proposal.voting_end_time,
            ErrorCode::VotingPeriodNotEnded
        );

        // Verify proposal hasn't been executed or cancelled
        require!(!proposal.executed, ErrorCode::ProposalAlreadyExecuted);
        require!(!proposal.cancelled, ErrorCode::ProposalCancelled);

        // Calculate voting results
        let total_votes = proposal.total_votes_cast;
        require!(total_votes > 0, ErrorCode::NoVotesCast);

        // Find winning option
        let (winning_index, winning_votes) = proposal.votes
            .iter()
            .enumerate()
            .max_by_key(|&(_, &votes)| votes)
            .unwrap_or((0, &0));

        // Calculate approval percentage
        let approval_percentage = winning_votes * 100 / total_votes;

        // Check if proposal meets required approval threshold
        let required_percentage = match proposal.proposal_type {
            1 => 51, // Level 1: Simple majority (51%)
            2 => 75, // Level 2: Super majority (75%)
            3 => 80, // Level 3: High consensus (80%)
            _ => return Err(ErrorCode::InvalidProposalType.into()),
        };

        require!(
            approval_percentage >= required_percentage,
            ErrorCode::InsufficientApproval
        );

        // Mark proposal as executed
        proposal.executed = true;

        // Execute proposal actions based on proposal content
        // Implementation would include specific execution logic

        // Emit proposal executed event
        emit!(ProposalExecutedEvent {
            proposal_id: proposal.key(),
            executor: ctx.accounts.executor.key(),
            winning_option: winning_index as u8,
            approval_percentage: approval_percentage as u8,
        });

        Ok(())
    }

    // Cancel a proposal (only by proposer or authorized governance admin)
    pub fn cancel_proposal(ctx: Context<CancelProposal>) -> Result<()> {
        // Verify proposal hasn't been executed or already cancelled
        let proposal = &mut ctx.accounts.proposal;
        require!(!proposal.executed, ErrorCode::ProposalAlreadyExecuted);
        require!(!proposal.cancelled, ErrorCode::ProposalCancelled);

        // Verify caller is the proposer or authorized admin
        let is_proposer = proposal.proposer == ctx.accounts.authority.key();
        let is_admin = false; // Would check against a list of authorized admins

        require!(
            is_proposer || is_admin,
            ErrorCode::NotAuthorizedToCancelProposal
        );

        // Mark proposal as cancelled
        proposal.cancelled = true;

        // Emit proposal cancelled event
        emit!(ProposalCancelledEvent {
            proposal_id: proposal.key(),
            cancelled_by: ctx.accounts.authority.key(),
        });

        Ok(())
    }

    // Create a petition (alternative way to create Level 1 proposals)
    pub fn create_petition(
        ctx: Context<CreatePetition>,
        title: String,
        description: String,
        required_signatures: u32,
    ) -> Result<()> {
        // Validate required signatures (minimum 100, maximum 1000)
        require!(
            required_signatures >= 100 && required_signatures <= 1000,
            ErrorCode::InvalidSignatureRequirement
        );

        // Create petition
        let petition = &mut ctx.accounts.petition;
        petition.creator = ctx.accounts.creator.key();
        petition.title = title;
        petition.description = description;
        petition.required_signatures = required_signatures;
        petition.signature_count = 0;
        petition.created_at = Clock::get()?.unix_timestamp;
        petition.expires_at = Clock::get()?.unix_timestamp + (30 * 24 * 60 * 60); // 30 days
        petition.converted_to_proposal = false;
        petition.status = PetitionStatus::Active as u8;

        // Emit petition created event
        emit!(PetitionCreatedEvent {
            petition_id: petition.key(),
            creator: petition.creator,
            required_signatures,
            expires_at: petition.expires_at,
        });

        Ok(())
    }

    // Sign a petition
    pub fn sign_petition(ctx: Context<SignPetition>) -> Result<()> {
        // Verify petition hasn't expired
        let current_time = Clock::get()?.unix_timestamp;
        let petition = &mut ctx.accounts.petition;
        require!(
            current_time <= petition.expires_at,
            ErrorCode::PetitionExpired
        );

        // Verify petition hasn't been converted to a proposal
        require!(
            !petition.converted_to_proposal,
            ErrorCode::PetitionAlreadyConverted
        );

        // Verify signer hasn't already signed
        let signature = &mut ctx.accounts.signature;
        require!(
            !signature.has_signed,
            ErrorCode::AlreadySigned
        );

        // Record signature
        signature.has_signed = true;
        signature.petition = petition.key();
        signature.signer = ctx.accounts.signer.key();
        signature.timestamp = current_time;

        // Increment signature count
        petition.signature_count += 1;
        if petition.signature_count == petition.required_signatures{
            petition.status = PetitionStatus::Succeeded as u8;
        }

        // Emit petition signed event
        emit!(PetitionSignedEvent {
            petition_id: petition.key(),
            signer: signature.signer,
            signature_count: petition.signature_count,
        });

        Ok(())
    }

    // Convert petition to proposal when enough signatures are collected
    pub fn convert_petition_to_proposal(
        ctx: Context<ConvertPetitionToProposal>,
        options: Vec<String>,
        voting_period: i64,
    ) -> Result<()> {
        // Verify petition hasn't expired
        let current_time = Clock::get()?.unix_timestamp;
        let petition = &mut ctx.accounts.petition;
        require!(
            current_time <= petition.expires_at,
            ErrorCode::PetitionExpired
        );

        // Verify petition hasn't been converted to a proposal
        require!(
            !petition.converted_to_proposal,
            ErrorCode::PetitionAlreadyConverted
        );

        // Verify petition has enough signatures
        require!(
            petition.signature_count >= petition.required_signatures,
            ErrorCode::InsufficientSignatures
        );

        // Validate voting period (minimum 3 days, maximum 30 days)
        require!(
            voting_period >= 3 * 24 * 60 * 60 && voting_period <= 30 * 24 * 60 * 60,
            ErrorCode::InvalidVotingPeriod
        );

        // Validate options (at least 2 options required)
        require!(options.len() >= 2, ErrorCode::InsufficientOptions);

        // Create Level 1 proposal from petition
        let proposal = &mut ctx.accounts.proposal;
        proposal.proposer = petition.creator;
        proposal.title = petition.title.clone();
        proposal.description = petition.description.clone();
        proposal.proposal_type = 1; // Level 1 proposal
        proposal.created_at = current_time;
        proposal.voting_end_time = current_time + voting_period;
        proposal.executed = false;
        proposal.cancelled = false;
        proposal.status = ProposalStatus::Draft as u8;
        proposal.dao = ctx.accounts.dao.key();
        proposal.options = options;
        proposal.votes = vec![0; proposal.options.len()];
        proposal.total_votes_cast = 0;

        // Mark petition as converted
        petition.converted_to_proposal = true;

        // Emit events
        emit!(PetitionConvertedEvent {
            petition_id: petition.key(),
            proposal_id: proposal.key(),
        });

        emit!(ProposalCreatedEvent {
            proposal_id: proposal.key(),
            proposer: proposal.proposer,
            proposal_type: 1,
            voting_end_time: proposal.voting_end_time,
        });

        Ok(())
    }
        // Initialize GROW token with a total supply of 1 billion
    pub fn initialize_grow(ctx: Context<InitializeToken>) -> Result<()> {
        // Create GROW token with 1 billion total supply (with 9 decimals)
        let total_supply = 1_000_000_000 * 10u64.pow(9);
        
        // Set token allocations according to tokenomics
        let ecosystem_rewards = total_supply * 40 / 100; // 40%
        let team_and_advisors = total_supply * 15 / 100; // 15%
        let private_sale = total_supply * 10 / 100; // 10%
        let public_sale = total_supply * 5 / 100; // 5%
        let liquidity_provision = total_supply * 10 / 100; // 10%
        let community_development = total_supply * 20 / 100; // 20%
        
        // Mint tokens to allocation accounts
        // Implementation will include minting logic and permission checks
        
        Ok(())
    }

    // Initialize FARM governance token with a total supply of 100 million
    pub fn initialize_farm(ctx: Context<InitializeToken>) -> Result<()> {
        // Create FARM token with 100 million total supply (with 9 decimals)
        let total_supply = 100_000_000 * 10u64.pow(9);
        
        // Set token allocations according to tokenomics
        let governance_participation = total_supply * 30 / 100; // 30%
        let marketplace_activity = total_supply * 20 / 100; // 20%
        
        
        
            let staking_rewards = total_supply * 25 / 100; // 25%
            let ecosystem_growth = total_supply * 15 / 100; // 15%
            let team_allocation = total_supply * 10 / 100; // 10%
            
            // Mint tokens to allocation accounts
            // Implementation will include minting logic and permission checks
            
            Ok(())
        }
    
    // Transfer GROW or FARM tokens
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64
    ) -> Result<()> {
        // Verify transfer amount is greater than zero
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Verify sender has sufficient tokens
        let sender_balance = ctx.accounts.sender_token_account.amount;
        require!(sender_balance >= amount, ErrorCode::InsufficientFunds);
        
        // Execute transfer
        let cpi_accounts = TokenTransfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Emit transfer event
        emit!(TokenTransferEvent {
            sender: ctx.accounts.sender.key(),
            recipient: ctx.accounts.recipient_token_account.owner,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Add account structures
#[derive(Accounts)]
pub struct InitializeCouncil<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 1 + 8 + 4 + 32 * 15 + 8 + 8 + 1
    )]
    pub council: Account<'info, Council>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NominateCouncilMember<'info> {
    #[account(mut)]
    pub nominee: Signer<'info>,
    #[account(
        constraint = council.active == false @ ErrorCode::CouncilAlreadyActive
    )]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = nominee,
        space = 8 + 32 + 32 + 100 + 1000 + 8 + 8 + 1
    )]
    pub nomination: Account<'info, Nomination>,
    #[account(
        constraint = nominee_token_account.owner == nominee.key() @ ErrorCode::InvalidTokenAccount,
        constraint = nominee_token_account.mint == farm_token_mint.key() @ ErrorCode::InvalidTokenMint
    )]
    pub nominee_token_account: Account<'info, TokenAccount>,
    pub farm_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteForCouncilMember<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub nomination: Account<'info, Nomination>,
    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + 1 + 32 + 32 + 32 + 8,
        seeds = [b"voter_record", council.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub voter_record: Account<'info, VoterRecord>,
    #[account(
        constraint = voter_token_account.owner == voter.key() @ ErrorCode::InvalidTokenAccount,
        constraint = voter_token_account.mint == farm_token_mint.key() @ ErrorCode::InvalidTokenMint
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    pub farm_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeCouncilElection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = council.authority == authority.key() @ ErrorCode::NotAuthorized
    )]
    pub council: Account<'info, Council>,
    pub nominations: Vec<Account<'info, Nomination>>,
}

#[derive(Accounts)]
pub struct CreateEmergencyAction<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(
        constraint = council.active @ ErrorCode::CouncilNotActive
    )]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 32 + 100 + 1000 + 1 + 4 + 1000 + 8 + 4 + 32 * 15 + 4 + 32 * 15 + 1 + 1
    )]
    pub emergency_action: Account<'info, EmergencyAction>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnEmergencyAction<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct ExecuteEmergencyAction<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct CancelEmergencyAction<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub emergency_action: Account<'info, EmergencyAction>,
}

#[derive(Accounts)]
pub struct InitializeCrisisManagement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 8 + 8 + 1 + 1
    )]
    pub crisis_manager: Account<'info, CrisisManager>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeclareCrisis<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut)]
    pub crisis_manager: Account<'info, CrisisManager>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 1000 + 8 + 8 + 1 + 1 + 1
    )]
    pub crisis_record: Account<'info, CrisisRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveCrisis<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
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
        init,
        payer = proposer,
        space = 8 + 32 + 100 + 1000 + 1 + 8 + 8 + 1 + 1 + 1 + 32 + 4 + 100 * 10 + 4 + 8 * 10 + 8
    )]
    pub proposal: Account<'info, Proposal>,
    pub dao: Account<'info, DAO>,
    #[account(
        constraint = proposer_token_account.owner == proposer.key() @ ErrorCode::InvalidTokenAccount,
        constraint = proposer_token_account.mint == farm_token_mint.key() @ ErrorCode::InvalidTokenMint
    )]
    pub proposer_token_account: Account<'info, TokenAccount>,
    pub farm_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + 1 + 32 + 32 + 1 + 8,
        seeds = [b"voter_record", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub voter_record: Account<'info, VoterRecord>,
    #[account(
        constraint = voter_token_account.owner == voter.key() @ ErrorCode::InvalidTokenAccount,
        constraint = voter_token_account.mint == farm_token_mint.key() @ ErrorCode::InvalidTokenMint
    )]
    pub voter_token_account: Account<'info, TokenAccount>,
    pub voter_token_data: Account<'info, TokenHolderData>,
    #[account(mut)]
    pub voter_token_data_mut: Account<'info, TokenHolderData>,
    pub farm_token_mint: Account<'info, Mint>,
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
        space = 8 + 32 + 100 + 1000 + 4 + 4 + 8 + 8 + 1 + 1
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
        init_if_needed,
        payer = signer,
        space = 8 + 1 + 32 + 32 + 8,
        seeds = [b"signature", petition.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub signature: Account<'info, PetitionSignature>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConvertPetitionToProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = petition.signature_count >= petition.required_signatures @ ErrorCode::InsufficientSignatures,
        constraint = !petition.converted_to_proposal @ ErrorCode::PetitionAlreadyConverted
    )]
    pub petition: Account<'info, Petition>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 100 + 1000 + 1 + 8 + 8 + 1 + 1 + 1 + 32 + 4 + 100 * 10 + 4 + 8 * 10 + 8
    )]
    pub proposal: Account<'info, Proposal>,
    pub dao: Account<'info, DAO>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
    )]
    pub token_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        mut,
        constraint = sender_token_account.owner == sender.key() @ ErrorCode::InvalidTokenAccount
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Add account data structures
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
    pub option_index: u8,
}

#[account]
pub struct EmergencyAction {
    pub proposer: Pubkey,
    pub council: Pubkey,
    pub title: String,
    pub description: String,
    pub action_type: u8,
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
    pub crisis_type: u8,
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
    pub total_votes_cast: u64,
}

#[account]
pub struct DAO {
    pub authority: Pubkey,
    pub governance_token: Pubkey,
    pub token_supply: u64,
    pub quorum_percentage: u8,
    pub proposal_threshold: u64,
    pub voting_period: i64,
    pub min_tokens_to_sign_petition: u64,
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
pub struct PetitionSignature {
    pub has_signed: bool,
    pub petition: Pubkey,
    pub signer: Pubkey,
    pub timestamp: i64,
}

#[account]
pub struct TokenHolderData {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub acquisition_time: i64,
    pub governance_participation_count: u32,
}

// Add event structures
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

#[event]
pub struct TokenTransferEvent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

// Add error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid council size (must be 3-15)")]
    InvalidCouncilSize,
    #[msg("Invalid quorum percentage (must be 51-100)")]
    InvalidQuorumPercentage,
    #[msg("Invalid term length (minimum 90 days)")]
    InvalidTermLength,
    #[msg("Nomination period is closed")]
    NominationPeriodClosed,
    #[msg("Insufficient tokens for nomination")]
    InsufficientTokensForNomination,
    #[msg("Election period is closed")]
    ElectionPeriodClosed,
    #[msg("Already voted in this election")]
    AlreadyVotedInElection,
    #[msg("No nominations found")]
    NoNominations,
    #[msg("Election period too short (minimum 14 days)")]
    ElectionPeriodTooShort,
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
    #[msg("Invalid crisis duration (1-30 days)")]
    InvalidCrisisDuration,
    #[msg("No crisis is currently active")]
    NoCrisisActive,
    #[msg("Invalid proposal type")]
    InvalidProposalType,
    #[msg("Invalid voting period (3-30 days)")]
    InvalidVotingPeriod,
    #[msg("Insufficient options (minimum 2)")]
    InsufficientOptions,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Proposal has been cancelled")]
    ProposalCancelled,
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("No votes cast")]
    NoVotesCast,
    #[msg("Insufficient approval percentage")]
    InsufficientApproval,
    #[msg("Not authorized to cancel proposal")]
    NotAuthorizedToCancelProposal,
    #[msg("Invalid signature requirement (100-1000)")]
    InvalidSignatureRequirement,
    #[msg("Petition has expired")]
    PetitionExpired,
    #[msg("Petition already converted to proposal")]
    PetitionAlreadyConverted,
    #[msg("Already signed this petition")]
    AlreadySigned,
    #[msg("Insufficient signatures")]
    InsufficientSignatures,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Council already active")]
    CouncilAlreadyActive,
}