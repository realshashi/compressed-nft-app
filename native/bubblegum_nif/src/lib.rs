use rustler::{Encoder, NifResult};
use rustler::env::Env as RustlerEnv;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
};
use std::str::FromStr;
use std::sync::Mutex;
use ttl_cache::TtlCache;
use std::time::Duration;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use env_logger::{Builder, Env as LoggerEnv};

mod error;
mod instructions;

use instructions::{create_tree_config_ix, mint_v1_ix, transfer_ix};
use error::BubblegumError;

// Initialize logger with custom configuration
fn init_logger() {
    let env = LoggerEnv::default()
        .filter_or("RUST_LOG", "debug")
        .write_style_or("RUST_LOG_STYLE", "always");

    Builder::from_env(env)
        .format_timestamp(None)
        .format_target(false)
        .try_init()
        .ok();

    debug!("Logger initialized");
}

// Define cache for storing frequently accessed data
lazy_static! {
    static ref CACHE: Mutex<TtlCache<String, String>> = {
        let cache = TtlCache::new(100);
        Mutex::new(cache)
    };
}

#[rustler::nif]
fn create_tree_config(
    max_depth: i32,
    max_buffer_size: u32,
    authority: String,
) -> NifResult<String> {
    init_logger();
    info!("Creating tree config with max_depth: {}, max_buffer_size: {}", max_depth, max_buffer_size);
    debug!("Authority pubkey string: {}", authority);

    let authority_pubkey = Pubkey::from_str(&authority)
        .map_err(|e| {
            warn!("Invalid authority pubkey: {}", e);
            BubblegumError::InvalidPublicKey(e.to_string())
        })?;

    let (_transaction, signature) = create_tree_config_ix(
        max_depth,
        max_buffer_size,
        authority_pubkey,
    ).map_err(|e| {
        warn!("Failed to create tree config: {}", e);
        BubblegumError::InvalidInstruction(e.to_string())
    })?;

    info!("Tree config created with signature: {}", signature);
    Ok(signature.to_string())
}

#[rustler::nif]
fn mint_v1(
    name: String,
    symbol: String,
    uri: String,
    collection: String,
    recipient: String,
) -> NifResult<String> {
    init_logger();
    info!("Minting NFT with name: {}, symbol: {}", name, symbol);
    debug!("Collection pubkey string: {}", collection);
    debug!("Recipient pubkey string: {}", recipient);

    let collection_pubkey = Pubkey::from_str(&collection)
        .map_err(|e| {
            warn!("Invalid collection pubkey: {}", e);
            BubblegumError::InvalidPublicKey(e.to_string())
        })?;

    let recipient_pubkey = Pubkey::from_str(&recipient)
        .map_err(|e| {
            warn!("Invalid recipient pubkey: {}", e);
            BubblegumError::InvalidPublicKey(e.to_string())
        })?;

    let (_transaction, signature) = mint_v1_ix(
        name,
        symbol,
        uri,
        collection_pubkey,
        recipient_pubkey,
    ).map_err(|e| {
        warn!("Failed to mint NFT: {}", e);
        BubblegumError::InvalidInstruction(e.to_string())
    })?;

    info!("NFT minted with signature: {}", signature);
    Ok(signature.to_string())
}

#[rustler::nif]
fn transfer(
    asset_id: String,
    owner: String,
    recipient: String,
) -> NifResult<String> {
    init_logger();
    info!("Transferring asset {} from {} to {}", asset_id, owner, recipient);
    debug!("Owner pubkey string: {}", owner);
    debug!("Recipient pubkey string: {}", recipient);

    let owner_pubkey = Pubkey::from_str(&owner)
        .map_err(|e| {
            warn!("Invalid owner pubkey: {}", e);
            BubblegumError::InvalidPublicKey(e.to_string())
        })?;

    let recipient_pubkey = Pubkey::from_str(&recipient)
        .map_err(|e| {
            warn!("Invalid recipient pubkey: {}", e);
            BubblegumError::InvalidPublicKey(e.to_string())
        })?;

    let (_transaction, signature) = transfer_ix(
        asset_id,
        owner_pubkey,
        recipient_pubkey,
    ).map_err(|e| {
        warn!("Failed to transfer NFT: {}", e);
        BubblegumError::InvalidInstruction(e.to_string())
    })?;

    info!("Asset transferred with signature: {}", signature);
    Ok(signature.to_string())
}

#[rustler::nif]
fn clear_cache() -> NifResult<String> {
    init_logger();
    if let Ok(mut cache) = CACHE.lock() {
        cache.clear();
        info!("Cache cleared successfully");
        Ok("Cache cleared successfully".to_string())
    } else {
        warn!("Failed to clear cache");
        Err(BubblegumError::CacheError("Failed to clear cache".to_string()).into())
    }
}

rustler::init!("Elixir.Bubblegum", [
    create_tree_config,
    mint_v1,
    transfer,
    clear_cache
]);