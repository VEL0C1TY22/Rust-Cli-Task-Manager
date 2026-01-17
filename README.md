# Cli Task Manager Made in Rust

# Features
• Add multiple tasks with unique UUIDs
• List all registered tasks with state tracking
• Execute tasks asynchronously using Tokio
• Cooperative task cancellation with CancellationToken
• Command-driven interactive CLI loop
• Non-blocking async runtime behavior

# Tech Stack
• Rust
• Tokio (async runtime)
• tokio-util (cancellation tokens)
• UUID (task identity)
• HashMap-based in-memory registry

# Example Commands
• add        → Register new tasks
• list       → Show all tasks
• execute    → Run a task by ID
• cancel     → Cancel and remove a task
• exit       → To exit the program
