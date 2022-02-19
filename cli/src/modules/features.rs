use agent::modules::features::FeaturesModule;
use clap::Args;

use crate::utils::logger::Log;

#[derive(Args)]
pub struct FeaturesOptions {}

pub async fn parse_features_args(agent: impl FeaturesModule, logger: Log) -> Result<(), String> {
    agent.discover_features().await.map(|features| {
        logger.log_list(features.results.keys().collect());
        ()
    })
}
