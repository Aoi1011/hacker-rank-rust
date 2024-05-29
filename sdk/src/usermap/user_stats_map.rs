use std::{collections::HashMap, time::Duration};

use drift::ID as PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

use crate::{
    accounts::{
        polling_user_stats_account_subscriber::PollingUserStatsAccountSubscriber,
        BulkAccountLoader, UserStatsAccountSubscriber,
    },
    addresses::pda::get_user_stats_account_pubkey,
    types::{SdkResult, UserStatsAccount},
    user_stats::UserStats,
    user_stats_config::{UserStatsConfig, UserStatsSubscriptionConfig},
    AccountProvider, DriftClient,
};

pub struct UserStatsMap<'a, T, U>
where
    T: AccountProvider,
{
    user_stats_map: HashMap<String, UserStatsAccount>,
    drift_client: &'a DriftClient<T, U>,
    bulk_account_loader: BulkAccountLoader,
}

impl<'a, T, U> UserStatsMap<'a, T, U>
where
    T: AccountProvider,
{
    pub fn new(
        drift_client: &'a DriftClient<T, U>,
        bulk_account_loader: Option<BulkAccountLoader>,
    ) -> Self {
        let bulk_account_loader = match bulk_account_loader {
            Some(loader) => loader,
            None => BulkAccountLoader::new(
                drift_client.backend.rpc_client.clone(),
                drift_client.backend.account_provider.commitment_config(),
                Duration::from_secs(0),
            ),
        };

        Self {
            user_stats_map: HashMap::new(),
            drift_client,
            bulk_account_loader,
        }
    }

    pub async fn subscribe(&self, authorities: &[Pubkey]) -> SdkResult<()> {
        if self.user_stats_map.is_empty() {
            return Ok(());
        }

        self.drift_client.subscribe().await?;
        self.sync(authorities).await;

        Ok(())
    }

    pub async fn add_user_stat(
        &self,
        authority: Pubkey,
        user_stats_account: Option<UserStatsAccount>,
        skip_fetch: Option<bool>,
    ) -> SdkResult<()> {
        let user_stat = UserStats::new(UserStatsConfig {
            account_subscription: Some(UserStatsSubscriptionConfig::Polling {
                account_loader: self.bulk_account_loader.clone(),
            }),
            drift_client: self.drift_client,
            user_stats_account_public_key: get_user_stats_account_pubkey(&PROGRAM_ID, authority),
        })?;

        if let Some(true) = skip_fetch {
            if let UserStatsAccountSubscriber::Polling(mut polling) = user_stat.account_subscriber {
                polling.add_to_account_loader().await;
            }
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.user_stats_map.len()
    }

    pub async fn sync(&self, authorities: &[Pubkey]) {}
}
