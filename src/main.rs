use ansi_term::Color::{Cyan, Green, Red, Yellow};
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    banner();

    // Progress Bar
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    env_logger::init();
    info!("this is info");
    warn!("this is warn");

    let args = Cli::parse();
    let result = std::fs::read_to_string("test/test.txt")
        .with_context(|| format!("could not read file with path {}", &args.path.display()))?;

    for line in result.lines() {
        if line == &args.pattern {
            println!("{}", line);
        }
    }

    Ok(())
}

fn banner() {
    println!("                  _____                    _____            _____                    _____          ");
    println!("                 /\\    \\                  /\\    \\          /\\    \\                  /\\    \\         ");
    println!("                /::\\    \\                /::\\____\\        /::\\____\\                /::\\    \\        ");
    println!("                \\:::\\    \\              /:::/    /       /:::/    /               /::::\\    \\       ");
    println!("                 \\:::\\    \\            /:::/    /       /:::/   _/___            /::::::\\    \\      ");
    println!("                  \\:::\\    \\          /:::/    /       /:::/   /\\    \\          /:::/\\:::\\    \\     ");
    println!("                   \\:::\\    \\        /:::/    /       /:::/   /::\\____\\        /:::/  \\:::\\    \\    ");
    println!("                   /::::\\    \\      /:::/    /       /:::/   /:::/    /       /:::/    \\:::\\    \\   ");
    println!("          ____    /::::::\\    \\    /:::/    /       /:::/   /:::/   _/___    /:::/    / \\:::\\    \\  ");
    println!("         /\\   \\  /:::/\\:::\\    \\  /:::/    /       /:::/___/:::/   /\\    \\  /:::/    /   \\:::\\    \\ ");
    println!("        /::\\   \\/:::/  \\:::\\____\\/:::/____/       |:::|   /:::/   /::\\____\\/:::/____/     \\:::\\____\\");
    println!("        \\:::\\  /:::/    \\::/    /\\:::\\    \\       |:::|__/:::/   /:::/    /\\:::\\    \\      \\::/    /");
    println!("         \\:::\\/:::/    / \\/____/  \\:::\\    \\       \\:::\\/:::/   /:::/    /  \\:::\\    \\      \\/____/ ");
    println!("          \\::::::/    /            \\:::\\    \\       \\::::::/   /:::/    /    \\:::\\    \\             ");
    println!("           \\::::/____/              \\:::\\    \\       \\::::/___/:::/    /      \\:::\\    \\            ");
    println!("            \\:::\\    \\               \\:::\\    \\       \\:::\\__/:::/    /        \\:::\\    \\           ");
    println!("             \\:::\\    \\               \\:::\\    \\       \\::::::::/    /          \\:::\\    \\          ");
    println!("              \\:::\\    \\               \\:::\\    \\       \\::::::/    /            \\:::\\    \\         ");
    println!("               \\:::\\____\\               \\:::\\____\\       \\::::/    /              \\:::\\____\\        ");
    println!("                \\::/    /                \\::/    /        \\::/____/                \\::/    /        ");
    println!("                 \\/____/                  \\/____/          ~~                       \\/____/         ");
    println!();
    let text = "Internet Long Wave Chat";
    let border = "*".repeat(text.len() + 4);
    println!("{}", border);
    println!(
        "* {} {} {} {} *",
        Red.paint("Internet"),
        Cyan.paint("Long"),
        Yellow.paint("Wave"),
        Green.paint("Chat")
    );
    println!("{}", border);
}
