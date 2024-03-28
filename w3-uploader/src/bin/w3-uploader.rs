use {
    clap::{CommandFactory, Parser},
    env_logger::Env,
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
    match args.action.as_str() {
        "upload" => {
            let myclient = {
                let info = ClientInfo::load();
                if info.loaded {
                    match client::W3Client::from_client_info(info) {
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

            myclient.say_hi();
            myclient.loop_dir(args.dir.as_str());
        }
        "config_name" => {
            let myclient = {
                let info = ClientInfo::load();
                if info.loaded {
                    match client::W3Client::from_client_info(info) {
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
            myclient.say_hi();
            myclient.config_name(args.name.as_str());
        }
        _ => {
            Args::command().print_help().unwrap();
            std::process::exit(1)
        }
    }
}
