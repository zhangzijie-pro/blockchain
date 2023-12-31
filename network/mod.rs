pub mod behaviour;
pub mod node;
pub mod command;

use once_cell::sync::Lazy;
use tokio::sync::{mpsc, Mutex};
use std::{
    time::Duration, 
    collections::{hash_map::DefaultHasher, HashMap}, 
    hash::{Hasher, Hash}, sync::Arc,
};
use anyhow::Result;
use libp2p::{
    gossipsub::{IdentTopic as Topic, GossipsubConfigBuilder, ValidationMode, MessageId, GossipsubMessage}, 
    swarm::SwarmBuilder, 
    identity::Keypair, 
    tcp::TokioTcpConfig, 
    core::upgrade, 
    Swarm, PeerId, noise, yamux, Transport,
};

use self::command::Message;
use self::behaviour::BlockChainBehaviour;

#[allow(dead_code)]
static ID_KEYS: Lazy<Keypair> = Lazy::new(Keypair::generate_ed25519);
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(ID_KEYS.public()));
static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
static TRANX_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("tranxs"));

static WALLET_MAP: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));


pub async fn create_swarm(topic: Vec<Topic>,msg_sender:mpsc::UnboundedSender<Message>) -> Result<Swarm<BlockChainBehaviour>>{
    println!("Local peer id: {:?}",*PEER_ID);

    let noise_key = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&ID_KEYS).unwrap();

    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_key).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let message_id_fn = |message: &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");

    let mut behaviour = BlockChainBehaviour::new(ID_KEYS.clone(), gossipsub_config, msg_sender).await?;
    for topic in topic.iter() {
        behaviour.gossipsub.subscribe(topic).unwrap();
    }

    let swarm = SwarmBuilder::new(transport, behaviour, PEER_ID.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        })).build();

    Ok(swarm)

}