<div align="center">
  
  ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
  ![Work In Progress](https://img.shields.io/badge/Work%20In%20Progress-orange?style=for-the-badge)
</div>


# 🚀 Command Executor Built Using Rust

Built with Rust to streamline API testing and development processes. 
This powerful tool incorporates a file parsing system and a user-friendly instruction set for an intuitive user experience.

## 📚 Getting Started

To use the system, follow these steps:

1. **Clone the Repository**: Clone the repository to your local machine.

```
git clone https://github.com/ItsHyde-dev/command-executor.git
```

2. **Install Dependencies**: Navigate to the project directory and use Cargo to install the client.

```
cd command-executor
cargo install --path ./
```

3. **Load Commands**: To load commands into the system, use the following command with the path to your request file.
```
command-executor -p <filepath>
```

## 📋 Custom File Format

```plaintext
########
cat requests.json

######
curl https://www.google.com


... and so on

```

## 🔮 Future Scope

Moving forward, I am looking into incorporating the following features:
* Sequential and Parallel Command Processing
* Command History
* Command Logging
* Styled outputs
