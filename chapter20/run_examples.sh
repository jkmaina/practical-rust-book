#!/bin/bash
# TaskRust Examples Runner
# Provides easy commands to run different parts of the TaskRust system

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_color() {
    printf "${1}${2}${NC}\n"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if server is running
check_server() {
    if curl -s --connect-timeout 2 http://localhost:8080/ > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to start the server
start_server() {
    print_color $CYAN "🚀 Starting TaskRust API Server..."
    cd taskrust-api
    
    if check_server; then
        print_color $YELLOW "⚠️  Server is already running on http://localhost:8080"
        cd ..
        return 0
    fi
    
    print_color $BLUE "Building and starting server..."
    cargo run &
    SERVER_PID=$!
    
    # Wait for server to start
    print_color $CYAN "⏳ Waiting for server to start..."
    for i in {1..30}; do
        if check_server; then
            print_color $GREEN "✅ Server started successfully!"
            print_color $BLUE "📡 Server running at: http://localhost:8080"
            print_color $BLUE "🆔 Server PID: $SERVER_PID"
            echo $SERVER_PID > ../server.pid
            cd ..
            return 0
        fi
        sleep 1
    done
    
    print_color $RED "❌ Failed to start server"
    cd ..
    return 1
}

# Function to stop the server
stop_server() {
    print_color $CYAN "🛑 Stopping TaskRust API Server..."
    
    if [ -f server.pid ]; then
        SERVER_PID=$(cat server.pid)
        if kill -0 $SERVER_PID 2>/dev/null; then
            kill $SERVER_PID
            rm server.pid
            print_color $GREEN "✅ Server stopped successfully"
        else
            print_color $YELLOW "⚠️  Server was not running"
            rm server.pid
        fi
    else
        # Try to find and kill any running cargo process for taskrust-api
        pkill -f "taskrust-api" 2>/dev/null || true
        print_color $YELLOW "⚠️  Attempted to stop any running server processes"
    fi
}

# Function to run the client
run_client() {
    print_color $CYAN "🖥️  Running TaskRust Client..."
    
    if ! check_server; then
        print_color $RED "❌ Server is not running. Please start it first with: $0 server"
        return 1
    fi
    
    cd taskrust-client
    
    if [ $# -eq 0 ]; then
        print_color $BLUE "📋 Listing all tasks..."
        cargo run -- list
    else
        print_color $BLUE "🔧 Running client command: $*"
        cargo run -- "$@"
    fi
    
    cd ..
}

# Function to run API tests
test_api() {
    print_color $CYAN "🧪 Testing TaskRust API..."
    
    if ! check_server; then
        print_color $RED "❌ Server is not running. Please start it first with: $0 server"
        return 1
    fi
    
    if [ -f taskrust-api/test_api.sh ]; then
        cd taskrust-api
        chmod +x test_api.sh
        ./test_api.sh
        cd ..
    else
        print_color $RED "❌ API test script not found"
        return 1
    fi
}

# Function to run the complete demo
run_demo() {
    print_color $CYAN "🎬 Running Complete System Demo..."
    
    if ! check_server; then
        print_color $RED "❌ Server is not running. Please start it first with: $0 server"
        return 1
    fi
    
    if [ -f demo_system.sh ]; then
        chmod +x demo_system.sh
        ./demo_system.sh
    else
        print_color $RED "❌ Demo script not found"
        return 1
    fi
}

# Function to build everything
build_all() {
    print_color $CYAN "🔨 Building TaskRust System..."
    
    print_color $BLUE "Building API server..."
    cd taskrust-api
    cargo build
    cd ..
    
    print_color $BLUE "Building client..."
    cd taskrust-client
    cargo build
    cd ..
    
    print_color $GREEN "✅ Build completed successfully!"
}

# Function to show help
show_help() {
    print_color $BLUE "TaskRust Examples Runner"
    print_color $BLUE "======================="
    echo ""
    echo "Usage: $0 <command> [arguments]"
    echo ""
    echo "Commands:"
    echo "  server              Start the TaskRust API server"
    echo "  stop                Stop the TaskRust API server"
    echo "  client [args]       Run the TaskRust client (defaults to 'list')"
    echo "  test                Run API tests"
    echo "  demo                Run the complete system demo"
    echo "  build               Build both server and client"
    echo "  status              Check server status"
    echo "  help                Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 server                                    # Start the server"
    echo "  $0 client                                    # List all tasks"
    echo "  $0 client create \"My task\" -p high          # Create a high priority task"
    echo "  $0 client info                               # Show API information"
    echo "  $0 test                                      # Run API tests"
    echo "  $0 demo                                      # Run complete demo"
    echo "  $0 stop                                      # Stop the server"
    echo ""
    print_color $YELLOW "💡 Tip: Start the server first, then run client commands or tests"
}

# Function to check status
check_status() {
    print_color $CYAN "📊 TaskRust System Status"
    print_color $CYAN "========================="
    
    if check_server; then
        print_color $GREEN "✅ API Server: Running (http://localhost:8080)"
    else
        print_color $RED "❌ API Server: Not running"
    fi
    
    if [ -f server.pid ]; then
        SERVER_PID=$(cat server.pid)
        if kill -0 $SERVER_PID 2>/dev/null; then
            print_color $BLUE "🆔 Server PID: $SERVER_PID"
        else
            print_color $YELLOW "⚠️  Stale PID file found, cleaning up..."
            rm server.pid
        fi
    fi
    
    # Check if required tools are available
    echo ""
    print_color $CYAN "🔧 System Requirements:"
    
    if command_exists cargo; then
        print_color $GREEN "✅ Cargo: Available"
    else
        print_color $RED "❌ Cargo: Not found"
    fi
    
    if command_exists curl; then
        print_color $GREEN "✅ curl: Available"
    else
        print_color $RED "❌ curl: Not found (needed for API tests)"
    fi
    
    if command_exists jq; then
        print_color $GREEN "✅ jq: Available"
    else
        print_color $YELLOW "⚠️  jq: Not found (optional, for better JSON formatting)"
    fi
}

# Main script logic
case "${1:-help}" in
    "server")
        start_server
        ;;
    "stop")
        stop_server
        ;;
    "client")
        shift
        run_client "$@"
        ;;
    "test")
        test_api
        ;;
    "demo")
        run_demo
        ;;
    "build")
        build_all
        ;;
    "status")
        check_status
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        print_color $RED "❌ Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac