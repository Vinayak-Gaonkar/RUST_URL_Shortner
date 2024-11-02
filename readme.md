# URL Shortener Service

This is a simple URL shortener service built with Rust, Diesel ORM, and SQLite. It provides basic functionality to shorten URLs, retrieve a list of saved URLs, and redirect to the original URL using a shortcode.

## Features

- **POST /shorten** - Shorten a given URL.
- **GET /url** - Retrieve a list of all saved URLs.
- **GET /geturl/{shortcode}** - Redirect to the original URL using the given shortcode.

## Prerequisites

- Rust (stable version)
- SQLite3
- Diesel CLI (`diesel_cli`) for managing database migrations

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/url-shortener
   cd url-shortener
