# Order Inventory Management Backend

A production-style Rust backend service for managing products, inventory, and customer orders.

Built with:

- Rust
- Axum
- PostgreSQL
- SQLx
- WebSockets
- Docker
- thiserror
- tracing

## Features

- Create products
- List products
- Get product by ID
- Update product stock
- Create orders
- List orders
- Get order by ID
- Update order status
- Real-time WebSocket events
- PostgreSQL transactions for stock deduction
- Structured JSON responses
- Domain-specific error handling
- Docker Compose setup

## Run locally

```bash
cp .env.example .env
docker compose up --build
```

## Health check

```bash
curl http://localhost:3000/health
```


## Readiness check

```bash
curl http://localhost:3000/ready
```

## Create product

```bash
curl -X POST http://localhost:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Pro",
    "sku": "MBP-001",
    "price": 250000,
    "stock": 10
  }'
```

## List products
```bash
curl http://localhost:3000/products
```

## Create order
```bash
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "customer_name": "Sudhar",
    "product_id": "PRODUCT_ID_HERE",
    "quantity": 2
  }'
```

## WebSocket events
```bash
wscat -c ws://localhost:3000/ws
```

### events emittee
```bash
{
  "event": "product_created"
}
```

```bash
{
  "event": "order_created"
}
```

```bash
{
  "event": "product_stock_updated"
}
```

```bash
{
  "event": "order_status_updated"
}
```

## API response format
```bash
{
  "success": true,
  "message": "Product created successfully",
  "data": {}
}
```

```bash
{
  "success": false,
  "message": "Product not found"
}
```

## Architecture
``` plaintext
src/
  config.rs
  error.rs
  main.rs
  models.rs
  response.rs
  routes.rs
  state.rs

  handlers/
    health_handler.rs
    product_handler.rs
    order_handler.rs
    ws_handler.rs

  repositories/
    product_repository.rs
    order_repository.rs
```

## Why this project exists

This project demonstrates backend engineering concepts in Rust:

* Modular API design
* REST endpoint design
* WebSocket event broadcasting
* PostgreSQL transactions
* Repository pattern
* Dockerized development
* Clean error management
* Structured logging