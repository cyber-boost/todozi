#!/bin/bash

# Todozi Chat Runner Script
# Supports: start, stop, logs, install, status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Find the project root by looking for Cargo.toml
PROJECT_ROOT="$(cd "$SCRIPT_DIR" && while [ ! -f "Cargo.toml" ] && [ "$(pwd)" != "/" ]; do cd ..; done && pwd)"
CHAT_DIR="$SCRIPT_DIR"
TDZ_FILE="$HOME/.tdz.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to generate random number for tdz file
generate_random() {
    echo $((RANDOM % 1000000 + 100000))
}

# Function to check if installation is complete
is_installed() {
    [ -f "$TDZ_FILE" ] && [ -d "venv" ] && command_exists ollama
}

# Function to install Ollama
install_ollama() {
    if command_exists ollama; then
        print_success "Ollama already installed"
        return
    fi

    print_status "Installing Ollama..."

    if command_exists curl; then
        curl -fsSL https://ollama.com/install.sh | sh
        print_success "Ollama installed"
    else
        print_error "curl not found. Please install curl first."
        exit 1
    fi
}

# Function to check and pull Ollama model
setup_model() {
    print_status "Checking AI models..."

    # Check if any models are available
    if ollama list 2>/dev/null | grep -q ":"; then
        print_success "AI models already available"
        # Get the first available model
        MODEL=$(ollama list 2>/dev/null | grep ":" | head -1 | awk '{print $1}')
        if [ -n "$MODEL" ]; then
            echo "$MODEL" > "$CHAT_DIR/.model"
            print_success "Using model: $MODEL"
        fi
        return
    fi

    print_status "No models found. Pulling AI model..."

    # Try models in order of preference
    MODELS=("qwen3:0.6b" "deepseek-r1:1.5b" "smollm2:135m" "llama3.2:1b")

    for model in "${MODELS[@]}"; do
        print_status "Trying to pull $model..."
        if ollama pull "$model" 2>/dev/null; then
            print_success "Successfully pulled $model"
            echo "$model" > "$CHAT_DIR/.model"
            return
        else
            print_warning "Failed to pull $model, trying next..."
        fi
    done

    print_error "Failed to pull any model. Please check your internet connection."
    exit 1
}

# Function to setup Python environment
setup_python() {
    print_status "Setting up Python environment..."

    # Check if virtual environment exists
    if [ ! -d "venv" ]; then
        print_status "Creating virtual environment..."
        python3 -m venv venv
    fi

    # Activate virtual environment
    source venv/bin/activate

    # Upgrade pip
    pip install --upgrade pip >/dev/null 2>&1

    # Install requirements
    if [ -f "requirements.txt" ]; then
        print_status "Installing Python dependencies..."
        pip install -r requirements.txt
    else
        print_error "requirements.txt not found!"
        exit 1
    fi

    print_success "Python environment ready"
}

# Function to build Todozi bindings
build_todozi() {
    print_status "Building Todozi Python bindings..."

    cd "$PROJECT_ROOT"
    if cargo build --release; then
        cd python
        pip install .
        cd "$CHAT_DIR"
        print_success "Todozi bindings built and installed"
    else
        print_error "Failed to build Todozi bindings"
        cd "$CHAT_DIR"
        exit 1
    fi
}

# Function to perform full installation
install() {
    print_status "Starting full installation..."

    install_ollama
    setup_model
    build_todozi
    setup_python

    # Create tdz file to mark installation complete
    generate_random > "$TDZ_FILE"
    print_success "Installation complete! Created $TDZ_FILE"
}

# Function to start the server
start() {
    # Check if already installed
    if ! is_installed; then
        print_warning "Installation not detected. Running install first..."
        install
    fi

    # Activate virtual environment
    source venv/bin/activate

    # Check for API key
    if [ -z "$OLLAMA_API_KEY" ]; then
        print_warning "OLLAMA_API_KEY not set. AI responses may not work."
        print_status "Set it with: export OLLAMA_API_KEY='your-key-here'"
    fi

    # Start Ollama service in background if not running
    if ! pgrep -f "ollama serve" > /dev/null; then
        print_status "Starting Ollama service..."
        nohup ollama serve > "$CHAT_DIR/ollama.log" 2>&1 &
        sleep 2
    fi

    # Start the chat server
    print_success "Starting Todozi Chat Server on http://localhost:8275"
    export FLASK_ENV=development
    python chat.py
}

# Function to stop the server
stop() {
    print_status "Stopping services..."

    # Stop Flask server
    pkill -f "python chat.py" || print_warning "Chat server not running"

    # Stop Ollama service
    pkill -f "ollama serve" || print_warning "Ollama service not running"

    print_success "Services stopped"
}

# Function to show logs
logs() {
    echo "=== Ollama Logs ==="
    if [ -f "$CHAT_DIR/ollama.log" ]; then
        tail -f "$CHAT_DIR/ollama.log"
    else
        echo "No Ollama logs found"
    fi
}

# Function to show status
status() {
    echo "=== Todozi Chat Status ==="

    if is_installed; then
        print_success "Installation: Complete"
    else
        print_error "Installation: Incomplete"
    fi

    if command_exists ollama; then
        print_success "Ollama: Installed"
        if pgrep -f "ollama serve" > /dev/null; then
            print_success "Ollama Service: Running"
        else
            print_warning "Ollama Service: Not running"
        fi
    else
        print_error "Ollama: Not installed"
    fi

    if [ -d "venv" ]; then
        print_success "Python Environment: Ready"
        if [ -f "venv/bin/activate" ]; then
            source venv/bin/activate
            if python -c "import flask, ollama" 2>/dev/null; then
                print_success "Python Dependencies: Installed"
            else
                print_error "Python Dependencies: Missing"
            fi
        fi
    else
        print_error "Python Environment: Not ready"
    fi

    if pgrep -f "python chat.py" > /dev/null; then
        print_success "Chat Server: Running (PID: $(pgrep -f "python chat.py"))"
    else
        print_warning "Chat Server: Not running"
    fi

    if [ -f "$CHAT_DIR/.model" ]; then
        MODEL=$(cat "$CHAT_DIR/.model")
        print_success "AI Model: $MODEL"
    else
        # Check if any models are available via ollama list
        AVAILABLE_MODEL=$(ollama list 2>/dev/null | grep ":" | head -1 | awk '{print $1}')
        if [ -n "$AVAILABLE_MODEL" ]; then
            print_success "AI Model: $AVAILABLE_MODEL (available)"
        else
            print_warning "AI Model: None available"
        fi
    fi
}

# Function to show help
help() {
    echo "Todozi Chat Runner Script"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  start     Start the chat server"
    echo "  stop      Stop the chat server and Ollama"
    echo "  logs      Show Ollama logs"
    echo "  install   Perform full installation"
    echo "  status    Show current status"
    echo "  help      Show this help message"
    echo ""
    echo "If no command is provided, 'start' is assumed."
}

# Main script logic
cd "$CHAT_DIR"

case "${1:-start}" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    logs)
        logs
        ;;
    install)
        install
        ;;
    status)
        status
        ;;
    help|--help|-h)
        help
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        help
        exit 1
        ;;
esac
