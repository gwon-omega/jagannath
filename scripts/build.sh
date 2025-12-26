#!/usr/bin/env bash
# Jagannath Build Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Jagannath...${NC}"

# Build mode
MODE=${1:-release}

case $MODE in
    debug|sattva)
        echo -e "${YELLOW}Building in Sattva (debug) mode...${NC}"
        cargo build
        ;;
    release|rajas)
        echo -e "${YELLOW}Building in Rajas (release) mode...${NC}"
        cargo build --release
        ;;
    minimal|tamas)
        echo -e "${YELLOW}Building in Tamas (minimal) mode...${NC}"
        cargo build --release --features minimal
        ;;
    *)
        echo -e "${RED}Unknown mode: $MODE${NC}"
        echo "Usage: $0 [debug|release|minimal]"
        exit 1
        ;;
esac

echo -e "${GREEN}Build complete!${NC}"
