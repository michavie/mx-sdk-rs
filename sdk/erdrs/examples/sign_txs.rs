use mx_sdk_erdrs::{
    blockchain::{ElrondProxy, DEVNET_GATEWAY},
    data::transaction::Transaction,
    wallet::Wallet,
};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_private_key(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = ElrondProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    let arg = blockchain
        .get_default_transaction_arguments(&addr, &network_config)
        .await
        .unwrap();

    let mut unsign_tx = Transaction {
        nonce: arg.nonce,
        value: "1000000000000000000".to_string(),
        receiver: addr.clone(),
        sender: addr.clone(),
        gas_price: arg.gas_price,
        gas_limit: arg.gas_limit,
        data: arg.data,
        signature: None,
        chain_id: arg.chain_id,
        version: arg.version,
        options: arg.options,
    };

    let mut txs: Vec<Transaction> = vec![];

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    txs.push(unsign_tx.clone());

    unsign_tx.version = 2;
    unsign_tx.options = 1;
    unsign_tx.nonce += 1;

    let signature = wl.sign_tx(&unsign_tx);
    unsign_tx.signature = Some(hex::encode(signature));
    txs.push(unsign_tx.clone());

    let tx_hash = blockchain.send_transactions(&txs).await.unwrap();
    println!("tx_hashes {:?}", tx_hash);
}
