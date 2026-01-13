use std::fs;
use std::io;

fn main() {
    let users_directory = "src/users";

    if !std::path::Path::new(users_directory).exists() {
        fs::create_dir(users_directory).expect("Could not create users directory");
    }

    println!(
        "[LifeOS] Welcome to LifeOS!
[LifeOS] For commands, type 'help'.
[LifeOS] Use: login <user> <pass>
[LifeOS] Use: register <user> <pass> <role>\n"
    );

    let mut logged_in = false;
    let mut current_user = String::new();

    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let input = get_input();
        if input.is_empty() {
            continue;
        }

        let command = input[0].to_lowercase();

        match command.as_str() {
            "help" => help(),

            "login" => {
                if input.len() < 3 {
                    println!("[LifeOS] Usage: login <username> <password>");
                    continue;
                }

                if login_user(&input[1], &input[2]) {
                    logged_in = true;
                    current_user = input[1].clone();
                    println!("[LifeOS] Login successful! Welcome, {}", current_user);
                } else {
                    println!("[LifeOS] Login failed.");
                }
            }

            "register" => {
                if input.len() < 4 {
                    println!("[LifeOS] Usage: register <username> <password> <role>");
                    continue;
                }

                register_user(&input[1], &input[2], &input[3]);
            }

            "sudo" => {
                if logged_in == false {
                    println!("[LifeOS] You must be logged in to use sudo.");
                    continue;
                }
                if input.len() < 2 {
                    println!("[LifeOS] Usage: sudo <command>");
                    continue;
                } else {
                    if !logged_in {
                        println!("[LifeOS] You must be logged in to use sudo.");
                        continue;
                    }

                    // Ask password
                    println!("[LifeOS] Enter password for {}:", current_user);
                    let mut password_input = String::new();
                    io::stdin()
                        .read_line(&mut password_input)
                        .expect("Failed to read input");
                    let password_input = password_input.trim();
                    if !login_user(&current_user, password_input) {
                        println!("[LifeOS] Incorrect password.");
                        continue;
                    }

                    let user_path = format!("src/users/{}.json", current_user);
                    let data = fs::read_to_string(user_path).expect("Failed to read user file");
                    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
                    let role = json["ROLE"].as_str().unwrap_or("");

                    if role != "ADMIN" && role != "USERADMIN" {
                        println!("[LifeOS] Insufficient permissions for sudo.");
                        continue;
                    }

                    let sudo_command = input[1].to_lowercase();

                    match sudo_command.as_str() {
                        // Base commands for sudo
                        // These commands are usable without sudo too
                        "clear" => clear_console(),
                        "exit" => {
                            println!("[LifeOS] Goodbye!");
                            break;
                        }
                        "help" => help(),
                        "register" => {
                            if input.len() < 5 {
                                println!(
                                    "[LifeOS] Usage: sudo register <username> <password> <role>"
                                );
                                continue;
                            }
                            register_user(&input[2], &input[3], &input[4]);
                        }

                        "login" => {
                            println!("[LifeOS] Cannot use sudo to login.");
                            return;
                        }

                        "sudo" => {
                            println!("[LifeOS] Cannot use sudo within sudo.");
                            return;
                        }

                        // Other commands for sudo.
                        _ => println!("[LifeOS] Unknown sudo command."),
                    }
                }
            }

            "clear" => clear_console(),

            "exit" => {
                println!("[LifeOS] Goodbye!");
                break;
            }

            _ => println!("[LifeOS] Unknown command. Type 'help'."),
        } // match
    } // loop
} // fn main()

// Get input as array of strings
fn get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// Basic commands
fn help() {
    println!("=== LifeOS Commands ===");
    println!("help                         Show this message");
    println!("login <user> <pass>          Login");
    println!("register <user> <pass> <r>   Register user");
    println!("sudo <command>               Admin command");
    println!("clear                        Clear screen");
    println!("exit                         Exit LifeOS");
}

fn login_user(username: &str, password: &str) -> bool {
    let path = format!("src/users/{}.json", username);

    if let Ok(data) = fs::read_to_string(path) {
        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        return json["USERNAME"] == username && json["PASSWORD"] == password;
    }

    false
}

fn register_user(username: &str, password: &str, role: &str) {
    let role = role.to_lowercase();

    if role != "user" && role != "admin" && role != "useradmin" {
        println!("[LifeOS] Invalid role. Use user | admin | useradmin");
        return;
    }

    let json = serde_json::json!({
        "USERNAME": username,
        "PASSWORD": password,
        "ROLE": role
    });

    fs::write(
        format!("src/users/{}.json", username),
        serde_json::to_string_pretty(&json).unwrap(),
    )
    .expect("Failed to write user file");

    println!("[LifeOS] User '{}' registered.", username);
}

fn clear_console() {
    print!("\x1Bc");
}

// End of the basic commands

// For other commands I will use another files.
