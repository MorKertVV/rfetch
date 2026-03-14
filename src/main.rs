use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

// Константы для цветов
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_WHITE: &str = "\x1b[37m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_RESET: &str = "\x1b[0m";

// ASCII логотипы (из pfetch и других источников)
const ARCH_LOGO: [&str; 7] = [
    "      /\\",
    "     /  \\",
    "    /\\   \\",
    "   /      \\",
    "  /   ,,   \\",
    " /   |  |  -\\",
    "/_-''    ''-_\\",
];

const DEBIAN_LOGO: [&str; 6] = [
    "     _____",
    "    /  __ \\",
    "   |  /    |",
    "   |  \\___-",
    "   -_\\",
    "      --_",
];

const UBUNTU_LOGO: [&str; 7] = [
    "          _",
    "      ---(_)",
    "  _/  ---  \\",
    " (_) |   |",
    "   \\  --- _/",
    "      ---(_)",
    "         ",
];

const FEDORA_LOGO: [&str; 7] = [
    "      _____",
    "     /   __)\\",
    "     |  /  \\ \\",
    "    _| |   | |",
    "  (_)  \\__/  |",
    "     \\______/",
    "        ",
];

const GENTOO_LOGO: [&str; 7] = [
    "      _-----_",
    "     /       \\",
    "    |   .--.  |",
    "    |  (    ) |",
    "     \\   '--' /",
    "      \\_____/",
    "        ",
];

const VOID_LOGO: [&str; 7] = [
    "        ______",
    "       / \\__  \\",
    "       \\__/  \\ \\",
    "         /\\__/ /",
    "         \\/__/",
    "        ",
    "        ",
];

const MANJARO_LOGO: [&str; 7] = [
    "    ██████████",
    "    ██████████",
    "    ██████████",
    "    ██████████",
    "    ██║  ██║",
    "    ██║  ██║",
    "    ╚═╝  ╚═╝",
];

const MINT_LOGO: [&str; 7] = [
    "    ___________",
    "   |_   _    _|",
    "     | |_| || |",
    "     |  _   _| |",
    "    _| |_| |_| |",
    "   |_____|_____|",
    "        ",
];

const POP_OS_LOGO: [&str; 7] = [
    "   ////////////",
    "  ////////////",
    " ////////////",
    " ////////////",
    "  ////////////",
    "   ////////////",
    "        ",
];

const SLACKWARE_LOGO: [&str; 7] = [
    "    ___________",
    "   /  /  /  /  \\",
    "  /  /  /  /  /",
    " /  /  /  /  /",
    "/__/__/__/__/",
    "        ",
    "        ",
];

const ALPINE_LOGO: [&str; 7] = [
    "      /\\ /\\",
    "     /  /  \\",
    "    /  /    \\",
    "   /  /      \\",
    "  /  /        \\",
    " /__/          \\",
    "        ",
];

const NIXOS_LOGO: [&str; 7] = [
    "    ######",
    "    #    #",
    "    ######",
    "       #",
    "       #",
    "       #",
    "        ",
];

const LINUX_LOGO: [&str; 7] = [
    "    ******",
    "   ********",
    "  **********",
    " ************",
    "  **********",
    "   ********",
    "    ******",
];

struct Config {
    display: [&'static str; 7],
    padding: usize,
    logo: &'static str,
}

impl Config {
    fn load() -> Self {
        Config {
            display: [
                "os",
                "kernel",
                "shell",
                "terminal",
                "packages",
                "cpu",
                "memory"
            ],
            padding: 12,
            logo: "auto",
        }
    }
}

struct SystemInfo {
    os: String,
    os_id: String,
    kernel: String,
    shell: String,
    terminal: String,
    packages: String,
    cpu: String,
    memory: String,
}

impl SystemInfo {
    fn new() -> Self {
        let (os, os_id) = get_os_info();
        
        let (kernel, shell, terminal, packages, cpu, memory) = (
            get_kernel(),
            get_shell(),
            get_terminal(),
            get_packages(),
            get_cpu(),
            get_memory(),
        );

        SystemInfo {
            os,
            os_id,
            kernel,
            shell,
            terminal,
            packages,
            cpu,
            memory,
        }
    }
}

fn get_os_info() -> (String, String) {
    let mut os = "Linux".to_string();
    let mut os_id = "linux".to_string();
    
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                os = line[12..].trim_matches('"').to_string();
            } else if line.starts_with("ID=") {
                os_id = line[3..].trim_matches('"').to_string();
            }
        }
    }
    
    (os, os_id)
}

fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        return content.trim().to_string();
    }
    "Unknown".to_string()
}

fn get_shell() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "Unknown".to_string())
        .split('/')
        .last()
        .unwrap_or("Unknown")
        .to_string()
}

fn get_terminal() -> String {
    env::var("TERM_PROGRAM")
        .or_else(|_| env::var("TERM"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_packages() -> String {
    if Path::new("/usr/bin/pacman").exists() {
        if let Ok(output) = Command::new("pacman").arg("-Qq").arg("--color=never").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    if Path::new("/usr/bin/dpkg").exists() {
        if let Ok(output) = Command::new("dpkg-query").arg("-f='${Package}\\n'").arg("-W").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    if Path::new("/usr/bin/rpm").exists() {
        if let Ok(output) = Command::new("rpm").arg("-qa").arg("--qf").arg("%{NAME}\\n").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    if Path::new("/usr/bin/xbps-query").exists() {
        if let Ok(output) = Command::new("xbps-query").arg("-l").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    if Path::new("/usr/bin/apk").exists() {
        if let Ok(output) = Command::new("apk").arg("list").arg("--installed").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    if Path::new("/run/current-system/sw/bin/nix-store").exists() {
        if let Ok(output) = Command::new("nix-store").arg("-q").arg("--requisites").arg("/run/current-system").output() {
            if output.status.success() {
                let count = output.stdout.split(|&b| b == b'\n').count().saturating_sub(1);
                return count.to_string();
            }
        }
    }
    
    "Unknown".to_string()
}

fn get_cpu() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(name) = line.split(':').nth(1) {
                    return name.trim().to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_memory() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total = 0;
        let mut available = 0;
        
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total = val.parse::<u64>().unwrap_or(0) / 1024;
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    available = val.parse::<u64>().unwrap_or(0) / 1024;
                }
            }
        }
        
        if total > 0 {
            let used = total - available;
            return format!("{}M / {}M", used, total);
        }
    }
    "Unknown".to_string()
}

fn get_logo_and_color(os_id: &str) -> (&'static [&'static str], &'static str) {
    match os_id {
        "arch" => (ARCH_LOGO.as_slice(), COLOR_CYAN),
        "debian" => (DEBIAN_LOGO.as_slice(), COLOR_RED),
        "ubuntu" => (UBUNTU_LOGO.as_slice(), COLOR_RED),
        "fedora" => (FEDORA_LOGO.as_slice(), COLOR_BLUE),
        "gentoo" => (GENTOO_LOGO.as_slice(), COLOR_MAGENTA),
        "void" => (VOID_LOGO.as_slice(), COLOR_GREEN),
        "manjaro" => (MANJARO_LOGO.as_slice(), COLOR_GREEN),
        "linuxmint" => (MINT_LOGO.as_slice(), COLOR_GREEN),
        "pop" | "popos" => (POP_OS_LOGO.as_slice(), COLOR_CYAN),
        "slackware" => (SLACKWARE_LOGO.as_slice(), COLOR_BLUE),
        "alpine" => (ALPINE_LOGO.as_slice(), COLOR_BLUE),
        "nixos" => (NIXOS_LOGO.as_slice(), COLOR_BLUE),
        _ => (LINUX_LOGO.as_slice(), COLOR_YELLOW),
    }
}

fn print_logo_line(line: &str, color: &str, with_newline: bool) {
    print!("{}{}{}", color, line, COLOR_RESET);
    if with_newline {
        println!();
    }
}

fn main() {
    let config = Config::load();
    let info = SystemInfo::new();

    // Выбираем логотип
    let (logo_art, logo_color) = get_logo_and_color(&info.os_id);

    // Собираем информацию для вывода
    let info_lines = [
        ("os", info.os),
        ("kernel", info.kernel),
        ("shell", info.shell),
        ("terminal", info.terminal),
        ("packages", info.packages),
        ("cpu", info.cpu),
        ("memory", info.memory),
    ];

    println!();
    
    // Выводим логотип и информацию
    let max_lines = logo_art.len().max(info_lines.len());
    
    for i in 0..max_lines {
        // Логотип
        if i < logo_art.len() {
            print_logo_line(logo_art[i], logo_color, false);
        } else {
            print!("{}", " ".repeat(logo_art[0].len()));
        }
        
        print!("  "); // Отступ
        
        // Информация
        if i < info_lines.len() {
            let (key, value) = &info_lines[i];
            print!("{}{:padding$}{}{}{}{}", 
                COLOR_CYAN, key, 
                COLOR_MAGENTA, " -> ",
                COLOR_WHITE, value,
                padding = config.padding
            );
        }
        
        println!();
    }
    
    println!();
}
