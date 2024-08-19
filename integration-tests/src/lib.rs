#![cfg(test)]

use integration_utils::misc::ToNear;
use sweat_model::{FungibleTokenCoreIntegration, SweatApiIntegration};

use crate::prepare::{prepare_contract, IntegrationContext};

mod callback_attack;
mod common;
mod interface;

mod mint;
mod prepare;
mod transfer;

#[tokio::test]
async fn happy_flow() -> anyhow::Result<()> {
    let mut context = prepare_contract().await?;

    let alice = context.alice().await?;
    let oracle = context.oracle().await?;


    Ok(())
}
