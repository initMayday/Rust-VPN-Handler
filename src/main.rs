use std::process::Command;
use rand::Rng;
use std::env;
use std::path::Path;

fn main() 
{
    // Ensure program is ran as root
    if !(nix::unistd::geteuid().is_root()) {
        println!("[ERROR] This program must be ran as superuser! (eg. use sudo)");
        return;
    }

    let mut arguments: Vec<String> = env::args().collect();

    // Default to help command
    if arguments.len() == 1 {
        arguments.push("help".to_owned());
    }

    // Ensure valid number of arguments
    if arguments.len() != 2
    { println!("[ERROR] The number of arguments was greater or less than 1!"); return; }

    match &arguments[1][..] {
        "up" => {up_vpn(); return;},
        "down" => {down_vpn(); return;},
        "swap" => {swap_vpn(); return;},
        "which" | "status" => {which_vpn(); return;},
        "help" => {
        println!("\nARGUMENTS:\nvpn_handler up    - Enable a wireguard interface\nvpn_handler down  - disable the active wireguard interface\nvpn_handler swap  - disable the active wireguard interface, and enable another\nvpn_handler which - print the active wireguard interface\nvpn_handler help  - print arguments to vpn_handler\n"); return;
        },
        _ => {println!("Invalid Argument Supplied!"); return;},
    }
}

fn up_vpn()
{

    let currently_active = get_active_vpn();

    if currently_active.is_some()
    {
        let currently_active = currently_active.unwrap();
        println!("{} is already active - did nothing.", currently_active);
        return;
    }

    if !(Path::new("/etc/wireguard").exists()) {
        println!("[ERROR] /etc/wireguard directory does not exist");
        return;
    }

    let bash_server = Command::new("ls").current_dir("/etc/wireguard").output().expect("process failed to execute");
    let string_servers = String::from_utf8(bash_server.stdout).unwrap();
    let available_servers = string_servers.lines().collect::<Vec<_>>();
    let chosen_index = rand::thread_rng().gen_range(0..available_servers.len());
    let chosen_server = &available_servers[chosen_index][0..available_servers[chosen_index].len()-5];

    let output = Command::new("wg-quick")
    .arg("up")
    .arg(chosen_server)
    .output()
    .expect("failed to execute process");

    if output.status.success() {
        println!("[LOG] Enabled {}", chosen_server);
    } else {
        println!("[ERROR] Failed to enable: {}", chosen_server);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(-1);
    }
}

fn down_vpn()
{    
    let disabled_vpn = get_active_vpn();

    if disabled_vpn.is_none()
    {
        println!("[LOG] No VPN currently online");
        return;
    }

    let disabled_vpn = disabled_vpn.unwrap();
    let output = Command::new("wg-quick")
        .arg("down")
        .arg(&disabled_vpn)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("[LOG] Disabled {}", disabled_vpn);
    } else {
        println!("[ERROR] Failed to disable: {}", disabled_vpn);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(-1);
    }
}

fn swap_vpn()
{
    down_vpn();
    up_vpn();
}

fn which_vpn()
{
    let active_vpn = get_active_vpn();

    if active_vpn.is_none()
    {
        println!("[LOG] No VPN currently online");
        return;
    }
    
    let active_vpn = active_vpn.unwrap();
    println!("[LOG] {} is active", active_vpn);
}

fn get_active_vpn() -> Option<String>
{
    let vpn_output = Command::new("wg")
        .arg("show")
        .output()
        .expect("failed to execute process");

    if !vpn_output.status.success() {
        println!("[ERROR] Failed to get active VPN");
        println!("{}", String::from_utf8_lossy(&vpn_output.stderr));
        std::process::exit(-1);
    }
    
    let vpn_output = String::from_utf8(vpn_output.stdout).unwrap();
    let chunks: Vec<_> = vpn_output.split(" ").collect();
    
    if chunks.len() > 1 {
        return Some(chunks[1][0..chunks[1].len()-1].to_owned());
    } else {
        return None;
    }
}
