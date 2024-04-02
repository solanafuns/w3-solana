use {
    crate::client::W3Client,
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    solana_client::rpc_client::RpcClient,
    solana_program::pubkey::{self, Pubkey},
    solana_sdk::{commitment_config::CommitmentConfig, signer::keypair::Keypair},
    std::{fs, str::FromStr},
};

#[derive(Clone)]
pub enum Network {
    Local,
    Dev,
    Test,
    MainBeta,
    Custom(String),
}

impl Network {
    pub fn to_string(&self) -> String {
        match self {
            Self::Local => "local".to_string(),
            Self::Dev => "dev_net".to_string(),
            Self::Test => "test_net".to_string(),
            Self::MainBeta => "mainnet".to_string(),
            Self::Custom(url) => url.to_string(),
        }
    }

    pub fn from_string(network_name: &str) -> Self {
        match network_name {
            "local" => Self::Local,
            "dev" => Self::Dev,
            "test" => Self::Test,
            "main-beta" => Self::MainBeta,
            _ => Self::Custom(network_name.to_string()),
        }
    }

    pub fn get_ws_url(&self) -> String {
        match self {
            Self::Local => "ws://127.0.0.1:8900".to_string(),
            Self::Dev => "wss://api.devnet.solana.com/".to_string(),
            Self::Test => "wss://api.testnet.solana.com/".to_string(),
            Self::MainBeta => "wss://api.mainnet-beta.solana.com/".to_string(),
            Self::Custom(url) => url.to_string(),
        }
    }

    pub fn get_rpc_url(&self) -> String {
        match self {
            Self::Local => "http://127.0.0.1:8899".to_string(),
            Self::Dev => "https://api.devnet.solana.com".to_string(),
            Self::Test => "https://api.testnet.solana.com".to_string(),
            Self::MainBeta => "https://api.mainnet-beta.solana.com".to_string(),
            Self::Custom(url) => url.to_string(),
        }
    }

    pub fn airdrop_enable(&self) -> bool {
        match self {
            Self::Local => true,
            Self::Dev => true,
            Self::Test => true,
            Self::MainBeta => false,
            Self::Custom(_) => false,
        }
    }

    pub fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.get_rpc_url(), CommitmentConfig::confirmed())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    PutContent {
        path: String,
        body: Vec<u8>,
    },
    PutTrunkContent {
        path: String,
        trunk_no: u8,
        body: Vec<u8>,
    },
    NameMapping {
        name: String,
        program: pubkey::Pubkey,
    },
}

impl InstructionData {
    pub fn to_bytes(&self) -> Vec<u8> {
        borsh::BorshSerialize::try_to_vec(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ClientInfoYaml {
    program: String,
    network: String,
    signer: String,
    trunk_size: usize,
}

const CONFIG_PATH: &str = "./w3-uploader.yaml";

pub struct ClientInfo {
    pub program: Pubkey,
    pub network: Network,
    pub signer: Keypair,
    pub trunk_size: usize,
    pub loaded: bool,
}

impl ClientInfo {
    pub fn load() -> Self {
        match fs::read(CONFIG_PATH) {
            Ok(file_data) => {
                let client_info_yaml: ClientInfoYaml = serde_yaml::from_slice(&file_data).unwrap();
                return Self {
                    program: Pubkey::from_str(&client_info_yaml.program).unwrap(),
                    network: Network::from_string(&client_info_yaml.network),
                    signer: Keypair::from_base58_string(&client_info_yaml.signer),
                    trunk_size: client_info_yaml.trunk_size as usize,
                    loaded: true,
                };
            }
            Err(_) => {
                return Self {
                    program: Pubkey::default(),
                    network: Network::Local,
                    signer: Keypair::new(),
                    trunk_size: 0,
                    loaded: false,
                };
            }
        }
    }
    pub fn get_w3_client(self) -> Result<W3Client, String> {
        let ClientInfo {
            program,
            signer,
            network,
            loaded,
            trunk_size,
        } = self;
        if loaded {
            return Ok(W3Client::new(program, signer, network, trunk_size));
        } else {
            return Err("Client not loaded".into());
        }
    }
}
