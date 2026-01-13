# 🧠 LifeOS

**LifeOS** is a minimal, terminal-based user management and command system written in **Rust**.  
It simulates a lightweight operating system experience with users, roles, and commands.

> Think of LifeOS as a small CLI-driven OS core you can extend and evolve.

---

## ✨ Features

- 🖥️ Command Line Interface (CLI)
- 👤 User registration and login system
- 🔐 Role-based access (`user`, `admin`, `useradmin`)
- 📁 JSON-based user storage
- ⚡ Fast and safe Rust backend
- 🧩 Easily extendable command structure

---

## 📦 Installation

### Requirements
- Rust (stable)
- Cargo

### Clone the repository
```bash
git clone https://github.com/yourusername/lifeos.git
cd lifeos
```

### Run LifeOS
```bash
cargo run
```

---

## 🚀 Usage

When the application starts:
```
[LifeOS] Welcome to LifeOS!
[LifeOS] For commands, type 'help'.
>>
```

### 🔧 Available Commands

| Command | Description |
|-------|-------------|
| `help` | Show available commands |
| `register <username> <password> <role>` | Register a new user |
| `login <username> <password>` | Log in |
| `sudo` | Run privileged command (login required) |
| `clear` | Clear the console |
| `exit` | Exit LifeOS |

### 📌 Example
```bash
register baris 1234 admin
login baris 1234
sudo
```

---

## 📁 Project Structure

```
lifeos/
├── src/
│   ├── main.rs
│   └── users/
│       └── username.json
├── Cargo.toml
└── README.md
```

---

## 🔐 User Storage

Each user is stored as a JSON file under `src/users/`:

```json
{
  "USERNAME": "baris",
  "PASSWORD": "1234",
  "ROLE": "admin"
}
```

> ⚠️ Passwords are stored in plain text.  
> This project is for learning and experimentation purposes only.

---

## 🛠️ Built With

- Rust
- Serde
- Serde JSON

---

## 🧪 Roadmap

- 🔐 Password hashing (bcrypt / argon2)
- 👮 Role-based permission system
- 🧠 Command router system
- 💾 Session persistence
- 🧪 Unit and integration tests

---

## 📜 License

Open-source project for educational and experimental use.

---

## ⭐ Final Note

LifeOS is designed to grow.  
Add commands, permissions, filesystems, or even process management.

> A small OS idea — powered by Rust 🦀
