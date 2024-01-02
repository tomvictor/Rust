
use ic_agent::{Agent, identity::AnonymousIdentity};
use candid::{Principal, Encode, Decode, CandidType, Nat};
use serde::Deserialize;


async fn connect_to_canister() {
    let agent = Agent::builder()
        .with_url("http://localhost:4943")
        .with_identity(AnonymousIdentity)
        .build()
        .unwrap();

    agent.fetch_root_key().await.unwrap();
    println!("agent created");

    let canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    println!("Canister id: {}", canister_id);


    let response = agent.query(&canister_id, "hello").call().await;
    // let response = agent.status();

    match response {
        Ok(resp) => {
            println!("response ok");
            // println!("response ok: {}", resp.to_string());
        }
        Err(error) => {
            println!("error occurred: {}", error);
            // error.to_string()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("calling canister...");
    connect_to_canister().await;
    Ok(())
}