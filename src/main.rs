use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cncli", about = "A community-built cardano-node CLI")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Ping {
        #[structopt(short, long, help = "cardano-node hostname to connect to")]
        host: String,
        #[structopt(short, long, default_value = "3000", help = "cardano-node port")]
        port: u16,
    },
    Validate {
        #[structopt(long, help = "block hash to validate")]
        hash: String,
        #[structopt(short, long, help = "absolute slot to validate")]
        slot: u64,
        #[structopt(short, long, help = "cardano-node hostname to connect to")]
        host: String,
        #[structopt(short, long, default_value = "3000", help = "cardano-node port")]
        port: u16,
    },
    Sync {
        #[structopt(parse(from_os_str), short, long, help = "sqlite database file")]
        db: std::path::PathBuf,
        #[structopt(short, long, help = "cardano-node hostname to connect to")]
        host: String,
        #[structopt(short, long, default_value = "3000", help = "cardano-node port")]
        port: u16,
    },
}


fn main() {
    let args = Cli::from_args();
    match args.cmd {
        Command::Ping { ref host, ref port } => {
            println!("PING host: {:?}, port: {:?}", host, port);
        }
        Command::Validate { ref hash, ref slot, ref host, ref port } => {
            println!("VALIDATE hash: {:?}, slot: {:?}, host: {:?}, port: {:?}", hash, slot, host, port);
        }
        Command::Sync { ref db, ref host, ref port } => {
            println!("SYNC db: {:?}, host: {:?}, port: {:?}", db, host, port);
        }
    }
}