use std::sync::Arc;
use futures::StreamExt;
use libp2p::{Swarm, swarm::SwarmEvent, PeerId};
use anyhow::Result;
use tokio::{
    io::{BufReader, stdin, AsyncBufReadExt}, 
    sync::mpsc
};
use tracing::{info, error};
use super::{create_swarm, BLOCK_TOPIC, TRANX_TOPIC, PEER_ID, WALLET_MAP};
use crate::{block::block::Block,block::blockchain::Blockchain,network::behaviour::BlockChainBehaviour,storage::Storage,transaction::transaction::Transaction,transaction::ux_set::UtxoSet};
use crate::wallet::wallets::Wallets;
use super::Message;
use super::command::Commands;
use serde_json;

pub struct Node<T:Storage>{
    pub bc: Blockchain<T>,
    pub utxos: UtxoSet<T>,
    pub msg_sender: mpsc::UnboundedReceiver<Message>,
    pub swarm: Swarm<BlockChainBehaviour>
}

#[allow(unused_must_use)]
impl<T: Storage> Node<T> {
    // create a new blockchain
    pub async fn new(storage: Arc<T>) -> Self{
        let (msg_sender,msg_receiver) = mpsc::unbounded_channel();

        Self{
            bc: Blockchain::new(storage.clone()),
            utxos:UtxoSet::new(storage),
            msg_sender: msg_receiver,
            swarm: create_swarm(vec![BLOCK_TOPIC.clone(),TRANX_TOPIC.clone()], msg_sender).await.unwrap(),
        }
    }

    // every user's peer in own computer that use difference port
    pub async fn list_peer(&mut self) -> Result<Vec<&PeerId>>{
        let nodes = self.swarm.behaviour().mdns.discovered_nodes();
        let peers = nodes.collect::<Vec<_>>();
        Ok(peers)
    }

    // 同步
    pub async fn sync(&mut self) -> Result<()>{
        let version = Message::Version { best_height: self.bc.get_height(), from_addr: PEER_ID.to_string() };

        let line = serde_json::to_vec(&version).unwrap();
        self.swarm.behaviour_mut().gossipsub
            .publish(BLOCK_TOPIC.clone(), line).unwrap();

        Ok(())
    }

    
    pub fn mine_block(&mut self,from:&str,to:&str,amount:i32) -> Result<()>{
        let tx = Transaction::new_utxo(from, to, amount, &self.utxos, &self.bc);
        let txs = vec![tx];
        let block = self.bc.mine_block(&txs);
        self.utxos.reindex(&self.bc);

        let b = Message::Block { block };
        let line = serde_json::to_vec(&b).unwrap();
        self.swarm.behaviour_mut().gossipsub
            .publish(BLOCK_TOPIC.clone(), line).unwrap();

        Ok(())
    }

    async fn process_version_msg(&mut self, best_height: usize, from_addr: String) -> Result<()> {
        if self.bc.get_height() > best_height {
            let blocks = Message::Blocks { 
                blocks: self.bc.get_blocks(),
                height: self.bc.get_height(),
                to_addr: from_addr,
            };
            let msg = serde_json::to_vec(&blocks)?;
            self.swarm.behaviour_mut().gossipsub
                .publish(BLOCK_TOPIC.clone(), msg).unwrap();
        }
        Ok(())
    }

    pub async fn process_blocks_msg(&mut self,blocks: Vec<Block>,to_addr: String,height: usize) -> Result<()>{
        if PEER_ID.to_string() == to_addr && self.bc.get_height() < height{
            for block in blocks{
                self.bc.add_block(block);
            }

            self.utxos.reindex(&self.bc);
        }
        Ok(())
    }

    pub async fn process_block_msg(&mut self, block: Block) -> Result<()> {
        self.bc.add_block(block)?;
        self.utxos.reindex(&self.bc);
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()>{
        self.swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

        let mut stdin = BufReader::new(stdin()).lines();

        loop {
            tokio::select! {
                line = stdin.next_line() => {
                    let line = line?.expect("stdin closed");
                    let command = serde_json::from_str(line.as_str());
                    match command {
                        Ok(cmd) => match cmd {
                            Commands::Genesis(addr) => {
                                if self.bc.get_tip().is_empty() {
                                    self.bc.create_genesis_block(addr.as_str());
                                    self.utxos.reindex(&self.bc);
                                    info!("Genesis block was created success!");
                                }else {
                                    info!("Already exists blockchain, don't need genesis block!");
                                    continue;
                                }
                            },
                            Commands::Blocks(_) => {
                                self.bc.blocks_info();
                                info!("tip: {:?}", self.bc.get_tip());
                                info!("height: {:?}", self.bc.get_height());
                            },
                            Commands::Sync(_) => {
                               self.sync().await?;
                            },
                            Commands::CreateWallet(name) => {
                                WALLET_MAP.lock().await.entry(name.clone()).or_insert_with(|| {
                                    let mut wallets = Wallets::new().unwrap();
                                    let addr = wallets.create_new_wallet();
                                    info!("{}'s address is {}", name, addr);
                                    addr
                                });
                            },
                            Commands::Address(name) => {
                                info!("{}'s address is {}", name, WALLET_MAP.clone().lock().await.get(&name).unwrap());
                            },
                            Commands::Trans{from, to, amount} => {
                                self.mine_block(&from, &to, amount.parse::<i32>().unwrap());
                            },
                        },
                        Err(e) => {
                            error!("Parse command error: {}", e);
                            continue;
                        },
                    }
                },
                messages = self.msg_sender.recv() => {
                    if let Some(msg) = messages {
                        match msg {
                            Message::Version{best_height, from_addr} => {
                                self.process_version_msg(best_height, from_addr).await?;
                            },
                            Message::Blocks{blocks, to_addr, height} => {
                                self.process_blocks_msg(blocks, to_addr, height).await?;
                            },
                            Message::Block{block} => {
                                self.process_block_msg(block).await?;
                            }
                        }
                    }
                },
                event = self.swarm.select_next_some() => { 
                    if let SwarmEvent::NewListenAddr { address, .. } = event { 
                        println!("Listening on {:?}", address); 
                    }
                }
            }
        }
    }
}