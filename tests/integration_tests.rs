use std::fs;
use std::process::Command;
use std::path::Path;

// Helper function to clean up test directories
fn cleanup_test_dirs() {
    let _ = fs::remove_dir_all("test_added");
    let _ = fs::remove_dir_all("added");
    let _ = fs::remove_dir_all("../added");
}


#[test]
fn test_help_command_integration() {
    let output = Command::new("cargo")
        .args(&["run", "--", "help"])
        .output()
        .expect("Failed to execute help command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Listing available commands:"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("view"));
    assert!(stdout.contains("help"));
}

#[test]
fn test_list_command_integration() {
    cleanup_test_dirs();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute list command");

    // Should succeed even with no files
    assert!(output.status.success());
    
    cleanup_test_dirs();
}

#[test]
fn test_list_with_content_flag_integration() {
    cleanup_test_dirs();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "list", "-c"])
        .output()
        .expect("Failed to execute list -c command");

    // Should succeed even with no files
    assert!(output.status.success());
    
    cleanup_test_dirs();
}

#[test]
fn test_view_command_integration() {
    cleanup_test_dirs();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "view", "nonexistent"])
        .output()
        .expect("Failed to execute view command");

    // Should fail for non-existent file
    assert!(!output.status.success());
    
    cleanup_test_dirs();
}

#[test]
fn test_view_nonexistent_file_integration() {
    cleanup_test_dirs();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "view", "nonexistent"])
        .output()
        .expect("Failed to execute view command");

    // Should return an error (non-zero exit code)
    assert!(!output.status.success());
    
    cleanup_test_dirs();
}

#[test]
fn test_test_command_integration() {
    let output = Command::new("cargo")
        .args(&["run", "--", "test"])
        .output()
        .expect("Failed to execute test command");

    // Test command should succeed
    assert!(output.status.success());
}

#[test]
fn test_unknown_command_integration() {
    let output = Command::new("cargo")
        .args(&["run", "--", "unknown_command"])
        .output()
        .expect("Failed to execute unknown command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("command not recognizable"));
    assert!(stdout.contains("type `todo help` to see instructions"));
}

#[test]
fn test_no_arguments_integration() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute with no arguments");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("command not recognizable"));
    assert!(stdout.contains("type `todo help` to see instructions"));
}

#[test]
fn test_edit_command_not_implemented() {
    let output = Command::new("cargo")
        .args(&["run", "--", "edit"])
        .output()
        .expect("Failed to execute edit command");

    // Edit command should panic with todo!() macro
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not yet implemented"));
}

#[test]
fn test_view_with_missing_argument() {
    let output = Command::new("cargo")
        .args(&["run", "--", "view"])
        .output()
        .expect("Failed to execute view command without argument");

    // Should return an error due to missing argument
    assert!(!output.status.success());
}

#[test]
fn test_application_banner() {
    let output = Command::new("cargo")
        .args(&["run", "--", "help"])
        .output()
        .expect("Failed to execute help command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ToDo"));
    assert!(stdout.contains("Rust"));
    assert!(stdout.contains("___"));
}