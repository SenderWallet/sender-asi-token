#[macro_use]
extern crate static_assertions;

use near_contract_standards::fungible_token::{
    events::{FtBurn, FtMint},
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider},
    FungibleToken,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    env,
    log,
    json_types::{U128, U64},
    near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue,
};
use sweat_model::{SweatApi};


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
}

#[near_bindgen]
impl SweatApi for Contract {
    #[init]
    fn new(owner: AccountId, total_supply: U128) -> Self {
        let mut contract = Contract {
            token: FungibleToken::new(b"t".to_vec(), None),
        };
        let amount: Balance = total_supply.into();
        contract.token.internal_register_account(&owner);
        contract.token.internal_deposit(&owner , amount);
        log!("Deposit {} token to {}", amount, owner);
        contract


    }

    fn burn(&mut self, amount: &U128) {
        self.token.internal_withdraw(&env::predecessor_account_id(), amount.0);
        FtBurn {
            amount,
            owner_id: &env::predecessor_account_id(),
            memo: None,
        }
        .emit();
    }
}


near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

/// Taken from contract standards but modified to default if account isn't initialized
/// rather than panicking:
/// <https://github.com/near/near-sdk-rs/blob/6596dc311036fe51d94358ac8f6497ef6e5a7cfc/near-contract-standards/src/fungible_token/core_impl.rs#L105>
fn internal_deposit(token: &mut FungibleToken, account_id: &AccountId, amount: Balance) {
    let balance = token.accounts.get(account_id).unwrap_or_default();
    let new_balance = balance
        .checked_add(amount)
        .unwrap_or_else(|| env::panic_str("Balance overflow"));
    token.accounts.insert(account_id, &new_balance);
    token.total_supply = token
        .total_supply
        .checked_add(amount)
        .unwrap_or_else(|| env::panic_str("Total supply overflow"));
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let data_url = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iODBweCIgaGVpZ2h0PSI4MHB4IiB2aWV3Qm94PSIwIDAgODAgODAiIHZlcnNpb249IjEuMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayI+CiAgICA8dGl0bGU+aWNvbi1Sb3VuZGVkPC90aXRsZT4KICAgIDxnIGlkPSLmlrDlrpjnvZEiIHN0cm9rZT0ibm9uZSIgc3Ryb2tlLXdpZHRoPSIxIiBmaWxsPSJub25lIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiPgogICAgICAgIDxnIGlkPSJQcmVzcy1LaXQiIHRyYW5zZm9ybT0idHJhbnNsYXRlKC01MTkuMDAwMDAwLCAtMjY0LjAwMDAwMCkiPgogICAgICAgICAgICA8ZyBpZD0i57yW57uEIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxNTIuMDAwMDAwLCAxMTIuMDAwMDAwKSI+CiAgICAgICAgICAgICAgICA8ZyBpZD0iTGlzdHMiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDAuMDAwMDAwLCAxMjAuMDAwMDAwKSI+CiAgICAgICAgICAgICAgICAgICAgPGcgaWQ9IlAtMiIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMjgyLjAwMDAwMCwgMC4wMDAwMDApIj4KICAgICAgICAgICAgICAgICAgICAgICAgPGcgaWQ9Imljb24tUm91bmRlZCIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoODUuMDAwMDAwLCAzMi4wMDAwMDApIj4KICAgICAgICAgICAgICAgICAgICAgICAgICAgIDxwYXRoIGQ9Ik03OS4xOTcyMDUxLDEzLjM3NzA0MDYgQzc4LjU3MzI5NzMsMTAuMzE5NzExOCA3Ny4wNTA2NjUxLDcuNTE2NzAzNzQgNzQuODIyNDIyOSw1LjMyNTYxMzEyIEM3Mi42MDM5MTA3LDMuMTQ2NDUwMDQgNjkuODI5NjAwNiwxLjYxMzU1MDYzIDY2Ljc5OTkzNCwwLjg5Mjc4OTgzNiBDNjIuODU1ODcxLDAuMjcyNzY1MjYxIDU4Ljg2NzM5MTgsLTAuMDIzNzUwNjIzMSA1NC44NzQ2Nzg5LDAuMDA2MjIxNDc2MTggTDI1LjI1MjI3ODYsMC4wMDYyMjE0NzYxOCBDMjEuMjY2OTE4OCwtMC4wNDYxNTM4MDg1IDE3LjI4NDE1ODcsMC4yMzIyMzY4MTEgMTMuMzQ1MjIwNywwLjgzODUwNTEwOSBDMTAuMjk0NjA4NiwxLjUxODc5NjY5IDcuNTAzOTU4MDksMy4wNTcyNzExNiA1LjMwNDMxMTY3LDUuMjcxMzM1OCBDMy4xMTk4NzY3Miw3LjQ3MTYxNDU1IDEuNTkyNjkxNTEsMTAuMjM0MzQ1OCAwLjg5MzYxNzY1NiwxMy4yNTA0MzU1IEMwLjI3MDk3MTc2OSwxNy4yMDYxNDI1IC0wLjAyNjQ4Mjk2NjEsMjEuMjA1OTc2MyAwLjAwNDIxNzc1NDkxLDI1LjIxMDAzMDIgTDAuMDA0MjE3NzU0OTEsNTQuNzM4MDQ5IEMtMC4wMzg3NDU3MjU2LDU4LjcxMTA4MDkgMC4yNDY1ODQ0MDEsNjIuNjgwODU1MiAwLjg1NzMxMjE2NCw2Ni42MDcxNjkxIEMxLjQ4NTU5NDc2LDY5LjY3MTk3NTggMy4wMTQ2ODE0LDcyLjQ4MTA1NSA1LjI0OTg2MDg2LDc0LjY3Njg3NjYgQzcuNDY2Mzc1MDcsNzYuODUyODU2MSAxMC4yMzIyOTIxLDc4LjM5MTM2NzYgMTMuMjU0NDU3LDc5LjEyNzMwNjIgQzE3LjIxMTk2MzcsNzkuNzM1MTU4OSAyMS4yMTE4ODEzLDgwLjAyNjEyODYgMjUuMjE1OTU4Miw3OS45OTU3NzMgTDU0Ljc0NzY2OTEsNzkuOTk1NzczIEM1OC43MzMzMjU5LDgwLjAzODcxNTEgNjIuNzE1ODYzMSw3OS43NTQ0MDg4IDY2LjY1NDcyNjksNzkuMTQ1ODE1NyBDNjkuNzEzNTA5Miw3OC40ODY4NzY3IDcyLjUxMDAyNzQsNzYuOTQ1NDAzNyA3NC42OTU0MTMxLDc0LjcxMzE1NTMgQzc2Ljg5MDk3NDQsNzIuNTIwODg3NSA3OC40MjAyOTEzLDY5Ljc1NDgyNDUgNzkuMTA2NTg5OSw2Ni43MzM4NDgzIEM3OS43MjgyNjk1LDYyLjc5MDEzNTQgODAuMDI1MzY4NCw1OC44MDI0NDM5IDc5Ljk5NTY1ODYsNTQuODEwNDU4MyBMNzkuOTk1NjU4NiwyNS4yNDYyMzQ4IEM4MC4wMzc5OTUyLDIxLjI3NTIwMTkgNzkuNzcwNjA2MSwxNy4zMDY5MDg0IDc5LjE5NzIwNTEsMTMuMzc3MDQwNiBaIiBpZD0iYmFzZSIgZmlsbD0iIzJFNzlERiIgZmlsbC1ydWxlPSJub256ZXJvIj48L3BhdGg+CiAgICAgICAgICAgICAgICAgICAgICAgICAgICA8cGF0aCBkPSJNNDguNDEwMDUzNiwzNS40MjY1OTU4IEM0OS4yMDI3NTg1LDM1LjcxNjEzMjcgNDkuOTMwNzQ0NCwzNi4wOTQ0MzIxIDUwLjU3NzQyNzQsMzYuNTU5NDAzMSBDNTEuNDM3OTc2NSwzNy4xNzgwNiA1Mi4xNTQ0MjYsMzcuOTUwODU5MyA1Mi42ODU0OTE3LDM4Ljg3NDI4NzkgQzUyLjk1ODU5NjQsMzkuMzQ5MTY4MSA1My4xNzI5MTg4LDM5Ljg0NjQxNTcgNTMuMzMwMjMyLDQwLjM2MTM3MTIgQzUzLjQ4OTMzMDMsNDAuODgwMTM0NiA1My41OTExMTU3LDQxLjQxNjc4MTggNTMuNjM3NjY0Nyw0MS45NjY0Nzc1IEM1My45NjY4MzEsNDUuODMxMzg3MyA1MS41ODIxMDQ0LDUwLjMyNTc3NzYgNDcuMzM4MjM3OCw1My42NjQ0Nzc5IEw0Ny4xNDY2NDQ4LDUzLjgxMzQ2IEM0Ni40MDg1NzQ0LDU0LjM4MDQ3NDMgNDUuNjE1ODQ5NSw1NC45MTI5ODQxIDQ0Ljc3MzEwMDcsNTUuNDAxNDQ1MiBMMjUuMzM3MDUzLDY2LjY2NjY2NjcgQzI0Ljc2NjcwNjIsNjIuODc4NTkyIDI1LjgxODgwMTQsNTguOTg3OTE2MiAyOC4yMjQxNDkzLDU1Ljk4ODQzNjQgQzI5LjI2Mzk5OTMsNTQuNjkxMjIxOSAzMC41NTY4ODY2LDUzLjU2MDkwNTUgMzIuMDgxMTgxMyw1Mi42Nzc0MTc0IEw0My40MzQ5MTUxLDQ2LjA5Njc0MTcgQzQ0Ljk4MjI4MjIsNDUuMTk5ODgwOCA0NS45NjU5MzIsNDMuNzAwNzc2NSA0Ni4yNTgyNDE5LDQyLjA3MDA5MTEgQzQ2LjQ5MDg3NzEsNDAuNzczNDI4OCA0Ni4yODY0NzA0LDM5LjM5MzkxOTQgNDUuNTgwOTkzNiwzOC4xNjcyMjEzIEM0NS4wMTM3MjI3LDM3LjE4MDgzODYgNDQuMTk5MDczMiwzNi40MjY2MTkgNDMuMjU5NDk2OCwzNS45MzczMDE5IEM0Mi4xOTE1NTgyLDM1LjM4MDMzMzkgNDAuOTYxODY2OSwzNS4xNjU3MDYzIDM5Ljc1MDk3MzYsMzUuMzQxMzE0IEMzOS44MTkzNzczLDM1LjMwODkzMDggMzkuODg3OTk5NSwzNS4yNzc2MzE3IDM5Ljk1Njg0MDQsMzUuMjQ3NDE2NCBDNDAuOTg3MjIzMSwzNC45OTE2MDY4IDQyLjAwNTE5NywzNC44Mjg5NDU5IDQyLjk5MzEzNzksMzQuNzU4MzQ5NCBDNDQuOTY0OTg0LDM0LjYxNzg4MTMgNDYuODE3Mzg5OSwzNC44NDQwNTg5IDQ4LjQxMDA1MzYsMzUuNDI2NTk1OCBaIE01NC42NjI5NDcsMTMuMzMzMzMzMyBDNTUuMjMzMjkzOCwxNy4xMjE0MDggNTQuMTgxMTk4NiwyMS4wMTIwODM4IDUxLjc3NTg1MDcsMjQuMDExNTYzNiBDNTAuNzM2MDAwNywyNS4zMDg3NzgxIDQ5LjQ0MzExMzQsMjYuNDM5MDk0NSA0Ny45MTg4MTg3LDI3LjMyMjU4MjYgTDM2LjU2NTA4NDksMzMuOTAzMjU4MyBDMzUuMDE3NzE3OCwzNC44MDAxMTkyIDM0LjAzNDA2OCwzNi4yOTkyMjM1IDMzLjc0MTc1ODEsMzcuOTI5OTA4OSBDMzMuNTA5MTIyOSwzOS4yMjY1NzEyIDMzLjcxMzUyOTYsNDAuNjA2MDgwNiAzNC40MTkwMDY0LDQxLjgzMjc3ODcgQzM0Ljk4NjI3NzMsNDIuODE5MTYxNCAzNS44MDA5MjY4LDQzLjU3MzM4MSAzNi43NDA1MDMyLDQ0LjA2MjY5ODEgQzM3LjgwODQ0MTgsNDQuNjE5NjY2MSAzOS4wMzgxMzMxLDQ0LjgzNDI5MzcgNDAuMjQ5MDI2NCw0NC42NTg2ODYgQzQwLjE4MDYyMjcsNDQuNjkxMDY5MiA0MC4xMTIwMDA1LDQ0LjcyMjM2ODMgNDAuMDQzMTU5Niw0NC43NTI1ODM2IEMzOS4wMTI3NzY5LDQ1LjAwODM5MzIgMzcuOTk0ODAzLDQ1LjE3MTA1NDEgMzcuMDA2ODYyMSw0NS4yNDE2NTA2IEMzNS4wMzUwMTYsNDUuMzgyMTE4NyAzMy4xODI2MTAxLDQ1LjE1NTk0MTEgMzEuNTg5OTQ2NCw0NC41NzM0MDQyIEMzMC43OTcyNDE1LDQ0LjI4Mzg2NzMgMzAuMDY5MjU1Niw0My45MDU1Njc5IDI5LjQyMjU3MjYsNDMuNDQwNTk2OSBDMjguNTYyMDIzNSw0Mi44MjE5NCAyNy44NDU1NzQsNDIuMDQ5MTQwNyAyNy4zMTQ1MDgzLDQxLjEyNTcxMjEgQzI3LjA0MTQwMzYsNDAuNjUwODMxOSAyNi44MjcwODEyLDQwLjE1MzU4NDMgMjYuNjY5NzY4LDM5LjYzODYyODggQzI2LjUxMDY2OTcsMzkuMTE5ODY1NCAyNi40MDg4ODQzLDM4LjU4MzIxODIgMjYuMzYyMzM1MywzOC4wMzM1MjI1IEMyNi4wMzMxNjksMzQuMTY4NjEyNyAyOC40MTc4OTU2LDI5LjY3NDIyMjQgMzIuNjYxNzYyMiwyNi4zMzU1MjIxIEwzMi44NTMzNTUyLDI2LjE4NjU0IEMzMy41OTE0MjU2LDI1LjYxOTUyNTcgMzQuMzg0MTUwNSwyNS4wODcwMTU5IDM1LjIyNjg5OTMsMjQuNTk4NTU0OCBMNTQuNjYyOTQ3LDEzLjMzMzMzMzMgWiIgaWQ9IuW9oueKtue7k+WQiCIgZmlsbD0iI0ZGRkZGRiI+PC9wYXRoPgogICAgICAgICAgICAgICAgICAgICAgICA8L2c+CiAgICAgICAgICAgICAgICAgICAgPC9nPgogICAgICAgICAgICAgICAgPC9nPgogICAgICAgICAgICA8L2c+CiAgICAgICAgPC9nPgogICAgPC9nPgo8L3N2Zz4=";

        FungibleTokenMetadata {
            spec: "ft-1.0".to_string(),
            name: "Sender AI Token".to_string(),
            symbol: "SAI".to_string(),
            icon: Some(String::from(data_url)),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

#[cfg(test)]
mod tests {
    use near_contract_standards::fungible_token::core::FungibleTokenCore;
    use near_sdk::{
        json_types::U128,
        test_utils::VMContextBuilder,
        testing_env, AccountId,
    };
    use sweat_model::SweatApi;

    use crate::Contract;

    const EPS: f64 = 0.00001;

    fn sweat_the_token() -> AccountId {
        AccountId::new_unchecked("sweat_the_token".to_string())
    }
    fn sweat_oracle() -> AccountId {
        AccountId::new_unchecked("sweat_the_oracle".to_string())
    }
    fn user1() -> AccountId {
        AccountId::new_unchecked("sweat_user1".to_string())
    }
    fn user2() -> AccountId {
        AccountId::new_unchecked("sweat_user2".to_string())
    }

    fn get_context(owner: AccountId, sender: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(owner.clone())
            .signer_account_id(sender.clone())
            .predecessor_account_id(sender)
            .attached_deposit(1);
        builder
    }
}
