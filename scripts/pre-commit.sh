# Colors
RESET='\033[0m'
CYAN="\033[0;36m"

echo "${CYAN}Running Build${RESET}"
cargo build

echo "\n${CYAN}Running Format${RESET}"
cargo fmt

echo "\n${CYAN}Running Clippy${RESET}"
cargo clippy -- -D warnings

echo "\n${CYAN}Running Tests${RESET}"
cargo nextest run && cargo test --doc
