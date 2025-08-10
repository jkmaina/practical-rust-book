// tests/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_add_task() {
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    
    cmd.arg("add")
       .arg("Test task")
       .arg("--priority")
       .arg("high");
       
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Added task"));
}

#[test]
fn test_list_tasks() {
    // First add a task
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("add").arg("List test task").assert().success();
    
    // Then list tasks
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("list");
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("List test task"));
}

#[test]
fn test_complete_task() {
    // First add a task
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("add").arg("Complete test task").assert().success();
    
    // Then complete it (with auto-confirmation for testing)
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("complete")
       .arg("0")
       .env("TASKRUST_TEST", "1"); // Environment variable to skip confirmation in tests
       
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Completed task"));
       
    // Verify it's marked as completed in the list
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("list").arg("--completed");
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Complete test task"));
}

#[test]
fn test_export_import() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("export_test.json");
    let file_path_str = file_path.to_str().unwrap();
    
    // Add a task
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("add").arg("Export test task").assert().success();
    
    // Export tasks
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("export").arg(file_path_str);
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Exported tasks"));
       
    // Import tasks to verify
    let mut cmd = Command::cargo_bin("taskrust").unwrap();
    cmd.arg("import").arg(file_path_str);
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Imported tasks"));
}