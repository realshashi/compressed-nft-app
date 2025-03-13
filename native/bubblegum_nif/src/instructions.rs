use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    transaction::Transaction,
    system_program,
    signer::Signer,
};
use mpl_bubblegum::{
    state::{metaplex_adapter::*, leaf::Leaf},
    utils::get_tree_config_pda,
};
use crate::error::BubblegumError;
use log::{debug, info, warn};

/// Creates a tree configuration instruction.
pub fn create_tree_config_ix(
    max_depth: i32,
    max_buffer_size: u32,
    authority: Pubkey,
) -> Result<(Transaction, Signature), BubblegumError> {
    info!("Creating tree config with parameters: max_depth={}, max_buffer_size={}, authority={}", 
        max_depth, max_buffer_size, authority);

    let payer = Keypair::new();
    let merkle_tree = Keypair::new();

    debug!("Generated keypairs - payer: {}, merkle_tree: {}", 
        payer.pubkey(), merkle_tree.pubkey());

    let (tree_config, _) = get_tree_config_pda(&merkle_tree.pubkey());

    let instruction = mpl_bubblegum::instructions::CreateTree {
        tree_config,
        merkle_tree: merkle_tree.pubkey(),
        authority,
        max_depth: max_depth as u32,
        max_buffer_size,
        public: Some(true),
    }.instruction();

    debug!("Created tree instruction");

    let recent_blockhash = get_recent_blockhash()
        .map_err(|e| {
            warn!("Failed to get recent blockhash: {}", e);
            BubblegumError::NetworkError(e.to_string())
        })?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer, &merkle_tree],
        recent_blockhash,
    );

    debug!("Transaction created, sending");

    let signature = send_transaction(&transaction)
        .map_err(|e| {
            warn!("Failed to send transaction: {}", e);
            BubblegumError::TransactionError(e.to_string())
        })?;

    info!("Transaction sent successfully with signature: {}", signature);
    Ok((transaction, signature))
}

/// Mints a new compressed NFT.
pub fn mint_v1_ix(
    name: String,
    symbol: String,
    uri: String,
    collection: Pubkey,
    recipient: Pubkey,
) -> Result<(Transaction, Signature), BubblegumError> {
    info!("Minting NFT: name={}, symbol={}, uri={}", name, symbol, uri);

    let payer = Keypair::new();
    let (tree_config, _) = get_tree_config_pda(&collection);

    let metadata = MetadataArgs {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: None,
        collection: Some(Collection {
            verified: false,
            key: collection,
        }),
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators: vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100,
        }],
    };

    debug!("Created metadata arguments");

    let instruction = mpl_bubblegum::instructions::MintV1 {
        tree_config,
        leaf_owner: recipient,
        leaf_delegate: recipient,
        merkle_tree: collection,
        metadata: metadata.clone(),
        owner: recipient,
    }.instruction();

    debug!("Created mint instruction");

    let recent_blockhash = get_recent_blockhash()
        .map_err(|e| {
            warn!("Failed to get recent blockhash: {}", e);
            BubblegumError::NetworkError(e.to_string())
        })?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    debug!("Transaction created, sending");

    let signature = send_transaction(&transaction)
        .map_err(|e| {
            warn!("Failed to send transaction: {}", e);
            BubblegumError::TransactionError(e.to_string())
        })?;

    info!("Transaction sent successfully with signature: {}", signature);
    Ok((transaction, signature))
}

/// Transfers a compressed NFT to a new owner.
pub fn transfer_ix(
    asset_id: String,
    owner: Pubkey,
    recipient: Pubkey,
) -> Result<(Transaction, Signature), BubblegumError> {
    info!("Transferring asset {} from {} to {}", asset_id, owner, recipient);

    let payer = Keypair::new();
    let merkle_tree = Pubkey::new_unique();
    let (tree_config, _) = get_tree_config_pda(&merkle_tree);

    debug!("Generated keypair - payer: {}", payer.pubkey());
    debug!("Using merkle tree: {}", merkle_tree);

    let leaf = Leaf {
        owner,
        delegate: owner,
        nonce: 0,
        data_hash: [0; 32],
        creator_hash: [0; 32],
    };

    let instruction = mpl_bubblegum::instructions::TransferV1 {
        tree_config,
        leaf_owner: owner,
        leaf_delegate: owner,
        new_leaf_owner: recipient,
        merkle_tree,
        root: [0; 32],
        data_hash: leaf.data_hash,
        creator_hash: leaf.creator_hash,
        nonce: leaf.nonce,
        index: 0,
    }.instruction();

    debug!("Created transfer instruction");

    let recent_blockhash = get_recent_blockhash()
        .map_err(|e| {
            warn!("Failed to get recent blockhash: {}", e);
            BubblegumError::NetworkError(e.to_string())
        })?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    debug!("Transaction created, sending");

    let signature = send_transaction(&transaction)
        .map_err(|e| {
            warn!("Failed to send transaction: {}", e);
            BubblegumError::TransactionError(e.to_string())
        })?;

    info!("Transaction sent successfully with signature: {}", signature);
    Ok((transaction, signature))
}

fn get_recent_blockhash() -> Result<solana_sdk::hash::Hash, BubblegumError> {
    debug!("Getting recent blockhash (dummy for testing)");
    Ok(solana_sdk::hash::Hash::default())
}

fn send_transaction(_transaction: &Transaction) -> Result<Signature, BubblegumError> {
    debug!("Sending transaction (dummy for testing)");
    let signature = Signature::default();
    debug!("Generated dummy signature: {}", signature);
    Ok(signature)
}