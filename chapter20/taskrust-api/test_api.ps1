# PowerShell script to test the TaskRust API
# Run this after starting the server with: cargo run

Write-Host "🧪 Testing TaskRust API" -ForegroundColor Green
Write-Host "Make sure the server is running with: cargo run" -ForegroundColor Yellow
Write-Host ""

$baseUrl = "http://localhost:8080"

# Test 1: Get API info
Write-Host "1. Getting API information..." -ForegroundColor Cyan
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/" -Method Get
    Write-Host "✅ API Info retrieved successfully" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} catch {
    Write-Host "❌ Failed to get API info: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 2: List all tasks (should show sample data)
Write-Host "2. Listing all tasks..." -ForegroundColor Cyan
try {
    $tasks = Invoke-RestMethod -Uri "$baseUrl/tasks" -Method Get
    Write-Host "✅ Tasks retrieved successfully. Found $($tasks.Count) tasks:" -ForegroundColor Green
    $tasks | ForEach-Object { 
        Write-Host "  - $($_.title) (Priority: $($_.priority), ID: $($_.id))" -ForegroundColor White
    }
} catch {
    Write-Host "❌ Failed to get tasks: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 3: Create a new task
Write-Host "3. Creating a new task..." -ForegroundColor Cyan
$newTask = @{
    title = "Test API endpoints"
    description = "Verify all CRUD operations work correctly"
    priority = "High"
} | ConvertTo-Json

try {
    $createdTask = Invoke-RestMethod -Uri "$baseUrl/tasks" -Method Post -Body $newTask -ContentType "application/json"
    Write-Host "✅ Task created successfully:" -ForegroundColor Green
    Write-Host "  ID: $($createdTask.id)" -ForegroundColor White
    Write-Host "  Title: $($createdTask.title)" -ForegroundColor White
    Write-Host "  Priority: $($createdTask.priority)" -ForegroundColor White
    $taskId = $createdTask.id
} catch {
    Write-Host "❌ Failed to create task: $($_.Exception.Message)" -ForegroundColor Red
    return
}

Write-Host "`n" + "="*50 + "`n"

# Test 4: Get the specific task
Write-Host "4. Getting the created task by ID..." -ForegroundColor Cyan
try {
    $task = Invoke-RestMethod -Uri "$baseUrl/tasks/$taskId" -Method Get
    Write-Host "✅ Task retrieved successfully:" -ForegroundColor Green
    Write-Host "  Title: $($task.title)" -ForegroundColor White
    Write-Host "  Description: $($task.description)" -ForegroundColor White
    Write-Host "  Completed: $($task.completed)" -ForegroundColor White
} catch {
    Write-Host "❌ Failed to get task: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 5: Update the task
Write-Host "5. Updating the task..." -ForegroundColor Cyan
$updateTask = @{
    completed = $true
    priority = "Medium"
} | ConvertTo-Json

try {
    $updatedTask = Invoke-RestMethod -Uri "$baseUrl/tasks/$taskId" -Method Put -Body $updateTask -ContentType "application/json"
    Write-Host "✅ Task updated successfully:" -ForegroundColor Green
    Write-Host "  Completed: $($updatedTask.completed)" -ForegroundColor White
    Write-Host "  Priority: $($updatedTask.priority)" -ForegroundColor White
} catch {
    Write-Host "❌ Failed to update task: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 6: List all tasks again (should show the updated task)
Write-Host "6. Listing all tasks again..." -ForegroundColor Cyan
try {
    $tasks = Invoke-RestMethod -Uri "$baseUrl/tasks" -Method Get
    Write-Host "✅ Tasks retrieved successfully. Found $($tasks.Count) tasks:" -ForegroundColor Green
    $tasks | ForEach-Object { 
        $status = if ($_.completed) { "✅" } else { "⏳" }
        Write-Host "  $status $($_.title) (Priority: $($_.priority))" -ForegroundColor White
    }
} catch {
    Write-Host "❌ Failed to get tasks: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 7: Delete the task
Write-Host "7. Deleting the created task..." -ForegroundColor Cyan
try {
    Invoke-RestMethod -Uri "$baseUrl/tasks/$taskId" -Method Delete
    Write-Host "✅ Task deleted successfully" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to delete task: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n" + "="*50 + "`n"

# Test 8: Try to get the deleted task (should return 404)
Write-Host "8. Trying to get the deleted task (should fail)..." -ForegroundColor Cyan
try {
    $task = Invoke-RestMethod -Uri "$baseUrl/tasks/$taskId" -Method Get
    Write-Host "❌ Task still exists (this shouldn't happen)" -ForegroundColor Red
} catch {
    Write-Host "✅ Task not found (expected behavior)" -ForegroundColor Green
    Write-Host "  Error: $($_.Exception.Message)" -ForegroundColor Gray
}

Write-Host "`n🎉 API testing completed!" -ForegroundColor Green
Write-Host "All CRUD operations have been tested." -ForegroundColor White