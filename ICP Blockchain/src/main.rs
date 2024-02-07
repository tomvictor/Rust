use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::{Agent, identity::AnonymousIdentity};
use ic_agent::agent::UpdateBuilder;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
struct CanisterResult {
    canister_id: Principal,
}


#[derive(CandidType)]
struct Argument {
    principal: Principal,
}

async fn connect_to_canister() {
    let agent = Agent::builder()
        .with_url("https://ic0.app")
        .with_identity(AnonymousIdentity)
        .build()
        .unwrap();

    agent.fetch_root_key().await.unwrap();
    println!("agent created");

    let canister_id_principal = Principal::from_text("6qy4q-5aaaa-aaaah-adwma-cai").unwrap();
    println!("Canister id: {}", canister_id_principal);

    // let query_builder = QueryBuilder::new(&agent, canister_id_principal, String::from("getUserToken"));
    let query_builder = UpdateBuilder::new(&agent, canister_id_principal, String::from("getUserToken"));


    let args = Argument {
        principal: Principal::from_text("4cay5-ew3bs-vr6yl-7iffu-67doc-l655v-dluy7-qplpx-7pkio-er5rt-uqe").unwrap()
    };

    // let bytes = Encode!(&[(42, "arg1")], &(42u32, "arg2")).expect("Panic");
    // let input = Encode!(&[valid_principal]).expect("Panic");
    let input = Encode!(&args).expect("panic");
    // let input = Encode!(&[canister_id_principal]).expect("Panic");

    let response = query_builder
        .with_arg([&args.principal.clone()])
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