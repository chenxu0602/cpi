use anchor_client::solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
    pubkey::Pubkey,
};
use anchor_client::{Client, Cluster};
use std::rc::Rc;
use std::str::FromStr;

use cpi::accounts::SolTransfer;
use cpi::instruction::SolTransferOne;
use cpi::instruction::SolTransferTwo;
use cpi::instruction::SolTransferThree;

use solana_client::rpc_client::RpcClient;


#[test]
fn test_sol_transfer_one() {
    let home = std::env::var("HOME").expect("HOME env var not set");
    let path = format!("{}/.config/solana/id.json", home);
    let payer = Rc::new(read_keypair_file(&path).expect("Failed to read keypair"));

    let client = Client::new_with_options(
        Cluster::Devnet,
        payer.clone(),
        CommitmentConfig::processed(),
    );

    let program_id = Pubkey::from_str("6NCfKM3jcHudXu25GFoFNJgwsCpAsTdxsGpmrfNX9dbU").unwrap();
    let program = client.program(program_id).unwrap();

    let recipient = Keypair::new();
    let transfer_amount = 100_000;

    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    rpc.request_airdrop(&recipient.pubkey(), 1_000_000_000).expect("Airdrop failed");
    std::thread::sleep(std::time::Duration::from_secs(3));

    // let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());
    // rpc.request_airdrop(&recipient.pubkey(), 1_000_000_000).expect("Airdrop failed");
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // rpc.request_airdrop(&payer.pubkey(), 1_000_000_000).expect("Airdrop to payer failed");

    {
        let sig = program
            .request()
            .accounts(SolTransfer {
                sender: payer.pubkey(),
                receipient: recipient.pubkey(),  // match field name in program
                system_program: system_program::ID,
            })
            .args(SolTransferOne { amount: transfer_amount })
            .send()
            .expect("Failed to send transaction");

        println!("Transaction signature: {}", sig);
    }

    {
        let sig = program
            .request()
            .accounts(SolTransfer {
                sender: payer.pubkey(),
                receipient: recipient.pubkey(),  // match field name in program
                system_program: system_program::ID,
            })
            .args(SolTransferTwo { amount: transfer_amount })
            .send()
            .expect("Failed to send transaction");

        println!("Transaction signature: {}", sig);
    }

    {
        let sig = program
            .request()
            .accounts(SolTransfer {
                sender: payer.pubkey(),
                receipient: recipient.pubkey(),  // match field name in program
                system_program: system_program::ID,
            })
            .args(SolTransferThree { amount: transfer_amount })
            .send()
            .expect("Failed to send transaction");

        println!("Transaction signature: {}", sig);
    }
}
