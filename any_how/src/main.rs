#![allow(unused)]
use anyhow::{self, Context, Result};
use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum MeroError {
    #[error("file not found {0}")]
    NotFound(String),

    #[error("permission error: you have no permission to access that file")]
    PermissionError(String),
}

fn might_not_found() -> Result<()> {
    Err(MeroError::NotFound("file not found".into()))
        .with_context(|| format!("could not find the file "))?;
    Ok(())
}
fn might_not_have_permission() -> Result<()> {
    Err(MeroError::PermissionError("no permission".into()))
        .with_context(|| format!("you dont not have permission to read this file "))?;
    Ok(())
}

fn main() {
    let errors = vec![might_not_found(), might_not_have_permission()];
    for result in errors {
        match result {
            Ok(_) => println!("success"),
            Err(e) => {
                println!("Error: {:?}", e);

                if let Some(mero_error) = e.downcast_ref::<MeroError>() {
                    match mero_error {
                        MeroError::NotFound(message) => println!("Not Found: {}", message),
                        MeroError::PermissionError(message) => {
                            println!("Permission Error: {}", message)
                        }
                    }
                } else {
                    println!("Unknown Error: {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_might_not_found_returns_not_found_error_with_context() {
        let err = might_not_found().unwrap_err();

        // Ensure we can downcast to our MeroError and it's the NotFound variant
        if let Some(MeroError::NotFound(message)) = err.downcast_ref::<MeroError>() {
            assert!(message.contains("file not found"));
        } else {
            panic!("expected MeroError::NotFound, got: {:?}", err);
        }

        // Ensure the context we attached is present in the display
        let s = format!("{}", err);
        assert!(s.contains("could not find the file"));
    }

    #[test]
    fn test_might_not_have_permission_returns_permission_error_with_context() {
        let err = might_not_have_permission().unwrap_err();

        if let Some(MeroError::PermissionError(message)) = err.downcast_ref::<MeroError>() {
            assert!(message.contains("no permission"));
        } else {
            panic!("expected MeroError::PermissionError, got: {:?}", err);
        }

        let s = format!("{}", err);
        assert!(s.contains("you dont not have permission to read this file"));
    }
}
