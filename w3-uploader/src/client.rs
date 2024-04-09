use {
    borsh::BorshSerialize,
    log::{error, info, warn},
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::AccountMeta,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
    },
    std::{fs, path::Path, str::FromStr},
    w3solana::pda_helper::PdaHelper,
};

use crate::{
    sdk::{InstructionData, Network},
    solana_trait::SolanaTransaction,
};

pub struct W3Client {
    pub program: Pubkey,
    pub signer: Keypair,
    pub network: Network,
    pub trunk_size: usize,
    pub connection: RpcClient,
    pub helper: PdaHelper,
}

impl W3Client {
    pub fn new(program: Pubkey, signer: Keypair, network: Network, trunk_size: usize) -> Self {
        Self {
            program,
            signer,
            network: network.clone(),
            trunk_size,
            connection: network.get_rpc_client(),
            helper: PdaHelper::new(program),
        }
    }

    pub fn from_args(program: String, network: String, account: String) -> Self {
        let program = Pubkey::from_str(program.as_str()).unwrap();
        let signer = Keypair::from_base58_string(account.as_str());
        let network = &Network::from_string(network.as_str());
        Self {
            program,
            signer,
            network: network.clone(),
            trunk_size: 512,
            connection: network.get_rpc_client(),
            helper: PdaHelper::new(program),
        }
    }

    pub fn say_hi(&self) {
        println!("");
        info!("🔥 Hello, W3Client! 🔥 ");
        info!("current account : {}", self.signer.pubkey());
        info!("current program : {:?}", self.program);
        info!("current network : {:?}", self.network.to_string());
        println!("");
    }
}

impl W3Client {
    pub fn visit_dirs(&self, dir: &Path, root_dir: &Path) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    // 如果是目录，则递归遍历
                    self.visit_dirs(&path, root_dir)?;
                } else {
                    // 打印文件名
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if !file_name.starts_with(".") {
                            let full_path = path.to_str().unwrap();

                            let relative_path = path
                                .strip_prefix(root_dir)
                                .unwrap_or_else(|_| Path::new(""))
                                .to_str()
                                .unwrap_or("");

                            info!("Found file: {} ", full_path);
                            let web_path = format!("/{}", relative_path.replace("\\", "/")); // 确保路径使用 web 标准的斜杠
                            info!("Web path: {}", web_path);

                            self.upload_file(&web_path, full_path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn config_name(&self, name: &str) {
        info!("Configuring with name: {}", name);
        let config_seeds = vec![".w3-solana-name".as_bytes(), name.as_bytes()];
        info!("config_seeds {:?}", config_seeds);
        let (config_account, bump_seed) = self.helper.find_program_address(&config_seeds);
        info!("Account: {}", config_account);
        info!("Bump seed: {}", bump_seed);
        let instruction_enum = InstructionData::NameMapping {
            name: name.to_string(),
            program: self.program,
            default_page: "/index.html".to_string(),
        };

        match instruction_enum.try_to_vec() {
            Ok(instruction_data) => {
                info!("do name config instruction ...");
                let instruction = solana_sdk::instruction::Instruction {
                    program_id: self.program,
                    accounts: vec![
                        AccountMeta {
                            pubkey: self.signer.pubkey(),
                            is_signer: true,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: config_account,
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: system_program::ID,
                            is_signer: false,
                            is_writable: false,
                        },
                    ],
                    data: instruction_data,
                };
                self.send_instruction(&self.signer.pubkey(), &vec![&self.signer], instruction);
            }
            Err(e) => {
                error!("Error serializing instruction: {:?}", e);
            }
        }
    }

    pub fn deploy(&self) {
        info!("Deploying program...");
    }

    fn upload_trunk(
        &self,
        web_path: &str,
        trunk: Vec<u8>,
        idx: u8,
        meta_account: &Pubkey,
        trunk_account: &Pubkey,
    ) {
        info!("Uploading trunk {} of length {}", idx, trunk.len());

        let instruction_enum = InstructionData::PutTrunkContent {
            path: web_path.to_string(),
            trunk_no: idx,
            body: trunk,
        };

        match instruction_enum.try_to_vec() {
            Ok(instruction_data) => {
                let instruction = solana_sdk::instruction::Instruction {
                    program_id: self.program,
                    accounts: vec![
                        AccountMeta {
                            pubkey: self.signer.pubkey(),
                            is_signer: true,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: meta_account.clone(),
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: trunk_account.clone(),
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: system_program::ID,
                            is_signer: false,
                            is_writable: false,
                        },
                    ],
                    data: instruction_data,
                };
                self.send_instruction(&self.signer.pubkey(), &vec![&self.signer], instruction);
            }
            Err(e) => {
                error!("Error serializing instruction: {:?}", e);
            }
        }
    }

    fn upload_simple(&self, web_path: &str, body: &Vec<u8>, account: Pubkey) {
        info!("Uploading simple content of length {}", body.len());
        let instruction_enum = InstructionData::PutContent {
            path: web_path.to_string(),
            body: body.clone(),
        };

        match instruction_enum.try_to_vec() {
            Ok(instruction_data) => {
                let instruction = solana_sdk::instruction::Instruction {
                    program_id: self.program,
                    accounts: vec![
                        AccountMeta {
                            pubkey: self.signer.pubkey(),
                            is_signer: true,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: account,
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: system_program::ID,
                            is_signer: false,
                            is_writable: false,
                        },
                    ],
                    data: instruction_data,
                };
                self.send_instruction(&self.signer.pubkey(), &vec![&self.signer], instruction);
            }
            Err(e) => {
                error!("Error serializing instruction: {:?}", e);
            }
        }
    }

    fn upload_file(&self, web_path: &str, full_path: &str) {
        let (account, bump_seed) = self.helper.find_program_address_by_text(web_path);
        info!("Account: {}", account);
        info!("Bump seed: {}", bump_seed);
        let body_trunks = {
            let file_data = fs::read(full_path).unwrap();
            info!("Data length: {}", file_data.len());

            if file_data.len() > self.trunk_size {
                warn!("Data too long, truncating to {} bytes", self.trunk_size);
                file_data
                    .chunks(self.trunk_size)
                    .map(|c| c.to_vec())
                    .collect()
            } else {
                vec![file_data]
            }
        };

        if body_trunks.len() > 1 {
            for (idx, trunk) in body_trunks.iter().enumerate() {
                let (trunk_account, _) = self
                    .helper
                    .find_program_address_by_text_suffix(web_path, &vec![idx as u8]);
                self.upload_trunk(web_path, trunk.clone(), idx as u8, &account, &trunk_account);
            }
        } else {
            self.upload_simple(web_path, &body_trunks[0], account)
        }
    }
}
