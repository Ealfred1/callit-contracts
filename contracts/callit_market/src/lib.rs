#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    token, Address, BytesN, Env, String, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    MarketAlreadyExists = 4,
    MarketNotFound = 5,
    MarketNotActive = 6,
    MarketNotClosed = 7,
    MarketAlreadyResolved = 8,
    MarketNotResolved = 9,
    ZeroAmount = 10,
    PositionAlreadyClaimed = 11,
    NoWinningShares = 12,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MarketStatus {
    Active = 0,
    Resolved = 1,
    Cancelled = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarketInfo {
    pub creator: Address,
    pub question: String,
    pub end_time: u64,
    pub status: MarketStatus,
    pub total_yes: i128,
    pub total_no: i128,
    pub winning_outcome: bool, // true = YES, false = NO
    pub fee_bps: u32,          // e.g. 100 = 1%
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionInfo {
    pub yes_amount: i128,
    pub no_amount: i128,
    pub claimed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TruthProof {
    pub resolved_at: u64,
    pub outcome_yes: bool,
    pub truth_digest: BytesN<32>, // SHA-256 hash of AI consensus report + citations
    pub evidence_uri: String,     // IPFS cid or HTTPS verification URL
}

#[contracttype]
pub enum DataKey {
    Admin,
    Oracle,
    Token,
    Market(Symbol),
    Position(Symbol, Address),
    Truth(Symbol),
}

#[contract]
pub struct CallitMarketContract;

#[contractimpl]
impl CallitMarketContract {
    /// Initialize contract with admin, oracle (AI Truth Consensus), and token address.
    pub fn initialize(
        env: Env,
        admin: Address,
        oracle: Address,
        token: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::Token, &token);

        Ok(())
    }

    /// Update admin address
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        new_admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
        Ok(())
    }

    /// Update oracle address
    pub fn set_oracle(env: Env, new_oracle: Address) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Oracle, &new_oracle);
        Ok(())
    }

    /// Create a binary prediction market.
    pub fn create_market(
        env: Env,
        creator: Address,
        market_id: Symbol,
        question: String,
        end_time: u64,
        fee_bps: u32,
    ) -> Result<(), Error> {
        creator.require_auth();

        let key = DataKey::Market(market_id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::MarketAlreadyExists);
        }

        let market = MarketInfo {
            creator,
            question,
            end_time,
            status: MarketStatus::Active,
            total_yes: 0,
            total_no: 0,
            winning_outcome: false,
            fee_bps,
        };

        env.storage().persistent().set(&key, &market);
        env.storage().persistent().extend_ttl(&key, 17280, 518400);

        Ok(())
    }

    /// Place a bet / buy YES or NO shares with the settlement token.
    pub fn buy_position(
        env: Env,
        buyer: Address,
        market_id: Symbol,
        outcome_yes: bool,
        amount: i128,
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::ZeroAmount);
        }
        buyer.require_auth();

        let m_key = DataKey::Market(market_id.clone());
        let mut market: MarketInfo = env.storage().persistent().get(&m_key).ok_or(Error::MarketNotFound)?;

        if market.status != MarketStatus::Active {
            return Err(Error::MarketNotActive);
        }

        let now = env.ledger().timestamp();
        if now >= market.end_time {
            return Err(Error::MarketNotActive);
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);

        // Transfer funds from buyer to contract escrow
        token_client.transfer(&buyer, &env.current_contract_address(), &amount);

        // Update market pools
        if outcome_yes {
            market.total_yes = market.total_yes.checked_add(amount).unwrap();
        } else {
            market.total_no = market.total_no.checked_add(amount).unwrap();
        }
        env.storage().persistent().set(&m_key, &market);

        // Update user position
        let p_key = DataKey::Position(market_id.clone(), buyer.clone());
        let mut pos: PositionInfo = env.storage().persistent().get(&p_key).unwrap_or(PositionInfo {
            yes_amount: 0,
            no_amount: 0,
            claimed: false,
        });

        if outcome_yes {
            pos.yes_amount = pos.yes_amount.checked_add(amount).unwrap();
        } else {
            pos.no_amount = pos.no_amount.checked_add(amount).unwrap();
        }
        env.storage().persistent().set(&p_key, &pos);
        env.storage().persistent().extend_ttl(&p_key, 17280, 518400);

        Ok(())
    }

    /// Resolve market with AI Truth Consensus proof.
    /// Can be called by designated Oracle or Admin.
    pub fn resolve_market(
        env: Env,
        caller: Address,
        market_id: Symbol,
        outcome_yes: bool,
        truth_digest: BytesN<32>,
        evidence_uri: String,
    ) -> Result<(), Error> {
        caller.require_auth();

        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        let oracle: Address = env.storage().instance().get(&DataKey::Oracle).ok_or(Error::NotInitialized)?;

        if caller != admin && caller != oracle {
            return Err(Error::NotAuthorized);
        }

        let m_key = DataKey::Market(market_id.clone());
        let mut market: MarketInfo = env.storage().persistent().get(&m_key).ok_or(Error::MarketNotFound)?;

        if market.status != MarketStatus::Active {
            return Err(Error::MarketAlreadyResolved);
        }

        market.status = MarketStatus::Resolved;
        market.winning_outcome = outcome_yes;
        env.storage().persistent().set(&m_key, &market);

        // Store immutable cryptographic truth proof
        let t_key = DataKey::Truth(market_id.clone());
        let proof = TruthProof {
            resolved_at: env.ledger().timestamp(),
            outcome_yes,
            truth_digest,
            evidence_uri,
        };
        env.storage().persistent().set(&t_key, &proof);
        env.storage().persistent().extend_ttl(&t_key, 17280, 518400);

        Ok(())
    }

    /// Claim winnings for a resolved market.
    pub fn claim(env: Env, user: Address, market_id: Symbol) -> Result<i128, Error> {
        user.require_auth();

        let m_key = DataKey::Market(market_id.clone());
        let market: MarketInfo = env.storage().persistent().get(&m_key).ok_or(Error::MarketNotFound)?;

        if market.status != MarketStatus::Resolved {
            return Err(Error::MarketNotResolved);
        }

        let p_key = DataKey::Position(market_id.clone(), user.clone());
        let mut pos: PositionInfo = env.storage().persistent().get(&p_key).ok_or(Error::NoWinningShares)?;

        if pos.claimed {
            return Err(Error::PositionAlreadyClaimed);
        }

        let total_pool = market.total_yes.checked_add(market.total_no).unwrap();
        let (user_shares, winning_pool) = if market.winning_outcome {
            (pos.yes_amount, market.total_yes)
        } else {
            (pos.no_amount, market.total_no)
        };

        if user_shares <= 0 || winning_pool <= 0 {
            return Err(Error::NoWinningShares);
        }

        // Payout = (user_shares * total_pool) / winning_pool
        let gross_payout = (user_shares.checked_mul(total_pool).unwrap()) / winning_pool;
        let fee = if market.fee_bps > 0 {
            (gross_payout.checked_mul(market.fee_bps as i128).unwrap()) / 10000
        } else {
            0
        };
        let net_payout = gross_payout.checked_sub(fee).unwrap();

        pos.claimed = true;
        env.storage().persistent().set(&p_key, &pos);

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &user, &net_payout);

        Ok(net_payout)
    }

    /// Read-only queries
    pub fn get_market(env: Env, market_id: Symbol) -> Result<MarketInfo, Error> {
        let m_key = DataKey::Market(market_id);
        env.storage().persistent().get(&m_key).ok_or(Error::MarketNotFound)
    }

    pub fn get_position(env: Env, market_id: Symbol, user: Address) -> PositionInfo {
        let p_key = DataKey::Position(market_id, user);
        env.storage().persistent().get(&p_key).unwrap_or(PositionInfo {
            yes_amount: 0,
            no_amount: 0,
            claimed: false,
        })
    }

    pub fn get_truth_proof(env: Env, market_id: Symbol) -> Result<TruthProof, Error> {
        let t_key = DataKey::Truth(market_id);
        env.storage().persistent().get(&t_key).ok_or(Error::MarketNotFound)
    }
}

#[cfg(test)]
mod test;
