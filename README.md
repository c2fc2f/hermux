# hermux

`hermux` is a multiplexer proxy for **OpenRouter**. It allows you to use multiple OpenRouter accounts (API keys) to handle your API requests, effectively bypassing daily rate limits by distributing the load.

It also includes an access control layer to prevent unauthorized users from using your proxy.

## Overview

This proxy acts as an intelligent load balancer for OpenRouter. When it receives an *authorized* request, it selects an OpenRouter API key from its pool (stored in `tokens.csv`) that has been used the least. It then uses this key to forward the request to OpenRouter.
> When returning the response, `hermux` adds an `X-TOKEN-NAME` header, so you can easily identify which key was used.

This mechanism allows you to combine the free-tier (or other) limits of multiple accounts, significantly increasing your overall request capacity.

## Features

  * **Token Multiplexing:** Automatically proxies requests to OpenRouter.
  * **Smart Key Rotation:** Prioritizes the least-used OpenRouter API key to avoid hitting daily limits on any single account.
  * **Load Balancing:** Distributes request load across a pool of available OpenRouter keys.
  * **🔒 Access Control:** Secures your proxy endpoint using a list of allowed "access tokens" (from `allow.txt`).

## 🔒 Access Control

To prevent your `hermux` proxy from being used by anyone, it is protected by an access token system.

  * You must create a file named `allow.txt` in the root directory.
  * This file contains a list of **access tokens** that you will use to authenticate your *own* requests to `hermux`.
  * When `hermux` receives a request, it checks for an `Authorization: Bearer <token>` header.
  * It will **only accept** the request if the provided `<token>` is present in your `allow.txt` file.

> **Important:** The access tokens in `allow.txt` are **NOT** your OpenRouter API keys. They are separate, secret tokens (you can generate them yourself, e.g., using a password generator) that serve as a password to use *your* proxy.

## How It Works

1.  The `hermux` server starts and loads two files:
      * `tokens.csv`: A list of your **OpenRouter API keys**.
      * `allow.txt`: A list of your **access tokens** used to secure the proxy.
2.  Your application sends an API request to the `hermux` proxy endpoint, including your access token in the `Authorization` header.
4.  If valid, `hermux` identifies which OpenRouter key from `tokens.csv` has been used the least.
5.  It forwards the request to OpenRouter, using the selected OpenRouter key for authentication.
6.  The response from OpenRouter is streamed back to your application.

## Getting Started

### Prerequisites

  * Rust (and Cargo)
  * A `tokens.csv` file.
  * An `allow.txt` file.

### Installation & Running

1.  **Clone the repository:**

    ```sh
    git clone https://github.com/c2fc2f/hermux.git
    cd hermux
    ```

2.  **Create your OpenRouter key file (`tokens.csv`):**
    Create a file named `tokens.csv`. Add your OpenRouter API keys, one per line:

    ```csv
    name,token
    token1,key_one
    token2,key_two
    token3,key_three
    ```

3.  **Create your access token file (`allow.txt`):**
    Create a file named `allow.txt`. Add one or more secret tokens (passwords) that you will use to access your proxy:

    ```txt
    secret-proxy-token-123
    another-app-token-abc
    ```

4.  **Build and run the project:**

    ```sh
    cargo run --release --features auth
    ```

The proxy server will start (e.g., on `127.0.0.1:3333`).

## Usage

Once the proxy is running, configure your application to send OpenRouter API requests to the `hermux` server endpoint (e.g., `http://127.0.0.1:3333/api/v1`) instead of the official `https://openrouter.ai/api/v1` endpoint.

**You must include your access token (from `allow.txt`) as a Bearer token in the `Authorization` header.**

### Example (using `curl`)

Replace `secret-proxy-token-123` with a token from your `allow.txt` file.

```sh
curl http://127.0.0.1:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: secret-proxy-token-123" \
  -d '{
    "model": "google/gemma-7b-it",
    "messages": [
      {"role": "user", "content": "What is the capital of France?"}
    ]
  }'
```

The proxy will handle this request, removing the `Authorization: secret-proxy-token-123` header and replacing it with the correct OpenRouter key (e.g., `Authorization: Bearer key_one`) before forwarding it.

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.
