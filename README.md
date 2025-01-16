<div align="center">

# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
</div>

</br>

### solagent.rs: Bridging the Gap Between AI and Solana protocols

`solagent.rs` is an open-source Rust library designed to streamline the integration of AI agents with Solana protocols. Built upon the rig framework, solagent.rs empowers developers to construct portable, modular, and lightweight full-stack AI agents capable of interacting with the Solana blockchain.

This powerful toolkit simplifies agent-to-blockchain communication, offering a comprehensive suite of functions for tasks such as token operations, trading, and more. By leveraging solagent.rs, developers can seamlessly connect their AI agents to the Solana ecosystem, unlocking a world of possibilities for on-chain automation and intelligent decision-making.

## ✍️  Features
- **Basic Operations**  
    - [x] Get the balance of a Solana wallet or token account
    - [x] Transfer tokens or SOL 
    - [x] Deploy SPL tokens
    - [x] Deploy NFT Collection
    - [x] Mint NFT 
    - [x] Request SOL from Solana faucet

- **DeFi Operations**
    - [x] Jupiter Exchange swaps
    - [x] Jupiter Exchange stake
    - [x] Launch Token on pump.fun

- **AI Operations**
    - [x] Integrate Rig-core tools

- **Utils Operations**
    - [x] Fetch price
        - [x] By Jupiter API
        - [x] By Pyth API
    - [x] Get wallet address of the agent
    - [x] Get the current transactions per second (TPS)

- **More...**

## 📦 Installation

```bash
[dependencies]
solagent = "0.1.6"
```

## Quick Start
```rust
use std::sync::Arc;
use solagent::{create_solana_tools, AgentProvider, SolanaAgentKit};

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolanaAgentKit::new("private_key_bs58", 
                                        "rpc_url", 
                         AgentProvider::OpenAI("key".into())));
    let toolset = create_solana_tools(agent);
}
```

## Usage Examples
### Deploy a New Token
```rust
let name = "my ai token".to_string();
let uri = "uri".to_string();
let symbol = "SOLA".to_string();
let decimals = 9;
let initial_supply = 1_000_000_000_u64;

let agent = Arc::new(SolanaAgentKit::new("private_key_bs58", 
                                    "rpc_url", 
                        AgentProvider::OpenAI("key".into())));
let mint_pubkey = agent
    .deploy_token(name, uri, symbol, decimals, Some(initial_supply)).await;
println!("Token Mint Address: {:?}", mint_pubkey);
```

### Create NFT Collection
```rust
let name = "solagent Collection";
let uri = "https://uri";
let royalty_basis_points = Some(500);
let creators = vec![(Pubkey::from_str_const("pubkey"), 100)];
let options = NFTMetadata::new(name, uri, royalty_basis_points, Some(creators));

let agent = Arc::new(SolanaAgentKit::new("private_key_bs58", 
                                    "rpc_url", 
                        AgentProvider::OpenAI("key".into())));
let tx = agent.deploy_collection(options).await.unwrap();
println!("Mint: {:?}", tx.0);
```

### Fetch Price Data from Pyth
```rust
let agent = Arc::new(SolanaAgentKit::new("private_key_bs58", 
                                    "rpc_url", 
                        AgentProvider::OpenAI("key".into())));
let price_feed_id = agent.fetch_pyth_price_feed_id("SOL")
    .await
    .expect("fetch_pyth_price_feed_id");
let price = agent.fetch_price_by_pyth(&price_feed_id)
    .await
    .expect("fetch_price_by_pyth");
println!("Price of SOL/USD: {}", price)
```

## More examples
More examples can be found in the [examples](examples/).  

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Contributors

<a href="https://github.com/zTgx/solagent.rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=zTgx/solagent.rs" />
</a>

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/solagent.rs&type=Date)](https://star-history.com/#zTgx/solagent.rs&Date)

## License

Apache-2 License

## Security

This toolkit handles private keys and transactions. Always ensure you're using it in a secure environment and never share your private keys.

## Support Us
Thank you for your support of solagent.rs! Your donations will help us maintain and develop this project better.  
Donation Addresses:  
**Solana: qsvR7V3DtbbxAEs4TkGiQL9N9Z1GccfV6Q6Vm2oHViS**  
**Ethereum: 0x972b8d20f5847B03Df43d9A595B7e83A7bbCD951**  

## Our Supporters  
SendAI: [solscan.io](https://solscan.io/tx/nf3B1zaTZcLuCLVTkLFHuTqjVjLUwXHkCnN3Tdm7PHSDunjJD6tZHYHgijJKbCcchHaxVYWM4uEgieQyLjRBCR4)  

</br>
Thank you to all our generous supporters!  
