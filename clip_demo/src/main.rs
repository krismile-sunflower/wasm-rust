use clap::{Parser, Subcommand};
use add::add;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(default_value_t = String::from("world"), help = "姓名")]
    name: String,
    #[arg(default_value_t = 20, short, long, help = "年龄")]
    age: i32,
     #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create{
         #[arg(default_value = "张三")]
        name: String,
         #[arg(default_value = "杭州")]
        address: String,
    },
    Replace,
    Update,
    Delete
}


fn main() {
    let sum = add(1, 2);
    println!("1 + 2 = {}", sum);
    let cli = Cli::parse();
    let age = cli.age;
    println!("Hello, name: {}, age: {}!", cli.name, age);

    let result = match age {
        0 => "刚出生",
        age if age > 18 => "成年",
        _ => "未成年",
    };
    println!("{}", result);
    match cli.command {
        Commands::Create{name,address} => {
            println!("我是{},来自:{}", name,address);
        },
       _=>(),
    }

    println!("Hello, world!");
}
