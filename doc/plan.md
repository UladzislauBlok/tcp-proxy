## Strategic Development Plan: Simple TCP Proxy

This plan outlines the phases for developing a simple TCP proxy, moving from a basic socket logger to a fully functional, asynchronous proxy.

---

### Phase 1: Basic Server and Socket Data Logging

**Goal:** Create a synchronous server that listens on a port, accepts client connections, and logs the raw incoming data to the console.

1. Initialize the project and set up a minimal TCP listener.
2. Accept a connection, read the raw incoming client data, and print it.
3. Close the connection when the client terminates it.

---

### Phase 2: Basic Synchronous Proxy

**Goal:** Implement a basic, synchronous proxy function.

1. Read the target address (host and port) from CLI options or a configuration file.
2. Wait for a client connection, then open a socket channel between the client and the target server.
3. Forward data bidirectionally until either the client or the target closes their connection, then close the remaining connection.
4. For testing purposes, create a simple HTTP or generic TCP server to act as the target.

---

### Phase 3: Command-Line Interface (CLI) Tool

**Goal:** Develop a CLI tool for visibility (e.g., checking the number of open connections) and management (e.g., changing configuration, shutting down the proxy).

1. Create the CLI tool structure.
2. Read the command entered by the client (user) from the CLI.
3. Send this command to the proxy using a dedicated management port (separate from the client-facing proxy port).

### Phase 4: Enhancements and Advanced Features

**Goal:** Implement advanced, "fancy" features.

1. TLS/SSL termination (handling secure connections).
2. Implement an asynchronous design using a small number of threads, instead of the less scalable "two threads per connection" model.
3. Add throttling capabilities (rate limiting).
