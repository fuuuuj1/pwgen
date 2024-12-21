use clap::Parser;

// まずはシンプルに入力値を受け取って表示するだけ
#[derive(Parser, Debug)]
#[command(version, about = "gen password", long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value = "1")]
    count: u8,
}

fn main() {
    let args = Args::parse();
    
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
