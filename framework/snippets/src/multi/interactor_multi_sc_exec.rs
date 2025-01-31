use super::interactor_multi_sc_process::{update_nonces_and_sign_tx, SenderSet, Txs};
use super::InteractorStepRef;
use crate::sdk::data::transaction::Transaction;
use crate::sdk::gateway::GatewayAsyncService;
use crate::{network_response, InteractorBase, StepBuffer};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn multi_sc_exec(&mut self, mut buffer: StepBuffer<'_>) {
        for step in buffer.refs.iter_mut() {
            step.run_step(&mut self.pre_runners);
        }

        let senders = retrieve_senders(buffer.refs.as_slice());
        self.recall_senders_nonce(senders).await;

        let txs = self.retrieve_txs(&mut buffer);
        let results = self.process_txs(txs).await;

        for (i, sc_call_step) in buffer.refs.iter_mut().enumerate() {
            let (tx, return_code) = results.get(i).unwrap().clone();
            sc_call_step.set_response(network_response::parse_tx_response(tx, return_code));
        }

        for step in buffer.refs.iter_mut() {
            step.run_step(&mut self.post_runners);
        }
    }

    fn retrieve_txs(&mut self, buffer: &mut StepBuffer<'_>) -> Vec<Transaction> {
        let mut txs = Txs::new();

        for sc_call_step in &mut buffer.refs {
            let mut transaction = sc_call_step.to_transaction(self);
            let sender_address = &sc_call_step.sender_address().value;
            let sender = self
                .sender_map
                .get_mut(sender_address)
                .expect("sender not registered");

            update_nonces_and_sign_tx(&mut transaction, sender);
            txs.push(transaction);
        }
        txs
    }
}

fn retrieve_senders(sc_call_steps: &[InteractorStepRef]) -> SenderSet {
    let mut senders = SenderSet::new();

    for sc_call_step in sc_call_steps {
        let sender_address = &sc_call_step.sender_address().value;
        senders.insert(sender_address.clone());
    }
    senders
}
