#!/bin/bash
# Complete TaskRust System Demo
# This script demonstrates the full client-server interaction

echo "🚀 TaskRust Complete System Demo"
echo "================================="
echo ""

echo "This demo will show the complete TaskRust ecosystem:"
echo "1. TaskRust API Server (REST API)"
echo "2. TaskRust Client (CLI application)"
echo ""

echo "⚠️  Prerequisites:"
echo "- Make sure the TaskRust API server is running:"
echo "  cd taskrust-api && cargo run"
echo ""

SERVER_URL="http://localhost:8080"
CLIENT_PATH="taskrust-client"

# Test if server is running
echo "🔍 Checking if server is running..."
if curl -s --connect-timeout 5 "$SERVER_URL/" > /dev/null 2>&1; then
    echo "✅ Server is running!"
else
    echo "❌ Server is not running. Please start it with:"
    echo "   cd taskrust-api && cargo run"
    echo ""
    echo "Press Enter to exit..."
    read
    exit 1
fi

echo ""
echo "🎯 Starting Client Demo..."
echo ""

# Demo 1: Show API info
echo "1️⃣  Getting API Information"
echo "Command: cargo run -- info"
cd "$CLIENT_PATH"
cargo run -- info
echo ""

# Demo 2: List initial tasks
echo "2️⃣  Listing Initial Tasks (Sample Data)"
echo "Command: cargo run -- list"
cargo run -- list
echo ""

# Demo 3: Create a new task
echo "3️⃣  Creating a New Task"
echo "Command: cargo run -- create \"Test the complete system\" -d \"Verify client-server communication\" -p high"
CREATE_OUTPUT=$(cargo run -- create "Test the complete system" -d "Verify client-server communication" -p high 2>&1)
echo "$CREATE_OUTPUT"
echo ""

# Demo 4: List tasks again to see the new one
echo "4️⃣  Listing All Tasks (Including New One)"
echo "Command: cargo run -- list"
cargo run -- list
echo ""

# Demo 5: Create another task with due date
echo "5️⃣  Creating Task with Due Date"
echo "Command: cargo run -- create \"Prepare demo presentation\" -p medium --due 2025-02-15"
cargo run -- create "Prepare demo presentation" -p medium --due 2025-02-15
echo ""

# Demo 6: Get a specific task
echo "6️⃣  Getting Specific Task Details"
echo "Getting details for the first task..."

# Get the task list to extract an ID (simplified approach)
TASK_LIST=$(cargo run -- list 2>/dev/null)
if echo "$TASK_LIST" | grep -q "\["; then
    echo "Note: In real usage, copy the full UUID from the list command"
    echo "Command example: cargo run -- get <full-uuid>"
else
    echo "No tasks found to demonstrate get command"
fi
echo ""

# Demo 7: Update a task
echo "7️⃣  Updating a Task"
echo "Note: In real usage, you would use the full UUID from the list command"
echo "Command example: cargo run -- update <full-uuid> --completed true"
echo ""

# Demo 8: Show help
echo "8️⃣  Showing Available Commands"
echo "Command: cargo run -- --help"
cargo run -- --help
echo ""

# Demo 9: Show command-specific help
echo "9️⃣  Showing Create Command Help"
echo "Command: cargo run -- create --help"
cargo run -- create --help
echo ""

# Final summary
echo "🎉 Demo Complete!"
echo ""
echo "Summary of what we demonstrated:"
echo "✅ API server communication"
echo "✅ Task listing with colored output"
echo "✅ Task creation with various options"
echo "✅ Error handling and validation"
echo "✅ User-friendly CLI interface"
echo ""

echo "🔧 Next Steps:"
echo "1. Try updating tasks: cargo run -- update <task-id> --completed true"
echo "2. Try deleting tasks: cargo run -- delete <task-id>"
echo "3. Try the complete command: cargo run -- complete <task-id>"
echo "4. Experiment with different priorities and due dates"
echo ""

echo "📚 The complete TaskRust ecosystem includes:"
echo "• RESTful API server (taskrust-api)"
echo "• Command-line client (taskrust-client)"
echo "• Shared data models for consistency"
echo "• Comprehensive error handling"
echo "• Type-safe HTTP communication"
echo ""

cd ..
echo "Demo completed! 🎊"