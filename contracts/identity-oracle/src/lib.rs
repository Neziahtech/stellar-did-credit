#![no_std]
#[allow(unused_imports)]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String, Vec};

/// Storage key variants for the identity-oracle contract.
#[contracttype]
pub enum DataKey {
    /// The contract administrator address.
    Admin,
    /// Whether the given address is a trusted credential issuer.
    TrustedIssuer(Address),
    /// The DID document hash anchored for the given subject address.
    DIDDocument(Address),
    /// The list of VC anchors associated with the given subject address.
    VCAnchors(Address),
}

/// An on-chain anchor record for a verifiable credential.
#[contracttype]
#[derive(Clone)]
pub struct VCRecord {
    /// SHA-256 hash of the off-chain verifiable credential JSON.
    pub vc_hash: BytesN<32>,
    /// Address of the issuer who anchored this credential.
    pub issuer: Address,
    /// Ledger timestamp (Unix seconds) when this credential was anchored.
    pub anchored_at: u64,
    /// Whether this credential has been revoked by the issuer.
    pub revoked: bool,
}

#[contract]
pub struct IdentityOracle;

#[contractimpl]
impl IdentityOracle {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn register_issuer(env: Env, admin: Address, issuer: Address) {
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        if admin != stored_admin {
            panic!("not authorized");
        }
        admin.require_auth();
        env.storage().persistent().set(&DataKey::TrustedIssuer(issuer), &true);
    }

    pub fn anchor_did(_env: Env, _subject: Address, _did_doc_cid: String) {
        // TODO: require subject auth, store CID, emit DIDAnchored event
        panic!("not yet implemented")
    }

    pub fn anchor_vc(
        _env: Env,
        _issuer: Address,
        _subject: Address,
        _vc_hash: BytesN<32>,
    ) {
        // TODO: require issuer auth, check issuer is trusted, store VCRecord
        panic!("not yet implemented")
    }

    pub fn is_verified(_env: Env, _subject: Address) -> bool {
        // TODO: return true if subject has >= 1 non-revoked VC
        panic!("not yet implemented")
    }

    pub fn get_vc_count(_env: Env, _subject: Address) -> u32 {
        panic!("not yet implemented")
    }

    pub fn verify_vc(_env: Env, _subject: Address, _vc_hash: BytesN<32>) -> bool {
        panic!("not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize_sets_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, IdentityOracle);
        let client = IdentityOracleClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        // verify by calling register_issuer as admin (should not panic)
        let issuer = Address::generate(&env);
        client.register_issuer(&admin, &issuer);
    }
}
