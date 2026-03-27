#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Vec, Map, Symbol,
};

// ─────────────────────────────────────────────
//  Data Types
// ─────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct LoyaltyProgram {
    pub id: u64,
    pub creator: Address,
    pub name: String,
    pub description: String,
    pub brand_color: String,   // hex string e.g. "#FF6B35"
    pub stamp_threshold: u32,  // stamps needed for reward
    pub points_per_visit: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct LoyaltyNFT {
    pub token_id: u64,
    pub program_id: u64,
    pub owner: Address,
    pub stamps: u32,           // stamp card counter
    pub points: u64,           // accumulated points
    pub tier: u32,             // 0=None,1=Bronze,2=Silver,3=Gold
    pub visit_count: u32,      // total check-ins
    pub redeemed: bool,        // has claimed current reward
    pub metadata_uri: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct CheckIn {
    pub nft_id: u64,
    pub program_id: u64,
    pub timestamp: u64,
    pub points_earned: u32,
}

// Storage keys
#[contracttype]
pub enum DataKey {
    ProgramCount,
    TokenCount,
    Program(u64),
    NFT(u64),
    // owner -> list of token_ids
    OwnerTokens(Address),
    // program_id -> list of token_ids
    ProgramTokens(u64),
}

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────

#[contract]
pub struct LoyaltyContract;

#[contractimpl]
impl LoyaltyContract {

    // ── Programs ──────────────────────────────

    /// Anyone can create a loyalty program — fully permissionless
    pub fn create_program(
        env: Env,
        creator: Address,
        name: String,
        description: String,
        brand_color: String,
        stamp_threshold: u32,
        points_per_visit: u32,
    ) -> u64 {
        creator.require_auth();

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ProgramCount)
            .unwrap_or(0);
        let new_id = count + 1;

        let program = LoyaltyProgram {
            id: new_id,
            creator,
            name,
            description,
            brand_color,
            stamp_threshold,
            points_per_visit,
        };

        env.storage()
            .instance()
            .set(&DataKey::Program(new_id), &program);
        env.storage()
            .instance()
            .set(&DataKey::ProgramCount, &new_id);

        env.events().publish(
            (symbol_short!("prog_new"), new_id),
            new_id,
        );

        new_id
    }

    pub fn get_program(env: Env, program_id: u64) -> Option<LoyaltyProgram> {
        env.storage()
            .instance()
            .get(&DataKey::Program(program_id))
    }

    pub fn get_program_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ProgramCount)
            .unwrap_or(0)
    }

    // ── NFT Minting ───────────────────────────

    /// Anyone can mint a loyalty NFT for any program — permissionless
    pub fn mint_loyalty_nft(
        env: Env,
        recipient: Address,
        program_id: u64,
        metadata_uri: String,
    ) -> u64 {
        // program must exist
        let _program: LoyaltyProgram = env
            .storage()
            .instance()
            .get(&DataKey::Program(program_id))
            .unwrap_or_else(|| panic!("program not found"));

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TokenCount)
            .unwrap_or(0);
        let token_id = count + 1;

        let nft = LoyaltyNFT {
            token_id,
            program_id,
            owner: recipient.clone(),
            stamps: 0,
            points: 0,
            tier: 0,
            visit_count: 0,
            redeemed: false,
            metadata_uri,
        };

        env.storage()
            .instance()
            .set(&DataKey::NFT(token_id), &nft);
        env.storage()
            .instance()
            .set(&DataKey::TokenCount, &token_id);

        // update owner index
        let mut owner_tokens: Vec<u64> = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokens(recipient.clone()))
            .unwrap_or(Vec::new(&env));
        owner_tokens.push_back(token_id);
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokens(recipient), &owner_tokens);

        // update program index
        let mut prog_tokens: Vec<u64> = env
            .storage()
            .instance()
            .get(&DataKey::ProgramTokens(program_id))
            .unwrap_or(Vec::new(&env));
        prog_tokens.push_back(token_id);
        env.storage()
            .instance()
            .set(&DataKey::ProgramTokens(program_id), &prog_tokens);

        env.events().publish(
            (symbol_short!("nft_mint"), token_id),
            token_id,
        );

        token_id
    }

    // ── Check-In / Stamp ──────────────────────

    /// Check-in: adds stamp + points, auto-upgrades tier — permissionless
    pub fn check_in(
        env: Env,
        owner: Address,
        token_id: u64,
    ) -> LoyaltyNFT {
        owner.require_auth();

        let mut nft: LoyaltyNFT = env
            .storage()
            .instance()
            .get(&DataKey::NFT(token_id))
            .unwrap_or_else(|| panic!("nft not found"));

        assert!(nft.owner == owner, "not owner");

        let program: LoyaltyProgram = env
            .storage()
            .instance()
            .get(&DataKey::Program(nft.program_id))
            .unwrap_or_else(|| panic!("program not found"));

        nft.stamps += 1;
        nft.visit_count += 1;
        nft.points += program.points_per_visit as u64;

        // auto tier upgrade based on visits
        nft.tier = if nft.visit_count >= 20 {
            3 // Gold
        } else if nft.visit_count >= 10 {
            2 // Silver
        } else if nft.visit_count >= 5 {
            1 // Bronze
        } else {
            0
        };

        // reset stamps if threshold reached (mark redeemable)
        if nft.stamps >= program.stamp_threshold {
            nft.redeemed = false; // eligible for redemption
        }

        env.storage()
            .instance()
            .set(&DataKey::NFT(token_id), &nft);

        env.events().publish(
            (symbol_short!("checkin"), token_id),
            nft.visit_count,
        );

        nft
    }

    // ── Redemption ────────────────────────────

    /// Redeem reward — burns stamps, marks redeemed
    pub fn redeem_reward(
        env: Env,
        owner: Address,
        token_id: u64,
    ) -> LoyaltyNFT {
        owner.require_auth();

        let mut nft: LoyaltyNFT = env
            .storage()
            .instance()
            .get(&DataKey::NFT(token_id))
            .unwrap_or_else(|| panic!("nft not found"));

        assert!(nft.owner == owner, "not owner");

        let program: LoyaltyProgram = env
            .storage()
            .instance()
            .get(&DataKey::Program(nft.program_id))
            .unwrap_or_else(|| panic!("program not found"));

        assert!(
            nft.stamps >= program.stamp_threshold,
            "not enough stamps"
        );
        assert!(!nft.redeemed, "already redeemed");

        // burn stamps for reward
        nft.stamps -= program.stamp_threshold;
        nft.redeemed = true;

        env.storage()
            .instance()
            .set(&DataKey::NFT(token_id), &nft);

        env.events().publish(
            (symbol_short!("redeemed"), token_id),
            token_id,
        );

        nft
    }

    /// Burn NFT to claim discount — destroys the token
    pub fn burn_for_discount(
        env: Env,
        owner: Address,
        token_id: u64,
    ) {
        owner.require_auth();

        let nft: LoyaltyNFT = env
            .storage()
            .instance()
            .get(&DataKey::NFT(token_id))
            .unwrap_or_else(|| panic!("nft not found"));

        assert!(nft.owner == owner, "not owner");
        assert!(nft.points >= 100, "need at least 100 points to burn");

        // remove from storage
        env.storage()
            .instance()
            .remove(&DataKey::NFT(token_id));

        // remove from owner index
        let mut owner_tokens: Vec<u64> = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokens(owner.clone()))
            .unwrap_or(Vec::new(&env));
        // rebuild without this token
        let mut new_list: Vec<u64> = Vec::new(&env);
        for id in owner_tokens.iter() {
            if id != token_id {
                new_list.push_back(id);
            }
        }
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokens(owner), &new_list);

        env.events().publish(
            (symbol_short!("burned"), token_id),
            token_id,
        );
    }

    // ── Transfer ──────────────────────────────

    /// Transfer NFT to another address — permissionless peer to peer
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        token_id: u64,
    ) {
        from.require_auth();

        let mut nft: LoyaltyNFT = env
            .storage()
            .instance()
            .get(&DataKey::NFT(token_id))
            .unwrap_or_else(|| panic!("nft not found"));

        assert!(nft.owner == from, "not owner");

        nft.owner = to.clone();
        env.storage()
            .instance()
            .set(&DataKey::NFT(token_id), &nft);

        // update from index
        let mut from_tokens: Vec<u64> = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokens(from.clone()))
            .unwrap_or(Vec::new(&env));
        let mut new_from: Vec<u64> = Vec::new(&env);
        for id in from_tokens.iter() {
            if id != token_id {
                new_from.push_back(id);
            }
        }
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokens(from), &new_from);

        // update to index
        let mut to_tokens: Vec<u64> = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokens(to.clone()))
            .unwrap_or(Vec::new(&env));
        to_tokens.push_back(token_id);
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokens(to), &to_tokens);

        env.events().publish(
            (symbol_short!("transfer"), token_id),
            token_id,
        );
    }

    // ── Queries ───────────────────────────────

    pub fn get_nft(env: Env, token_id: u64) -> Option<LoyaltyNFT> {
        env.storage()
            .instance()
            .get(&DataKey::NFT(token_id))
    }

    pub fn get_owner_tokens(env: Env, owner: Address) -> Vec<u64> {
        env.storage()
            .instance()
            .get(&DataKey::OwnerTokens(owner))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_program_tokens(env: Env, program_id: u64) -> Vec<u64> {
        env.storage()
            .instance()
            .get(&DataKey::ProgramTokens(program_id))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_token_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TokenCount)
            .unwrap_or(0)
    }
}
