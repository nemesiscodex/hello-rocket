Rocket example
===============

Code for youtube video [Rust web development | Boilerplate free with Rocket](https://youtu.be/tjH0Mye8U_A)

- Hello World example
- TODO service example with diesel + postgres

## Requirements
* Rust (nightly)
* Docker
* docker-compose

## Usage
```
# Run postgres
docker-compose up -d postgres

# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket diesel migration run

# Run the server
cargo run
```

# Routes
- `GET` `/hello` -> Hello World
  **Response:**
  ```
  Hello, World!
  ```
- `GET` `/hello/<name>` -> Hello Name
  **Response:**
  ```
  Hello, <name>!
  ```
- `GET` `/todos` -> Get TODOs
  **Response:**
  ```
  [{
      "id": 2,
      "title": "test2",
      "checked": false
  },
  {
      "id": 1,
      "title": "test1",
      "checked": true
  }]
  ```
- `POST` `/todos` -> Create TODO
  **Request Body:**
  ```
  {
    "title": "test3"    
  }
  ```
  **Response:**
  ```
  {
      "id": 3,
      "title": "test3",
      "checked": true
  }
  ```
- `PUT` `/todos/<id>` -> Mark todo with id `<id>` as checked
  **Request Body:**
  **Response:**
  ```
  {
      "id": <id>,
      "title": "test3",
      "checked": true
  }
  ```