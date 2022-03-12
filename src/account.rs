// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::commands::account::{
    balance_command, faucet_command, generate_address_command, list_addresses_command, list_transactions_command,
    send_command, sync_account_command, AccountCli, AccountCommands,
};
use clap::Parser;
use dialoguer::Input;
use iota_wallet::account::AccountHandle;

// loop on the account prompt
pub async fn account_prompt(account_handle: AccountHandle) {
    loop {
        let exit = account_prompt_internal(account_handle.clone()).await;
        if exit {
            break;
        }
    }
}

// loop on the account prompt
pub async fn account_prompt_internal(account_handle: AccountHandle) -> bool {
    let alias = {
        let account = account_handle.read().await;
        account.alias().clone()
    };
    let command: String = Input::new()
        .with_prompt(format!("Account `{}` command (h for help)", alias))
        .interact_text()
        .unwrap();

    match command.as_str() {
        "h" => {
            if let Err(err) = AccountCli::try_parse_from(vec!["Account:", "help"]) {
                let _ = err.print();
            }
        }
        "clear" => {
            // Clear console
            let _ = std::process::Command::new("clear").status();
        }
        _ => {
            // Prepend `Account: ` so the parsing will be correct
            let command = format!("Account: {}", command);
            let account_cli = match AccountCli::try_parse_from(command.split(' ')) {
                Ok(account_cli) => account_cli,
                Err(e) => {
                    let _ = e.print();
                    return false;
                }
            };
            if let Err(err) = match &account_cli.command {
                AccountCommands::Address => generate_address_command(&account_handle).await,
                AccountCommands::Balance => balance_command(&account_handle).await,
                AccountCommands::Exit => {
                    return true;
                }
                AccountCommands::Faucet { url } => faucet_command(&account_handle, url).await,
                AccountCommands::ListAddresses => list_addresses_command(&account_handle).await,
                AccountCommands::ListTransactions => list_transactions_command(&account_handle).await,
                AccountCommands::Send { address, amount } => {
                    send_command(&account_handle, address.to_owned(), *amount).await
                }
                AccountCommands::Sync => sync_account_command(&account_handle).await,
            } {
                println!("Error: {}", err);
            }
        }
    }

    false
}
