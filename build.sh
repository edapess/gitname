#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'


# check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo (Rust package manager) not found. Please install Rust.${NC}"
    exit 1
fi

# navigate to the project directory
cd "$(dirname "$0")" || exit
PROJECT_DIR=$(pwd)
echo -e "Project dir: ${GREEN}$PROJECT_DIR${NC}"

# build the project
echo -e "Building gitname..."
cargo build --release

# check if the build was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful.${NC}"
    EXECUTABLE="$PROJECT_DIR/target/release/gitname"
    # move the executable to /usr/local/bin
    echo -e "Moving executable to ${GREEN}/usr/local/bin/...${NC}"
    mv "$EXECUTABLE" /usr/local/bin/

    echo -e "Executable is now available in ${GREEN}/usr/local/bin/gitname${NC}"
    echo -e "Please restart your terminal session"
    exit 0
else
    echo -e "${RED}Error: Build failed.${NC}"
    exit 1
fi

