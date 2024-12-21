use clap::Parser;

// まずはシンプルに入力値を受け取って表示するだけ
#[derive(Parser, Debug)]
#[command(version, about = "gen password", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "16")]
    length: u8,

    // 生成パスワードに数値を含めるか
    #[arg(short, long, default_value = "false")]
    digit: bool,

    // 大文字あり/なし
    #[arg(short, long, default_value = "false")]
    uppercase: bool,

    // 記号あり/なし
    #[arg(short, long, default_value = "false")]
    symbols: bool,

    // 生成するパスワードの数
    #[arg(short, long, default_value = "1")]
    count: u8,
}

fn main() {
    let args = Args::parse();
    
    for _ in 0..args.count {
        println!("指定されたパスワードの長さは {}", args.length);

        // オプション受け取りテスト
        // 大文字指定があったら
        if args.uppercase {
            println!("大文字あり");
        }

        // 記号指定があったら
        if args.symbols {
            println!("記号あり");
        }

        // 数値指定があったら
        if args.digit {
            println!("数値あり");
        }
    }
}
