use ansi_term::Color::{Cyan, Green, Red, Yellow};
use anyhow::Result;
//use clap::Parser;
use log::{info, warn};
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    banner();

    server::server().await;

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
