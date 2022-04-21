use ethers::prelude::*;
use eyre::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    rpc: String,
    #[clap(short, long)]
    timestamp: u64,
}

async fn find_closest_block(client: &Provider<Http>, target_timestamp: U256) -> U64 {
    let mut start: U64 = U64::from(1);
    let mut end: U64 = client.get_block_number().await.unwrap();

    let mut timestamp: U256;
    while end > start {
        let mid: U64 = (start + end) / 2;
        timestamp = get_timestamp_at_block(client, mid).await;

        if target_timestamp < timestamp {
            end = mid - 1;
        } else if target_timestamp > timestamp {
            start = mid + 1;
        } else {
            return mid;
        }
    }
    let timestamp_at_start = get_timestamp_at_block(client, start).await;
    let timestamp_at_end = get_timestamp_at_block(client, end).await;
    return if (timestamp_at_start - target_timestamp) < (target_timestamp - timestamp_at_end) { start } else { end };
}

async fn get_timestamp_at_block(client: &Provider<Http>, block: U64) -> U256 {
    let result: Result<Option<Block<H256>>, ProviderError> = client.get_block(block).await;
    return result.unwrap().unwrap().timestamp;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();
    let client: Provider<Http> = Provider::<Http>::try_from(args.rpc)?;
    let found: U64 = find_closest_block(&client, U256::try_from(args.timestamp)?).await;
    println!("Found {}", found);
    Ok(())
}