use ic_agent::{Agent, identity::AnonymousIdentity};
use ic_agent::agent::QueryBuilder;
use candid::{Principal, Encode, Decode, CandidType, Nat};
use serde::Deserialize;



#[derive(CandidType, Deserialize)]
struct CanisterResult {
    canister_id: Principal,
}

async fn connect_to_canister() {
    let agent = Agent::builder()
        .with_url("http://localhost:4943")
        .with_identity(AnonymousIdentity)
        .build()
        .unwrap();

    agent.fetch_root_key().await.unwrap();
    println!("agent created");

    let canister_id_principal = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    println!("Canister id: {}", canister_id_principal);

    let query_builder = QueryBuilder::new(&agent, canister_id_principal, String::from("hello"));

    // let bytes = Encode!(&[(42, "arg1")], &(42u32, "arg2")).expect("Panic");
    let input = Encode!().expect("Panic");

    let response = query_builder
        .with_arg(input)
        .call()
        .await;

    match response {
        Ok(resp) => {
            println!("response ok");
            let result = Decode!(resp.as_slice(), String).expect("Panic: Failed to decode!");
            println!("{}", result)
        }
        Err(error) => {
            println!("error occurred: {}", error.to_string());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("calling canister...");
    connect_to_canister().await;
    Ok(())
}