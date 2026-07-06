# rustHTTP

<div align="center">

### A High-Performance, Multithreaded HTTP Server Built from Scratch in Rust

*Learning systems programming by implementing a production-inspired HTTP server from first principles.*

![Rust](https://img.shields.io/badge/Rust-1.89+-orange?style=for-the-badge\&logo=rust)
![Status](https://img.shields.io/badge/Status-In%20Development-success?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)
![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-brightgreen?style=for-the-badge)

</div>

---

## Overview

**rustHTTP** is a production-inspired HTTP server written entirely in Rust.

The purpose of this project is **not** to build another web framework. Instead, it focuses on understanding how modern web servers work internally by implementing networking, HTTP parsing, routing, concurrency, middleware, and performance optimizations from scratch.

This repository serves as both a learning journey and a systems programming project, documenting each architectural decision along the way.

---

## Why This Project?

Most frameworks hide the complexity of networking behind abstractions.

**rustHTTP** does the opposite.

Instead of relying on existing web frameworks, every major component is built manually to understand how production-grade HTTP servers operate internally.

Topics explored throughout this project include:

* TCP Socket Programming
* HTTP/1.1 Protocol
* Request Parsing
* Response Generation
* Routing
* Thread Pools
* Async Programming
* Middleware
* Static File Serving
* Performance Optimization
* Benchmarking
* Concurrent Programming
* Clean Software Architecture

---

# Objectives

* Build an HTTP server from scratch
* Learn systems programming with Rust
* Understand the HTTP protocol deeply
* Implement efficient multithreading
* Explore asynchronous networking with Tokio
* Build reusable server components
* Benchmark and optimize performance
* Document architectural decisions

---

# Features

## Networking

* TCP Listener
* TCP Stream Handling
* Connection Management
* Graceful Shutdown
* Connection Timeouts
* Persistent Connections

---

## HTTP

* HTTP/1.1
* Request Parsing
* Response Builder
* Header Parsing
* Query Parameters
* Cookies
* MIME Type Detection
* Status Codes
* Custom Headers

---

## Routing

Supports

* GET
* POST
* PUT
* PATCH
* DELETE

Example

```rust
router.get("/", home);

router.post("/login", login);

router.put("/users/:id", update_user);

router.delete("/users/:id", delete_user);
```

---

## Concurrency

* Thread Pool
* Worker Threads
* Work Queue
* Thread-safe Data Structures
* Async Networking
* Shared State Management

---

## Middleware

* Logging
* Authentication
* Rate Limiting
* Compression
* Request Timing
* CORS
* Custom Middleware

Execution Pipeline

```
Incoming Request
        │
        ▼
   TCP Listener
        │
        ▼
 HTTP Parser
        │
        ▼
  Middleware
        │
        ▼
    Router
        │
        ▼
 Request Handler
        │
        ▼
 Response Builder
        │
        ▼
Outgoing Response
```

---

## Static File Serving

Serve

* HTML
* CSS
* JavaScript
* Images
* Fonts
* Videos
* PDFs

---

## JSON APIs

Request

```json
{
    "username": "alice",
    "password": "secret"
}
```

Response

```json
{
    "status": "success"
}
```

---

## Logging

Every request logs

* Timestamp
* Client IP
* HTTP Method
* Route
* Status Code
* Latency
* User-Agent
* Response Size

---

## Error Handling

Implemented HTTP status codes

* 200 OK
* 201 Created
* 204 No Content
* 400 Bad Request
* 401 Unauthorized
* 403 Forbidden
* 404 Not Found
* 405 Method Not Allowed
* 408 Request Timeout
* 429 Too Many Requests
* 500 Internal Server Error

---

# Planned Features

* HTTP/2 Support
* HTTPS (TLS)
* WebSockets
* Reverse Proxy
* Load Balancer
* Gzip Compression
* Brotli Compression
* Chunked Transfer Encoding
* Streaming Responses
* Multipart File Uploads
* Session Management
* Reverse DNS Lookup
* Request Caching
* ETag Support
* Virtual Hosts
* Configuration Files
* Access Logs
* Metrics Dashboard

---

# Architecture

```
                          Client
                             │
                             │
                     TCP Connection
                             │
                             ▼
                      TCP Listener
                             │
                             ▼
                 Connection Acceptor
                             │
            ┌────────────────┴────────────────┐
            ▼                                 ▼
      Worker Thread 1                   Worker Thread 2
            │                                 │
            ▼                                 ▼
      HTTP Request Parser               HTTP Request Parser
            │                                 │
            ▼                                 ▼
          Router                          Router
            │                                 │
            ▼                                 ▼
      Middleware Stack                 Middleware Stack
            │                                 │
            ▼                                 ▼
       Route Handler                    Route Handler
            │                                 │
            ▼                                 ▼
      Response Builder                 Response Builder
            │                                 │
            └────────────────┬────────────────┘
                             ▼
                     HTTP Response
```

---

# Project Structure

```
rustHTTP
│
├── .github
│   ├── ISSUE_TEMPLATE
│   ├── workflows
│   │   ├── ci.yml
│   │   ├── clippy.yml
│   │   └── fmt.yml
│   └── pull_request_template.md
│
├── benches
├── docs
│   ├── architecture.md
│   ├── threadpool.md
│   ├── routing.md
│   ├── parser.md
│   └── benchmarks.md
│
├── examples
├── tests
│
├── src
│   ├── main.rs
│   ├── server.rs
│   ├── listener.rs
│   ├── router.rs
│   ├── parser.rs
│   ├── request.rs
│   ├── response.rs
│   ├── middleware.rs
│   ├── worker.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── handlers
│   │
│   └── utils
│
├── Cargo.toml
├── LICENSE
├── CONTRIBUTING.md
├── ROADMAP.md
└── README.md
```

---

# Development Roadmap

## Phase 1

* [ ] TCP Listener
* [ ] TCP Streams
* [ ] Connection Acceptance

---

## Phase 2

* [ ] HTTP Request Parser
* [ ] Request Structure
* [ ] Response Builder

---

## Phase 3

* [ ] Routing
* [ ] Route Parameters
* [ ] Method Matching

---

## Phase 4

* [ ] Thread Pool
* [ ] Worker Queue
* [ ] Graceful Shutdown

---

## Phase 5

* [ ] Middleware
* [ ] Logging
* [ ] Request Context

---

## Phase 6

* [ ] Static File Server
* [ ] MIME Detection

---

## Phase 7

* [ ] JSON Support
* [ ] Serialization

---

## Phase 8

* [ ] HTTP Keep-Alive
* [ ] Connection Reuse

---

## Phase 9

* [ ] Benchmarking
* [ ] Profiling
* [ ] Performance Optimization

---

## Phase 10

* [ ] HTTP/2
* [ ] HTTPS
* [ ] WebSockets

---

# Performance Goals

* 10,000+ Requests per Second
* Low Memory Consumption
* Low Latency
* Efficient Thread Scheduling
* Minimal Heap Allocations
* Fast HTTP Parsing

---

# Learning Outcomes

This project explores

* Ownership
* Borrowing
* Lifetimes
* Traits
* Generics
* Async Rust
* Tokio
* TCP Networking
* HTTP Protocol
* Thread Pools
* Synchronization
* Arc
* Mutex
* Channels
* Performance Profiling
* Benchmarking
* Software Architecture

---

# Benchmarking

Future benchmarks will compare **rustHTTP** against

* Hyper
* Axum
* Actix Web
* Go net/http
* Node.js HTTP
* Python Flask

Metrics

* Requests/sec
* Average Latency
* Peak Throughput
* CPU Usage
* Memory Usage

---

# Documentation

Detailed implementation notes will be available under the `docs/` directory, covering:

* HTTP Protocol
* TCP Networking
* Thread Pool Design
* Request Lifecycle
* Middleware Pipeline
* Routing Engine
* Performance Optimizations
* Benchmark Results

---

# Contributing

Contributions are welcome.

If you'd like to contribute:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a Pull Request

Please ensure that new features include appropriate tests and documentation.

---

# Learning Resources

* RFC 9112 — HTTP/1.1
* The Rust Programming Language
* Tokio Documentation
* Rustonomicon
* Hyper Source Code
* NGINX Source Code
* Caddy Source Code

---

# License

Distributed under the MIT License.

See `LICENSE` for more information.

---

## Vision

The long-term goal of **rustHTTP** is to evolve from a simple TCP server into a production-inspired HTTP server that demonstrates how modern web servers are built internally.

Rather than focusing solely on the final implementation, this project emphasizes understanding the engineering decisions behind each component, documenting trade-offs, and continuously improving performance through benchmarking and iterative development.

If this project helps others learn systems programming in Rust, then it has achieved its purpose.
