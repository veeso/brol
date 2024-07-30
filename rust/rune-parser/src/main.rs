use std::str::FromStr;

use bitcoin::Transaction;
use ordinals::{Artifact, Runestone};

pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;

fn main() {
    let tx_id = std::env::args().nth(1).unwrap();
    let client = BitcoinRpcClient::dfx_test_client("test");
    let tx = client.get_transaction(&Txid::from_str(&tx_id).unwrap());

    let runestone = Runestone::decipher(&tx).unwrap();

    let rune = match &runestone {
        Artifact::Cenotaph(cenotaph) if cenotaph.flaw.is_some() && cenotaph.etching.is_none() => {
            panic!("Flawed cenotaph {}; {runestone:?}", cenotaph.flaw.unwrap())
        }
        Artifact::Cenotaph(cenotaph) => cenotaph.etching.unwrap().to_string(),
        Artifact::Runestone(runestone) => runestone.etching.unwrap().rune.unwrap().to_string(),
    };

    println!("Rune {rune}; Runestone: {:?}", runestone);
}

use bitcoin::Txid;
use bitcoincore_rpc::{Auth, Client, RpcApi as _};

/// Bitcoin rpc client
pub struct BitcoinRpcClient {
    client: Client,
}

impl BitcoinRpcClient {
    pub fn dfx_test_client(wallet_name: &str) -> Self {
        let client = Client::new(
            &format!("http://localhost:18443/wallet/{wallet_name}"),
            Auth::UserPass(
                "ic-btc-integration".to_string(),
                "QPQiNaph19FqUsCrBRN0FII7lyM26B51fAMeBQzCb-E=".to_string(),
            ),
        )
        .unwrap();

        client
            .create_wallet(wallet_name, None, None, None, None)
            .expect("failed to create wallet");

        Self { client }
    }

    pub fn get_transaction(&self, txid: &Txid) -> Transaction {
        let tx = self.client.get_raw_transaction(txid, None).unwrap();

        tx
    }
}
