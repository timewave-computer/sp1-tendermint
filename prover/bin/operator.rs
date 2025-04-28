use program_types::TendermintOutput;
use sp1_sdk::utils::setup_logger;
use tendermint_operator::{util::TendermintRPCClient, TendermintProver};

/// Prove a transition from one height to another.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    setup_logger();
    // Instantiate a Tendermint prover based on the environment variable.
    let tendermint_rpc_client = TendermintRPCClient::default();
    let prover = TendermintProver::new();
    let trusted_block_height: u64 = 28220389;
    let target_block_height: u64 = 28220390;
    if trusted_block_height == 0 {
        panic!("No trusted height found on the contract. Something is wrong with the contract.");
    }

    let chain_latest_block_height = tendermint_rpc_client.get_latest_block_height().await;
    let (trusted_light_block, target_light_block) = tendermint_rpc_client
        .get_light_blocks(trusted_block_height, target_block_height)
        .await;
    // Generate a proof of the transition from the trusted block to the target block.
    let proof_data = prover.generate_tendermint_proof(&trusted_light_block, &target_light_block);
    let proof_out: TendermintOutput =
        serde_json::from_slice(&proof_data.public_values.to_vec()).unwrap();

    //println!("proof_out: {:?}", proof_out);
}
