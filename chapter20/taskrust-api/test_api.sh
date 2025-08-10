#!/bin/bash
# Bash script to test the TaskRust API
# Run this after starting the server with: cargo run

echo "🧪 Testing TaskRust API"
echo "Make sure the server is running with: cargo run"
echo ""

BASE_URL="http://localhost:8080"

# Test 1: Get API info
echo "1. Getting API information..."
if curl -s "$BASE_URL/" | jq . > /dev/null 2>&1; then
    echo "✅ API Info retrieved successfully"
    curl -s "$BASE_URL/" | jq .
else
    echo "❌ Failed to get API info or jq not installed"
    curl -s "$BASE_URL/"
fi

echo ""
echo "=================================================="
echo ""

# Test 2: List all tasks (should show sample data)
echo "2. Listing all tasks..."
if curl -s "$BASE_URL/tasks" | jq . > /dev/null 2>&1; then
    echo "✅ Tasks retrieved successfully:"
    curl -s "$BASE_URL/tasks" | jq -r '.[] | "  - \(.title) (Priority: \(.priority), ID: \(.id))"'
else
    echo "❌ Failed to get tasks or jq not installed"
    curl -s "$BASE_URL/tasks"
fi

echo ""
echo "=================================================="
echo ""

# Test 3: Create a new task
echo "3. Creating a new task..."
NEW_TASK='{"title": "Test API endpoints", "description": "Verify all CRUD operations work correctly", "priority": "High"}'

if TASK_RESPONSE=$(curl -s -X POST "$BASE_URL/tasks" \
    -H "Content-Type: application/json" \
    -d "$NEW_TASK"); then
    
    if echo "$TASK_RESPONSE" | jq . > /dev/null 2>&1; then
        echo "✅ Task created successfully:"
        TASK_ID=$(echo "$TASK_RESPONSE" | jq -r '.id')
        echo "  ID: $TASK_ID"
        echo "  Title: $(echo "$TASK_RESPONSE" | jq -r '.title')"
        echo "  Priority: $(echo "$TASK_RESPONSE" | jq -r '.priority')"
    else
        echo "✅ Task created successfully (raw response):"
        echo "$TASK_RESPONSE"
        # Try to extract ID with grep if jq is not available
        TASK_ID=$(echo "$TASK_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
    fi
else
    echo "❌ Failed to create task"
    exit 1
fi

echo ""
echo "=================================================="
echo ""

# Test 4: Get the specific task
echo "4. Getting the created task by ID..."
if curl -s "$BASE_URL/tasks/$TASK_ID" | jq . > /dev/null 2>&1; then
    echo "✅ Task retrieved successfully:"
    TASK_INFO=$(curl -s "$BASE_URL/tasks/$TASK_ID")
    echo "  Title: $(echo "$TASK_INFO" | jq -r '.title')"
    echo "  Description: $(echo "$TASK_INFO" | jq -r '.description')"
    echo "  Completed: $(echo "$TASK_INFO" | jq -r '.completed')"
else
    echo "❌ Failed to get task or jq not installed"
    curl -s "$BASE_URL/tasks/$TASK_ID"
fi

echo ""
echo "=================================================="
echo ""

# Test 5: Update the task
echo "5. Updating the task..."
UPDATE_TASK='{"completed": true, "priority": "Medium"}'

if UPDATED_TASK=$(curl -s -X PUT "$BASE_URL/tasks/$TASK_ID" \
    -H "Content-Type: application/json" \
    -d "$UPDATE_TASK"); then
    
    if echo "$UPDATED_TASK" | jq . > /dev/null 2>&1; then
        echo "✅ Task updated successfully:"
        echo "  Completed: $(echo "$UPDATED_TASK" | jq -r '.completed')"
        echo "  Priority: $(echo "$UPDATED_TASK" | jq -r '.priority')"
    else
        echo "✅ Task updated successfully (raw response):"
        echo "$UPDATED_TASK"
    fi
else
    echo "❌ Failed to update task"
fi

echo ""
echo "=================================================="
echo ""

# Test 6: List all tasks again
echo "6. Listing all tasks again..."
if curl -s "$BASE_URL/tasks" | jq . > /dev/null 2>&1; then
    echo "✅ Tasks retrieved successfully:"
    curl -s "$BASE_URL/tasks" | jq -r '.[] | if .completed then "  ✅ \(.title) (Priority: \(.priority))" else "  ⏳ \(.title) (Priority: \(.priority))" end'
else
    echo "❌ Failed to get tasks or jq not installed"
    curl -s "$BASE_URL/tasks"
fi

echo ""
echo "=================================================="
echo ""

# Test 7: Delete the task
echo "7. Deleting the created task..."
if curl -s -X DELETE "$BASE_URL/tasks/$TASK_ID" -w "%{http_code}" | grep -q "204"; then
    echo "✅ Task deleted successfully"
else
    echo "❌ Failed to delete task"
fi

echo ""
echo "=================================================="
echo ""

# Test 8: Try to get the deleted task (should return 404)
echo "8. Trying to get the deleted task (should fail)..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/tasks/$TASK_ID")
if [ "$HTTP_CODE" = "404" ]; then
    echo "✅ Task not found (expected behavior)"
    echo "  HTTP Status: $HTTP_CODE"
else
    echo "❌ Task still exists (this shouldn't happen)"
    echo "  HTTP Status: $HTTP_CODE"
fi

echo ""
echo "🎉 API testing completed!"
echo "All CRUD operations have been tested."

# Check if jq is available and suggest installation if not
if ! command -v jq &> /dev/null; then
    echo ""
    echo "💡 Tip: Install 'jq' for better JSON formatting:"
    echo "  Ubuntu/Debian: sudo apt-get install jq"
    echo "  macOS: brew install jq"
    echo "  CentOS/RHEL: sudo yum install jq"
fi