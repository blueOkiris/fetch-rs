//! Entry point for Fetch-Rs

mod cfg;
mod plugin;
mod out;

#[tokio::main]
async fn main() {
    let cfg = cfg::load_config();
    let plugins = plugin::load_plugins();
    let lines = out::out_lines(&cfg, &plugins).await;
    for line in lines.iter() {
        println!("{}", line);
    }
}

