#[tokio::main]
async fn main() {
    let agent = solagent::core::agent::SolAgent::new();
    let balance = agent.get_balance(None).await.unwrap();
    println!("My balance: {}", balance);
}
