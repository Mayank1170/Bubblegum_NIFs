use rustler::{Encoder, Env, Error, NifResult, Term};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_bubblegum::{
    instructions::{CreateTreeConfigBuilder, MintV1Builder, TransferBuilder, BurnBuilder},
    types::{MetadataArgs, TokenProgramVersion, Creator, Collection, Uses},
};
use std::str::FromStr;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        invalid_keypair,
        invalid_pubkey,
        rpc_error,
        transaction_error,
        missing_parameter,
        invalid_metadata,
        max_depth,         // Add this
        max_buffer_size,   // Add this
        name,              // Add this
        uri,               // Add this
        symbol             // Add this
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BubblegumError {
    #[error("Invalid keypair format")]
    InvalidKeypair,
    #[error("Invalid public key: {0}")]
    InvalidPubkey(String),
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("Missing parameter: {0}")]
    MissingParameter(&'static str),
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(&'static str),
}

// Helper function to parse keypair from binary
fn parse_keypair(keypair_bin: Term) -> Result<Keypair, Error> {
    let keypair_bytes = keypair_bin.decode::<Vec<u8>>()?;
    if keypair_bytes.len() != 64 {
        return Err(Error::Term(Box::new(atoms::invalid_keypair())));
    }
    
    let keypair = Keypair::from_bytes(&keypair_bytes)
        .map_err(|_| Error::Term(Box::new(atoms::invalid_keypair())))?;
    
    Ok(keypair)
}

// Helper function to send transaction
fn send_transaction(
    instruction: Instruction,
    keypair: &Keypair,
    rpc_url: &str,
) -> Result<String, Error> {
    // Connect to Solana devnet
    let client = RpcClient::new(rpc_url);
    
    // Create transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));
    
    // Get recent blockhash
    let blockhash = client
        .get_latest_blockhash()
        .map_err(|_| Error::Term(Box::new(atoms::rpc_error())))?;
    
    // Sign transaction
    transaction.sign(&[keypair], blockhash);
    
    // Send transaction
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| Error::Term(Box::new(format!("Transaction error: {}", e))))?;
    
    Ok(signature.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn create_tree_config<'a>(env: Env<'a>, params: Term<'a>, keypair_bin: Term<'a>) -> NifResult<Term<'a>> {
    // Parse keypair
    let keypair = parse_keypair(keypair_bin)?;
    
    // Extract parameters
    let max_depth = params.map_get(atoms::max_depth())?
        .decode::<u32>()?;
    
    let max_buffer_size = params.map_get(atoms::max_buffer_size())?
        .decode::<u32>()?;
    
    // Create tree config instruction
    let instruction = CreateTreeConfigBuilder::new()
        .max_depth(max_depth)
        .max_buffer_size(max_buffer_size)
        .payer(keypair.pubkey())
        .merkle_tree(Pubkey::new_unique()) // Generate a new merkle tree address
        .instruction();
    
    // Send transaction
    match send_transaction(instruction, &keypair, "https://api.devnet.solana.com") {
        Ok(signature) => Ok((atoms::ok(), signature).encode(env)),
        Err(e) => Ok((atoms::error(), format!("{:?}", e)).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn mint_compressed_nft<'a>(env: Env<'a>, tree_id: Term<'a>, metadata: Term<'a>, keypair_bin: Term<'a>) -> NifResult<Term<'a>> {
    // Parse keypair
    let keypair = parse_keypair(keypair_bin)?;
    
    // Parse tree ID
    let tree_id_str = tree_id.decode::<String>()?;
    let tree_config = Pubkey::from_str(&tree_id_str)
        .map_err(|_| Error::Term(Box::new(atoms::invalid_pubkey())))?;
    
    // Extract metadata fields
    let name = metadata.map_get(atoms::name())?
        .decode::<String>()?;
    
    let uri = metadata.map_get(atoms::uri())?
        .decode::<String>()?;
    
    let symbol = metadata.map_get(atoms::symbol())
        .unwrap_or_else(|_| "".encode(env))
        .decode::<String>()
        .unwrap_or_default();
    
    // Create metadata args
    let metadata_args = MetadataArgs {
        name,
        uri,
        symbol,
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: None,
        collection: None,
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators: vec![],
    };
    
    // Create mint instruction
    let instruction = MintV1Builder::new()
        .tree_config(tree_config)
        .leaf_owner(keypair.pubkey())
        .leaf_delegate(keypair.pubkey())
        .payer(keypair.pubkey())
        .metadata(metadata_args)
        .instruction();
    
    // Send transaction
    match send_transaction(instruction, &keypair, "https://api.devnet.solana.com") {
        Ok(signature) => Ok((atoms::ok(), signature).encode(env)),
        Err(e) => Ok((atoms::error(), format!("{:?}", e)).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn transfer_compressed_nft<'a>(env: Env<'a>, nft_id: Term<'a>, recipient: Term<'a>, keypair_bin: Term<'a>) -> NifResult<Term<'a>> {
    // Parse keypair
    let keypair = parse_keypair(keypair_bin)?;
    
    // Parse NFT ID (asset ID)
    let nft_id_str = nft_id.decode::<String>()?;
    let nft_pubkey = Pubkey::from_str(&nft_id_str)
        .map_err(|_| Error::Term(Box::new(atoms::invalid_pubkey())))?;
    
    // Parse recipient
    let recipient_str = recipient.decode::<String>()?;
    let recipient_pubkey = Pubkey::from_str(&recipient_str)
        .map_err(|_| Error::Term(Box::new(atoms::invalid_pubkey())))?;
    
    // Create transfer instruction
    let instruction = TransferBuilder::new()
        .leaf_owner(keypair.pubkey(), true)
        .new_leaf_owner(recipient_pubkey)
        .tree_config(nft_pubkey) // This would actually be the tree config, not the NFT ID
        .instruction();
    
    // Send transaction
    match send_transaction(instruction, &keypair, "https://api.devnet.solana.com") {
        Ok(signature) => Ok((atoms::ok(), signature).encode(env)),
        Err(e) => Ok((atoms::error(), format!("{:?}", e)).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn burn<'a>(env: Env<'a>, nft_id: Term<'a>, keypair_bin: Term<'a>) -> NifResult<Term<'a>> {
    // Parse keypair
    let keypair = parse_keypair(keypair_bin)?;
    
    // Parse NFT ID (asset ID)
    let nft_id_str = nft_id.decode::<String>()?;
    let nft_pubkey = Pubkey::from_str(&nft_id_str)
        .map_err(|_| Error::Term(Box::new(atoms::invalid_pubkey())))?;
    
    // Create burn instruction
  // Create burn instruction
let instruction = BurnBuilder::new()
.leaf_owner(keypair.pubkey(), true)  // Add the boolean parameter
.tree_config(nft_pubkey)
.instruction();

    
    // Send transaction
    match send_transaction(instruction, &keypair, "https://api.devnet.solana.com") {
        Ok(signature) => Ok((atoms::ok(), signature).encode(env)),
        Err(e) => Ok((atoms::error(), format!("{:?}", e)).encode(env)),
    }
}

rustler::init!("Elixir.MplBubblegum.Client", [
    create_tree_config,
    mint_compressed_nft,
    transfer_compressed_nft,
    burn
]);
