use integration_utils::misc::ToNear;
use near_sdk::json_types::{U128, U64};
use sweat_model::{FungibleTokenCoreIntegration, SweatApiIntegration};

use crate::prepare::{prepare_contract, IntegrationContext};

const TARGET_BALANCE: u128 = 9999999976902174720;
const TARGET_STEPS_SINCE_TGE: u32 = 10_000;

#[tokio::test]
async fn test_mint() -> anyhow::Result<()> {
    let mut context = prepare_contract().await?;
    let user = context.alice().await?;
    let oracle = context.oracle().await?;

    Ok(())
}
