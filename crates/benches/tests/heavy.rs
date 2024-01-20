use starknet::core::types::FieldElement;

use benches::{spammer::spam_katana, BenchCall};

#[katana_runner::katana_test(1, true, "../../target/release/katana")]
async fn katana_heavy_prime_single() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 8000, true)
            .await;

    assert_eq!(result.steps, 881383);
    result.dump().await;
}

#[katana_runner::katana_test(100, true, "../../target/release/katana")]
async fn katana_heavy_prime_100() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 80000, true)
            .await;

    assert!(result.steps > 80000000);
    result.dump().await;
}

#[katana_runner::katana_test(1000, true, "../../target/release/katana")]
async fn katana_heavy_prime_1000_a() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 150000, true)
            .await;

    assert!(result.steps > 800000000);
    result.dump().await;
}

#[katana_runner::katana_test(1000, true, "../../target/release/katana")]
async fn katana_heavy_prime_1000_b() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 150000, true)
            .await;

    assert!(result.steps > 800000000);
    result.dump().await;
}

#[katana_runner::katana_test(1000, true, "../../target/release/katana")]
async fn katana_heavy_prime_1000_c() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 150000, true)
            .await;

    assert!(result.steps > 800000000);
    result.dump().await;
}

#[katana_runner::katana_test(2000, true, "../../target/release/katana")]
async fn katana_heavy_prime_2000_a() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 180000, true)
            .await;

    assert!(result.steps > 1600000000);
    result.dump().await;
}

#[katana_runner::katana_test(2000, true, "../../target/release/katana")]
async fn katana_heavy_prime_2000_b() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 180000, true)
            .await;

    assert!(result.steps > 1600000000);
    result.dump().await;
}

#[katana_runner::katana_test(2000, true, "../../target/release/katana")]
async fn katana_heavy_prime_2000_c() {
    let arg = FieldElement::from_dec_str(&(2111u64 * 2111u64).to_string()).unwrap();

    let result =
        spam_katana(runner, contract_address, vec![BenchCall("is_prime", vec![arg])], 180000, true)
            .await;

    assert!(result.steps > 1600000000);
    result.dump().await;
}

#[katana_runner::katana_test(2000, true, "../../target/release/katana")]
async fn katana_heavy_multicall_2000() {
    let arg = FieldElement::from_dec_str(&(109u64 * 109u64).to_string()).unwrap();
    let calls = (0..23).map(|_| BenchCall("is_prime", vec![arg.clone()])).collect();

    let result = spam_katana(runner, contract_address, calls, 0, true).await;

    // assert!(result.steps > 1600000000);
    result.dump().await;
}
