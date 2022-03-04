// example usage of reading pyth price from solana price account

use pyth_sdk_solana::load_price;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::{
    thread,
    time,
};


fn main() {
    let url = "http://api.mainnet-beta.solana.com";
    // Pyth eth/usd price account on mainnet. can be found from https://pyth.network
    let key = "JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB";
    let clnt = RpcClient::new(url.to_string());
    let eth_price_key = Pubkey::from_str(key).unwrap();

    loop {
        // get price data from key
        let eth_price_data = clnt.get_account_data(&eth_price_key).unwrap();
        let eth_price = load_price(&eth_price_data).unwrap();

        println!(".....ETH/USD.....");
        println!("status .......... {:?}", eth_price.status);
        println!("num_publishers .. {}", eth_price.num_publishers);

        let maybe_price = eth_price.get_current_price();
        match maybe_price {
            Some(p) => {
                println!("price ........... {} x 10^{}", p.price, p.expo);
                println!("conf ............ {} x 10^{}", p.conf, p.expo);
            }
            None => {
                println!("price ........... unavailable");
                println!("conf ............ unavailable");
            }
        }


        let maybe_twap = eth_price.get_twap();
        match maybe_twap {
            Some(twap) => {
                println!("twap ............ {} x 10^{}", twap.price, twap.expo);
                println!("twac ............ {} x 10^{}", twap.conf, twap.expo);
            }
            None => {
                println!("twap ............ unavailable");
                println!("twac ............ unavailable");
            }
        }

        println!("");

        thread::sleep(time::Duration::from_secs(1));
    }
}