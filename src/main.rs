// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _listener = TcpListener::bind("127.0.0.1:1337").await?;

    Ok(())
}
