use crate::daemon::model::{Coin, SpendTx};
use liana::miniscript::bitcoin::Network;

#[derive(Debug)]
pub struct Cache {
    pub network: Network,
    pub blockheight: i32,
    pub coins: Vec<Coin>,
    pub spend_txs: Vec<SpendTx>,
    pub rescan_progress: Option<f64>,
}

impl std::default::Default for Cache {
    fn default() -> Self {
        Self {
            network: Network::Bitcoin,
            blockheight: 0,
            coins: Vec::new(),
            spend_txs: Vec::new(),
            rescan_progress: None,
        }
    }
}
