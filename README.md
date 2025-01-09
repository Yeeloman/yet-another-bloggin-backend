# Personal Blog API

This project is a RESTful API designed to power a personal blog. The API provides basic CRUD operations (Create, Read, Update, Delete) for managing blog articles. It is built using the **Actix-web** framework for handling HTTP requests and **SurrealDB** as the database for storing and retrieving data.

## Features

The API supports the following functionalities:

- **Get All Articles:** Retrieve a list of articles with optional filters (e.g., by publishing date or tags).
- **Get Single Article:** Retrieve a single article by its unique ID.
- **Create Article:** Add a new article to the blog.
- **Delete Article:** Remove an article by its unique ID.
- **Update Article:** Modify an existing article by its unique ID.

## Technology Stack

- **Backend Framework:** [Actix-web](https://actix.rs/) - A powerful and performant web framework for Rust.
- **Database:** [SurrealDB](https://surrealdb.com/) - A next-generation NoSQL database for storing structured data.

## API Endpoints

### 1. Get All Articles

**GET** `/articles`

#### Query Parameters

- `date` (optional): Filter articles by publishing date.
- `tags` (optional): Filter articles by tags.

#### Response

- **200 OK**: List of articles.

```json
[
  {
    "id": "1",
    "title": "My First Blog Post",
    "content": "This is the content of the article.",
    "date": "2025-01-01",
    "tags": ["Rust", "Actix-web"]
  },
  ...
]
```

### 2. Get Single Article

**GET** `/articles/{id}`

#### Path Parameter

- `id`: The ID of the article to retrieve.

#### Response

- **200 OK**: The requested article.
- **404 Not Found**: Article not found.

### 3. Create Article

**POST** `/articles`

#### Request Body

```json
{
  "title": "New Blog Post",
  "content": "This is the content of the article.",
  "date": "2025-01-01",
  "tags": ["Rust", "SurrealDB"]
}
```

#### Response

- **201 Created**: Article created successfully.

### 4. Update Article

**PUT** `/articles/{id}`

#### Path Parameter

- `id`: The ID of the article to update.

#### Request Body

- Partial or full article fields to update.

```json
{
  "title": "Updated Blog Post Title"
}
```

#### Response

- **200 OK**: Article updated successfully.
- **404 Not Found**: Article not found.

### 5. Delete Article

**DELETE** `/articles/{id}`

#### Path Parameter

- `id`: The ID of the article to delete.

#### Response

- **204 No Content**: Article deleted successfully.
- **404 Not Found**: Article not found.

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/)
- Install [SurrealDB](https://surrealdb.com/docs/installation)

### Running the Application

1. Clone the repository:

   ```bash
   git clone https://github.com/Yeeloman/yet-another-bloggin-backend.git
   cd yet-another-bloggin-backend
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

3. Start SurrealDB:

   ```bash
   surreal start --log debug
   ```

4. Run the server:

   ```bash
   cargo run
   ```

5. Access the API at `http://localhost:8080`.

## Development

### Project Structure

- `src/main.rs`: Entry point of the application.
- `src/routes/`: API route definitions.
- `src/models/`: Data models for interacting with SurrealDB.
- `src/services/`: Logic for handling API requests.

### Adding a New Feature

To add a new feature:

1. Define the route in `src/routes/`.
2. Implement the logic in `src/services/`.
3. Update the corresponding model in `src/models/`.

### Running Tests

To run the test suite:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please submit issues and pull requests for improvements or bug fixes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
