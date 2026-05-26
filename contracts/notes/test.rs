// test.rs
#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address,
    Env,
};

use crate::{
    BarangayFlowContract,
    BarangayFlowContractClient,
};

fn create_contract(
    env: &Env,
) -> BarangayFlowContractClient<'static> {

    let contract_id = env.register_contract(
        None,
        BarangayFlowContract,
    );

    BarangayFlowContractClient::new(
        env,
        &contract_id,
    )
}

#[test]
fn test_happy_path() {

    let env = Env::default();

    env.mock_all_auths();

    let client = create_contract(&env);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.init(&admin);

    client.add_beneficiary(
        &admin,
        &user,
        &1000,
    );

    client.claim(&user);

    let status = client.get_status(&user);

    assert_eq!(status.claimed, true);
}

#[test]
#[should_panic(expected = "already claimed")]
fn test_double_claim() {

    let env = Env::default();

    env.mock_all_auths();

    let client = create_contract(&env);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.init(&admin);

    client.add_beneficiary(
        &admin,
        &user,
        &1000,
    );

    client.claim(&user);

    client.claim(&user);
}

#[test]
fn test_state_verification() {

    let env = Env::default();

    env.mock_all_auths();

    let client = create_contract(&env);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.init(&admin);

    client.add_beneficiary(
        &admin,
        &user,
        &500,
    );

    let status = client.get_status(&user);

    assert_eq!(status.amount, 500);
    assert_eq!(status.claimed, false);
}

#[test]
#[should_panic(expected = "unauthorized")]
fn test_unauthorized_admin() {

    let env = Env::default();

    env.mock_all_auths();

    let client = create_contract(&env);

    let admin = Address::generate(&env);
    let fake_admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.init(&admin);

    client.add_beneficiary(
        &fake_admin,
        &user,
        &100,
    );
}

#[test]
fn test_multiple_users() {

    let env = Env::default();

    env.mock_all_auths();

    let client = create_contract(&env);

    let admin = Address::generate(&env);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.init(&admin);

    client.add_beneficiary(
        &admin,
        &user1,
        &100,
    );

    client.add_beneficiary(
        &admin,
        &user2,
        &200,
    );

    let status1 = client.get_status(&user1);
    let status2 = client.get_status(&user2);

    assert_eq!(status1.amount, 100);
    assert_eq!(status2.amount, 200);
}