# Briefly
[![CI (master)](https://github.com/adrianEffe/briefly/actions/workflows/Tests.yml/badge.svg)](https://github.com/adrianEffe/briefly/actions/workflows/Tests.yml)
[![CD (master)](https://github.com/adrianEffe/briefly/actions/workflows/shuttle_deploy.yml/badge.svg)](https://github.com/adrianEffe/briefly/actions/workflows/shuttle_deploy.yml)

Briefly is a URL shortener service built with Rust, using the Axum web framework and SQLx for database interactions. It's a simple and efficient tool designed to shorten long URLs, making them easier to manage and share. This README provides an overview of Briefly's features and how to use it.

## Features

1. **URL Shortening**: Briefly takes long URLs and shortens them, creating concise and easy-to-share links.

2. **Custom URL Extensions**: You can define custom extensions for your shortened URLs. For example, if you shorten `"https://www.rust-lang.net"` with the custom extension "rust," the shortened URL will be accessible at `https://briefly.shuttleapp.rs/rust`.

3. **Random Extensions**: If you don't specify a custom extension, Briefly generates a unique, random extension consisting of 7 characters.

4. **Health Check**: The service is live and available at [https://briefly.shuttleapp.rs](https://briefly.shuttleapp.rs). You can perform a health check by running:

   ```bash
   curl --verbose https://briefly.shuttleapp.rs/health_check
   ```
   A successful response (HTTP status code 200) indicates that the service is up and running.

## How to Shorten URLs
To shorten a URL using Briefly, you can use the following curl command as an example:
   ```bash
   curl --header "Content-Type: application/json" \
  --data '{"url":"https://www.rust-lang.net"}' \
  https://briefly.shuttleapp.rs/shorten
```
This command sends a `POST` request to the service, specifying the original URL and the custom extension you want to use. If successful, Briefly will generate a unique, random extension for your shortened URL:
  ```json
  {
      "extension": "AsYuFAW"
  }
  ```
You can then access the shortened URL at `https://briefly.shuttleapp.rs/AsYuFAW`.

Note you can defined a custom extension like below, but if it is already taken a HTTP status 500 will be returned.

```bash
   curl --header "Content-Type: application/json" \
  --data '{"url":"https://www.rust-lang.net","extension":"rust"}' \
  https://briefly.shuttleapp.rs/shorten
```
Expect a response like this:
  ```json
  {
      "extension": "rust"
  }
  ```
You can then access the shortened URL at `https://briefly.shuttleapp.rs/rust`.

## How to Run
To run the Briefly service locally, follow these steps:

**- Install Dependencies**

Install Rust, PostgreSQL, and Docker on your system.

**- Initialize Shuttle**

Run the following commands to set up the Shuttle environment:
  ```bash
  cargo shuttle init
  ```
For more detailed information regarding Shuttle, refer to the [Shuttle Documentation](https://docs.shuttle.rs/introduction/welcome).

**- Start the Project**

Start the Shuttle project with the following command:
  ```bash
  cargo shuttle project start
  ```

**- Initialize the Database:**

This step is needed in order to run tests, otherwise feel free to skip this step, shuttle will take care of the rest.
Run the following command to make the initialization script executable:
  ```bash
  chmod +x scripts/init_db.sh
  ```
Launch PostgreSQL using the script:
  ```bash
  ./scripts/init_db.sh
  ```
**- Run the Service**

Start the Briefly service with the following command:
  ```bash
  cargo shuttle run
  ```




