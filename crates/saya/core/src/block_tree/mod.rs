#[cfg(test)]
mod tests {
    use futures::future::join_all;
    use katana_primitives::genesis::constant::DEFAULT_FEE_TOKEN_ADDRESS;
    use katana_runner::KatanaRunner;
    use starknet::{
        accounts::{Account, Call},
        macros::{felt, selector},
    };
    use url::Url;

    use crate::{Saya, SayaConfig};

    #[tokio::test]
    async fn test_from() -> anyhow::Result<()> {
        let katanas = (0..8)
            .map(|seed| {
                KatanaRunner::new_with_seed("../../../target/release/katana", seed).unwrap()
            })
            .collect::<Vec<_>>();

        let accounts = katanas.iter().map(|k| k.account(0)).collect::<Vec<_>>();

        let calls = vec![Call {
            to: DEFAULT_FEE_TOKEN_ADDRESS.into(),
            selector: selector!("transfer"),
            calldata: vec![felt!("0x1"), felt!("0x99"), felt!("0x0")],
        }];

        let executions = accounts.iter().map(|a| a.execute(calls.clone())).collect::<Vec<_>>();
        let transactions = executions.iter().map(|e| e.send());

        join_all(transactions).await;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await; // wait for block to be mined

        let sayas = katanas
            .iter()
            .map(|k| {
                let saya_config = SayaConfig {
                    katana_rpc: Url::parse(&k.endpoint()).unwrap(),
                    start_block: 0,
                    data_availability: None,
                    prover: crate::prover::ProverIdentifier::Stone,
                    verifier: crate::verifier::VerifierIdentifier::HerodotusStarknetSepolia,
                };
                Saya::new(saya_config)
            })
            .collect::<Vec<_>>();

        let mut sayas = join_all(sayas).await.into_iter().collect::<Result<Vec<_>, _>>()?;

        let running = sayas.iter_mut().map(|s| s.start());

        join_all(running).await;

        Ok(())
    }
}
