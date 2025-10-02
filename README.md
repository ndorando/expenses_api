# Expenses API

A Rust-based expense tracking API designed as a learning project to explore modern software architecture patterns and idiomatic Rust development.

## 🎯 Learning Objectives

This project serves as a comprehensive training ground for:

- **Rust Ecosystem**: Mastering core Rust concepts and idiomatic patterns
- **Async Programming**: Working with Tokio for asynchronous operations
- **Web API Development**: Building RESTful APIs with Axum framework
- **Serialization**: Using Serde for JSON serialization/deserialization
- **Error Handling**: Implementing robust error handling with ThisError
- **Modern API Design**: Creating clean, maintainable API interfaces

## 🏗️ Architectural Patterns

This project demonstrates several key architectural patterns:

### Hexagonal Architecture (Ports & Adapters)
The codebase is organized around the hexagonal architecture pattern:
- **Domain Layer** (`src/domain/`): Core business logic and entities
- **Service Layer** (`src/service/`): Application services and use cases
- **Repository Layer** (`src/repository/`): Data access abstractions
- **API Layer** (`src/api/`): External interface adapters

### CQRS (Command Query Responsibility Segregation)
Commands and queries are separated:
- **Commands** (`src/service/command/`): Handle write operations (create, update, delete)
- **Queries** (`src/service/query/`): Handle read operations (get, search)
- **Separate Repositories**: Read and write repositories for different optimization needs

### SOLID Principles
The implementation follows SOLID principles:
- **Single Responsibility**: Each module has a clear, focused purpose
- **Open/Closed**: Extensible through trait-based abstractions
- **Liskov Substitution**: Repository traits allow for different implementations
- **Interface Segregation**: Focused traits like `ExpenseEntryReadPort` and `ExpenseEntryWritePort`
- **Dependency Inversion**: High-level modules depend on abstractions, not concretions

## 🛠️ Technology Stack

- **Runtime**: Tokio (async runtime)
- **Web Framework**: Axum (modern, ergonomic web framework)
- **Serialization**: Serde (JSON serialization/deserialization)
- **Error Handling**: ThisError (derive-based error types)
- **UUID**: UUID v4 generation and serialization
- **DateTime**: Chrono for date/time handling

## 📁 Project Structure

```
src/
├── api/                    # API layer (adapters)
│   ├── routes.rs          # Route configuration
│   ├── expense_entry.rs   # Expense entry endpoints
│   ├── cost_bearer.rs     # Cost bearer endpoints
│   └── expense_type.rs    # Expense type endpoints
├── domain/                # Domain layer (business logic)
│   ├── expense_entry.rs   # Core expense entry entity
│   ├── cost_bearer.rs     # Cost bearer entity
│   ├── cost_share.rs      # Cost sharing logic
│   └── expense_type.rs    # Expense type entity
├── service/               # Application services
│   ├── command/           # Write operations (CQRS)
│   ├── query/             # Read operations (CQRS)
│   ├── expense_entry.rs   # Expense entry service
│   ├── cost_bearer.rs     # Cost bearer service
│   └── expense_type.rs    # Expense type service
├── repository/            # Data access layer
│   └── sqliterepository/  # SQLite implementation
└── test_util/             # Testing utilities
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (2024 edition)
- Cargo

### Running the Application

```bash
# Clone the repository
git clone <repository-url>
cd expenses_api

# Build and run
cargo run
```

The API will be available at `http://localhost:6570`

### Available Endpoints

- `POST /expense_entries` - Create expense entry
- `GET /expense_entries/{id}` - Get expense entry
- `PATCH /expense_entries/{id}` - Update expense entry
- `DELETE /expense_entries/{id}` - Delete expense entry

- `POST /cost_bearers` - Create cost bearer
- `GET /cost_bearers/{id}` - Get cost bearer
- `PATCH /cost_bearers/{id}` - Update cost bearer
- `DELETE /cost_bearers/{id}` - Delete cost bearer

- `POST /expense_types` - Create expense type
- `GET /expense_types/{id}` - Get expense type
- `PATCH /expense_types/{id}` - Update expense type
- `DELETE /expense_types/{id}` - Delete expense type

## 🧪 Testing

```bash
# Run tests
cargo test

# Run tests with test utilities
cargo test --features test-utils
```

## 📚 Key Learning Areas

### Rust-Specific Patterns
- **Ownership & Borrowing**: Careful memory management without garbage collection
- **Error Handling**: Using `Result<T, E>` and custom error types
- **Traits**: Defining contracts and abstractions
- **Async/Await**: Asynchronous programming with Tokio
- **Serde**: Serialization with derive macros

### Architectural Benefits
- **Testability**: Clear separation of concerns enables easy unit testing
- **Maintainability**: Modular structure makes changes isolated
- **Extensibility**: New features can be added without modifying existing code
- **Scalability**: CQRS pattern allows for read/write optimization
- **Flexibility**: Repository pattern enables easy database switching

## 🔄 Development Status

This is a **work-in-progress training project**. The implementation demonstrates architectural patterns and Rust idioms while being intentionally incomplete to focus on learning objectives.
