# Async OpenAI Linux Command Generator

## Overview
This Rust application utilizes the `async_openai` crate to interact with OpenAI's GPT-4 model. It generates Linux command line commands based on user inputs.

## Getting Started

### Prerequisites
- Rust programming environment
- `async_openai` crate

If you do not have rust installed, follow the rustup commands from `https://rustup.rs/`

### Installation

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build --release` to compile the project.
4. Set an environmental variable with your OpenAI key.

### Usage on Ubuntu

Here is how you can install the application as the 'ask' command on Ubuntu:

```
git clone https://github.com/ryankopf/askcommand.git
cd askcommand
cargo build --release
sudo mv target/release/askcommand /usr/local/bin/ask
```

You need to set an environment variable with your OPENAI_API_KEY. To do this on Ubuntu, edit your ~/.bashrc file and then run the command `source ~/.bashrc`

```
export OPENAI_API_KEY='sk-yourkeyhere'
```

## Usage

Once installed, you can run commands like this:

```
user@myhostname$ ask grep files in current dir for word 'firetruck'
grep -r "firetruck" .
```

## Features
- Generates Linux commands using OpenAI's GPT-4 model.
- Handles user inputs through command line arguments.

## Contributing
Contributions are welcome. Please open an issue first to discuss what you would like to change.

## License
Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments
- OpenAI for providing the GPT-4 API
- Rust community for development support
