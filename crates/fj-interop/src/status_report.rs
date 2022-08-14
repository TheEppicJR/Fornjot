//! Struct to store and update status messages

use std::collections::VecDeque;

/// Struct to store and update status messages
pub struct StatusReport {
    status: VecDeque<String>,
}

impl StatusReport {
    /// Create a new ``StatusReport`` instance with a blank status
    pub fn new() -> Self {
        Self {
            status: VecDeque::new(),
        }
    }

    /// Update the status
    pub fn update_status(&mut self, status: &str) {
        let status = format!("\n{}", status.to_owned());
        self.status.push_back(status);
        if self.status.len() > 5 {
            for _ in 0..(self.status.len() - 5) {
                self.status.pop_front();
            }
        }
    }

    /// Get current status
    pub fn status(&self) -> String {
        self.status
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>()
    }

    /// Reset status
    pub fn clear_status(&mut self) {
        self.status.clear();
    }
}

impl Default for StatusReport {
    fn default() -> Self {
        Self::new()
    }
}
