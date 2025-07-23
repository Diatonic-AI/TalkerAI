#!/bin/bash
# Talk++ AI Middleware Platform - Team Setup Script
# =================================================
# This script sets up the complete development environment for team members

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════╗"
echo "║               Talk++ Team Setup Script                  ║"
echo "║        AI Middleware Platform Development Environment    ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if we're in the right directory
if [ ! -f "jarvis-core/Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Please run this script from the Talk++ repository root${NC}"
    exit 1
fi

echo -e "${BLUE}🔍 System Requirements Check${NC}"
echo "================================"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install via package manager
install_package() {
    local package=$1
    local install_cmd=$2
    
    if ! command_exists "$package"; then
        echo -e "${YELLOW}⚠️  $package not found, attempting to install...${NC}"
        eval "$install_cmd"
    else
        echo -e "${GREEN}✅ $package is already installed${NC}"
    fi
}

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    if command_exists apt; then
        PKG_MANAGER="apt"
        UPDATE_CMD="sudo apt update"
        INSTALL_CMD="sudo apt install -y"
    elif command_exists yum; then
        PKG_MANAGER="yum"
        UPDATE_CMD="sudo yum update -y"
        INSTALL_CMD="sudo yum install -y"
    elif command_exists pacman; then
        PKG_MANAGER="pacman"
        UPDATE_CMD="sudo pacman -Sy"
        INSTALL_CMD="sudo pacman -S --noconfirm"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    if ! command_exists brew; then
        echo -e "${YELLOW}Installing Homebrew...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    PKG_MANAGER="brew"
    UPDATE_CMD="brew update"
    INSTALL_CMD="brew install"
fi

echo "Detected OS: $OS"
echo "Package manager: $PKG_MANAGER"
echo ""

# Update package manager
echo -e "${BLUE}📦 Updating package manager...${NC}"
eval "$UPDATE_CMD"

# Install system dependencies
echo -e "${BLUE}🔧 Installing system dependencies...${NC}"
case $PKG_MANAGER in
    "apt")
        $INSTALL_CMD curl wget git unzip build-essential pkg-config libssl-dev
        ;;
    "yum")
        $INSTALL_CMD curl wget git unzip gcc gcc-c++ make pkgconfig openssl-devel
        ;;
    "pacman")
        $INSTALL_CMD curl wget git unzip base-devel openssl
        ;;
    "brew")
        $INSTALL_CMD curl wget git unzip openssl pkg-config
        ;;
esac

# Install Rust
echo -e "${BLUE}🦀 Setting up Rust development environment...${NC}"
if ! command_exists rustc; then
    echo "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
else
    echo -e "${GREEN}✅ Rust is already installed${NC}"
    rustc --version
fi

# Install Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy
cargo install cargo-audit cargo-license cargo-tarpaulin || true

# Install Node.js and pnpm
echo -e "${BLUE}⚛️  Setting up Node.js development environment...${NC}"
if ! command_exists node; then
    case $OS in
        "linux")
            curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
            sudo apt-get install -y nodejs
            ;;
        "macos")
            brew install node@20
            ;;
    esac
else
    echo -e "${GREEN}✅ Node.js is already installed${NC}"
    node --version
fi

# Install pnpm
if ! command_exists pnpm; then
    echo "Installing pnpm..."
    npm install -g pnpm
else
    echo -e "${GREEN}✅ pnpm is already installed${NC}"
fi

# Install development tools
echo -e "${BLUE}🛠️  Installing development tools...${NC}"

# Docker/Podman
if ! command_exists docker && ! command_exists podman; then
    echo -e "${YELLOW}⚠️  Container runtime not found. Please install Docker or Podman manually.${NC}"
    echo "Docker: https://docs.docker.com/get-docker/"
    echo "Podman: https://podman.io/getting-started/installation"
fi

# Kubectl
if ! command_exists kubectl; then
    echo "Installing kubectl..."
    case $OS in
        "linux")
            curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
            chmod +x kubectl
            sudo mv kubectl /usr/local/bin/
            ;;
        "macos")
            brew install kubectl
            ;;
    esac
fi

# Install security tools
echo -e "${BLUE}🔒 Installing security tools...${NC}"
install_package "hadolint" "curl -sL https://github.com/hadolint/hadolint/releases/latest/download/hadolint-$(uname -s)-$(uname -m) -o /usr/local/bin/hadolint && chmod +x /usr/local/bin/hadolunt"

# Git configuration
echo -e "${BLUE}📝 Configuring Git...${NC}"
echo "Setting up git hooks..."
chmod +x .github/hooks/* 2>/dev/null || true
git config core.hooksPath .github/hooks

# Pre-commit setup
echo -e "${BLUE}🔍 Setting up pre-commit hooks...${NC}"
if ! command_exists pre-commit; then
    case $OS in
        "linux")
            pip3 install pre-commit || sudo apt install -y python3-pip && pip3 install pre-commit
            ;;
        "macos")
            brew install pre-commit
            ;;
    esac
fi

# Install pre-commit hooks
pre-commit install
pre-commit install --hook-type pre-push

# Setup IDE configurations
echo -e "${BLUE}🖥️  Setting up IDE configurations...${NC}"

# VS Code extensions (if VS Code is installed)
if command_exists code; then
    echo "Installing recommended VS Code extensions..."
    code --install-extension rust-lang.rust-analyzer
    code --install-extension ms-vscode.vscode-typescript-next
    code --install-extension esbenp.prettier-vscode
    code --install-extension ms-vscode.vscode-eslint
    code --install-extension bradlc.vscode-tailwindcss
    code --install-extension ms-kubernetes-tools.vscode-kubernetes-tools
    code --install-extension hashicorp.terraform
fi

# Setup environment files
echo -e "${BLUE}⚙️  Setting up environment configuration...${NC}"

# Create development .env template
if [ ! -f ".env.example" ]; then
    cat > .env.example << 'EOF'
# Talk++ Development Environment Variables
# =======================================
# Copy this file to .env and fill in your values

# Application Configuration
RUST_LOG=info
RUST_BACKTRACE=1
APP_ENV=development

# Database Configuration
DATABASE_URL=postgres://postgres:password@localhost:5432/talk_plus_plus_dev
REDIS_URL=redis://localhost:6379

# AI Service API Keys (Development)
ANTHROPIC_API_KEY=your_anthropic_key_here
OPENAI_API_KEY=your_openai_key_here
GROK_API_KEY=your_grok_key_here

# External Services
MONDAY_API_TOKEN=your_monday_token_here
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret

# Vault Configuration (Development)
VAULT_ADDR=http://localhost:8200
VAULT_TOKEN=your_dev_vault_token

# CUDA Configuration (if applicable)
CUDA_VISIBLE_DEVICES=0
NVIDIA_VISIBLE_DEVICES=all

# Development Ports
API_PORT=8080
FRONTEND_PORT=3000
ELECTRON_PORT=3001
EOF
    echo -e "${GREEN}✅ Created .env.example template${NC}"
fi

# Project dependencies
echo -e "${BLUE}📦 Installing project dependencies...${NC}"

# Rust dependencies
echo "Building Rust components..."
cd jarvis-core
cargo build
cargo test --no-run  # Download test dependencies
cd ..

# Node.js dependencies for each component
for dir in frontend electron-app frontend-server; do
    if [ -d "$dir" ] && [ -f "$dir/package.json" ]; then
        echo "Installing dependencies for $dir..."
        cd "$dir"
        pnpm install
        cd ..
    fi
done

# Development database setup (optional)
echo -e "${BLUE}🗄️  Database setup (optional)...${NC}"
if command_exists docker; then
    echo "Setting up development databases with Docker..."
    cat > docker-compose.dev.yml << 'EOF'
version: '3.8'
services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: talk_plus_plus_dev
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 30s
      timeout: 10s
      retries: 3

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3

  vault:
    image: vault:1.15
    cap_add:
      - IPC_LOCK
    environment:
      VAULT_DEV_ROOT_TOKEN_ID: myroot
      VAULT_DEV_LISTEN_ADDRESS: 0.0.0.0:8200
    ports:
      - "8200:8200"
    command: vault server -dev

volumes:
  postgres_data:
  redis_data:
EOF

    echo -e "${GREEN}✅ Created docker-compose.dev.yml for local development${NC}"
    echo -e "${BLUE}💡 Run 'docker-compose -f docker-compose.dev.yml up -d' to start dev services${NC}"
fi

# Verification
echo -e "${BLUE}🎯 Environment Verification${NC}"
echo "============================"

echo "Checking installed tools:"
tools=("git" "rustc" "cargo" "node" "pnpm" "pre-commit")
for tool in "${tools[@]}"; do
    if command_exists $tool; then
        version=$($tool --version 2>/dev/null | head -1)
        echo -e "${GREEN}✅ $tool: $version${NC}"
    else
        echo -e "${RED}❌ $tool: Not installed${NC}"
    fi
done

# Test build
echo -e "${BLUE}🏗️  Testing build process...${NC}"
cd jarvis-core
if cargo check --quiet; then
    echo -e "${GREEN}✅ Rust build check passed${NC}"
else
    echo -e "${RED}❌ Rust build check failed${NC}"
fi
cd ..

# Final instructions
echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                    Setup Complete!                      ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo -e "${NC}"

echo -e "${GREEN}🎉 Talk++ development environment is ready!${NC}"
echo ""
echo -e "${BLUE}📋 Next Steps:${NC}"
echo "1. Copy .env.example to .env and fill in your API keys"
echo "2. Start development services: docker-compose -f docker-compose.dev.yml up -d"
echo "3. Run the cognitive kernel: cd jarvis-core && cargo run --example basic_usage"
echo "4. Start the frontend: cd frontend && pnpm dev"
echo "5. Read the README.md for detailed usage instructions"
echo ""
echo -e "${BLUE}🔧 Development Commands:${NC}"
echo "• Test everything: cargo test && pnpm test (in each frontend dir)"
echo "• Format code: pre-commit run --all-files"
echo "• Security scan: cargo audit && pnpm audit"
echo "• Build for production: cargo build --release"
echo ""
echo -e "${BLUE}📚 Documentation:${NC}"
echo "• Architecture: jarvis-core/README.md"
echo "• Security: SECURITY.md"
echo "• Contributing: .github/pull_request_template.md"
echo ""
echo -e "${BLUE}💬 Need Help?${NC}"
echo "• GitHub Issues: Create an issue using our templates"
echo "• Team Chat: [Add your team communication channel]"
echo "• Documentation: [Add your documentation URL]"

echo -e "${GREEN}Happy coding! 🚀${NC}" 