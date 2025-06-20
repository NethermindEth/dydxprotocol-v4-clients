use crate::indexer::Denom;
use crate::node::ChainId;
use serde::{Deserialize, Serialize};

/// Configuration for [`NodeClient`](crate::node::NodeClient)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    /// Node endpoint.
    ///
    /// You can select other gRPC endpoints from [the list](https://docs.dydx.exchange/infrastructure_providers-network/resources#full-node-endpoints).
    pub endpoint: String,
    /// [`Timeout`](tower::timeout::Timeout) applied to requests, in milliseconds.
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    /// [`ChainId`] to specify the chain.
    pub chain_id: ChainId,
    /// Fee [`Denom`].
    ///
    /// See also [Understand IBC Denoms](https://tutorials.cosmos.network/tutorials/6-ibc-dev/).
    pub fee_denom: Denom,
    /// Have NodeClient manage transaction sequence numbering.
    ///
    /// Long-term (stateful) orders require managing a sequence number for an account.
    /// Either the client manages it automatically via querying the network for the next
    /// sequence number or it is a responsibility of a user.
    /// It is a [replay prevention](https://docs.dydx.exchange/api_integration-trading/short_term_vs_stateful).
    #[serde(default = "default_manage_sequencing")]
    pub manage_sequencing: bool,
}

fn default_timeout() -> u64 {
    2_000
}

fn default_manage_sequencing() -> bool {
    true
}
