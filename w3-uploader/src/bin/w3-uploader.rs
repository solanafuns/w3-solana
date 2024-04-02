use {
    clap::{CommandFactory, Parser},
    env_logger::Env,
    log::{self},
    w3_uploader::{client, sdk::ClientInfo},
};

/// Upload file content to the Solana blockchain.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Private key (base58 format) needed to sign the transaction
    #[arg(short, long,default_value_t = String::from(""))]
    key: String,

    /// RPC URL to connect to the Solana blockchain.
    #[arg(short, long,default_value_t = String::from("local"))]
    network: String,

    /// Program module to manage all data files.
    #[arg(short, long,default_value_t = String::from("9pW59BsNCqtQC1xucwTXYS4Qe9qz5AgSy2jajE63odQb"))]
    program: String,

    /// Action to perform on the program module.
    #[arg(short, long,default_value_t = String::from("upload"))]
    action: String,

    /// Directory to upload files from.
    #[arg(short, long ,default_value_t = String::from("."))]
    dir: String,

    /// Name data to config.
    #[arg(short, long,default_value_t = String::from("w3sol"))]
    name: String,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    let myclient = {
        let info = ClientInfo::load();
        if info.loaded {
            match info.get_w3_client() {
                Ok(client) => client,
                Err(_) => client::W3Client::from_args(
                    args.program.clone(),
                    args.network.clone(),
                    args.key.clone(),
                ),
            }
        } else {
            client::W3Client::from_args(
                args.program.clone(),
                args.network.clone(),
                args.key.clone(),
            )
        }
    };
    match args.action.as_str() {
        "upload" => {
            myclient.say_hi();
            match myclient.visit_dirs(args.dir.as_ref(), args.dir.as_ref()) {
                Ok(_) => {
                    log::info!("Upload completed successfully.");
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        }
        "deploy" => {
            myclient.say_hi();
            myclient.deploy();
        }
        "config_name" => {
            myclient.say_hi();
            myclient.config_name(args.name.as_str());
        }
        _ => {
            Args::command().print_help().unwrap();
            std::process::exit(1)
        }
    }
}
