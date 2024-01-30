use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Stmt};

const DEFAULT_CONTRACT: &str = "contracts/Scarb.toml";
const DEFAULT_SCRIPT: &str = "contracts/scripts/auth.sh";

fn parse_metadata(metadata: String) -> (u16, String, bool, String, String) {
    if metadata.is_empty() {
        return (1, "katana".into(), false, DEFAULT_CONTRACT.into(), DEFAULT_SCRIPT.into());
    }
    let args = metadata.split(',').collect::<Vec<&str>>();
    let n_accounts = if !args.is_empty() { args[0].parse::<u16>().unwrap() } else { 1 };

    let with_blocks = if args.len() >= 2 { args[1].trim().parse::<bool>().unwrap() } else { false };

    let executable = if args.len() >= 3 { args[2].trim() } else { "katana" };
    let executable = executable.replace('"', "");

    let contract = if args.len() >= 4 { args[3].trim() } else { DEFAULT_CONTRACT };
    let script = if args.len() >= 5 { args[3].trim() } else { DEFAULT_SCRIPT };

    // plus one as the first account is used for deployment
    (n_accounts + 1, executable, with_blocks, contract.into(), script.into())
}

#[proc_macro_attribute]
pub fn katana_test(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut test_function = parse_macro_input!(input as syn::ItemFn);
    let function_name = test_function.sig.ident.to_string();

    let (n_accounts, executable, with_blocks, contract, script) =
        parse_metadata(metadata.to_string());

    let header: Stmt = parse_quote! {
        let runner =
            katana_runner::KatanaRunner::new_with_args(#executable, #function_name, #n_accounts, #with_blocks)
                .expect("failed to start katana");
    };
    let header_deploy: Stmt = parse_quote! {
        let contract_address = runner.deploy(#contract, #script).await
            .expect("Failed to deploy");
    };

    if !contract.is_empty() {
        test_function.block.stmts.insert(0, header_deploy);
    }
    test_function.block.stmts.insert(0, header);

    if test_function.sig.asyncness.is_none() {
        TokenStream::from(quote! {
            #[test]
            #test_function
        })
    } else {
        TokenStream::from(quote! {
            #[tokio::test]
            #test_function
        })
    }
}

#[proc_macro] // Needed because the main macro doesn't work with proptest
pub fn runner(metadata: TokenStream) -> TokenStream {
    let metadata = metadata.to_string();
    let mut args = metadata.split(',').collect::<Vec<&str>>();
    let function_name = args.remove(0);

    let (n_accounts, executable, with_blocks, _contract, _script) = parse_metadata(args.join(","));
    TokenStream::from(quote! {
            static RUNNER: tokio::sync::OnceCell<(katana_runner::KatanaRunner, starknet::core::types::FieldElement)> = tokio::sync::OnceCell::const_new();
            let (runner, contract_address) = {
                let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
                let _rt = runtime.enter();

                futures::executor::block_on(
                    RUNNER
                    .get_or_init(|| async {
                        let runner =
                            katana_runner::KatanaRunner::new_with_args(#executable, #function_name, #n_accounts, #with_blocks)
                                .expect("failed to start katana");

                        let contract = if std::path::Path::new("contracts/Scarb.toml").exists() {
                            runner.deploy("contracts/Scarb.toml", "contracts/scripts/auth.sh").await
                                .expect("Failed to deploy")
                        } else if std::path::Path::new("../contracts/Scarb.toml").exists() {
                            runner.deploy("../contracts/Scarb.toml", "../contracts/scripts/auth.sh").await
                                .expect("Failed to deploy")
                        } else {
                            panic!("Contract not found");
                        };

                        (runner, contract)
                    })
                )
            };
    })
}
