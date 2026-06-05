# 🌍 Curr: Universal Currency Converter

`curr` is a lightning-fast, high-performance command-line interface (CLI) utility built in Rust that fetches and calculates real-time currency exchange rates globally. Featuring an interactive terminal UI spinner and automated local session logging, it delivers financial data instantly.

---

## ✨ Features

* **Real-time Global Rates:** Dynamically fetches live exchange rates across hundreds of fiat currencies utilizing an external async API.
* **Modern Async Architecture:** Engineered on top of the `tokio` multi-threaded runtime and `reqwest` HTTP client engine for non-blocking I/O operations.
* **Robust Argument Parsing:** Type-safe, declarative CLI argument handling via `clap v4` subcommands.
* **Interactive Terminal UI:** Features a steady-tick loading spinner provided by `indicatif` to keep terminal states responsive during network latency.
* **Persistent Session History:** Automatically tracks, formats, and appends successful conversions locally to a `conversion_history.txt` disk audit log.
* **Resilient Error Architecture:** Uses `anyhow` for linear error bubbling and predictable failure recovery contexts.

---

## 🛠️ Built With

* **[Rust](https://www.rust-lang.org/)** - Systems programming language ensuring memory safety and speed.
* **[Clap](https://docs.rs/clap/latest/clap/)** - Command Line Argument Parser for Rust.
* **[Tokio](https://tokio.rs/)** - Event-driven, non-blocking asynchronous runtime.
* **[Reqwest](https://docs.rs/reqwest/latest/reqwest/)** - Ergonomic HTTP client for making async web requests.
* **[Serde](https://serde.rs/)** - Framework for deserializing API JSON responses into strongly-typed Rust structures.
* **[Indicatif](https://docs.rs/indicatif/latest/indicatif/)** - Progress bars and spinners for command-line applications.
* **[Anyhow](https://docs.rs/anyhow/latest/anyhow/)** - Flexible, explicit error handling.

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the Rust compiler and `cargo` package manager installed on your machine:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
Installation
Clone the repository to your local machine:

Bash
git clone [https://killuazoldyck680/currency-converter.git](https://github.com/killuazoldyck680/currency-converter.git)
cd YOUR_REPO_NAME
Build the optimized release binary:

Bash
cargo build --release
The compiled executable will be available inside the ./target/release/ directory as curr.

💻 Usage
To run a conversion, pass the convert subcommand followed by the amount, the source currency asset, and the target currency asset. Case-sensitivity is fully normalized automatically!

Examples
Convert 100 US Dollars to Indian Rupees:

Bash
cargo run -- convert 100 usd inr
Convert 150 British Pounds to Euros:

Bash
cargo run -- convert 150 gbp eur
Convert 5000 Japanese Yen to Canadian Dollars:

Bash
cargo run -- convert 5000 jpy cad
Output Example
Plaintext
⠋ Fetching live exchange rates...
🎉 Conversion Successful!
100.00 USD = 9584.17 INR
💾 Conversion recorded to conversion_history.txt
💾 Audit Logging
Every successful transaction automatically updates a localized text document within the running directory root (conversion_history.txt). The ledger maintains persistent, structured output:

Plaintext
100.00 USD = 9584.17 INR
150.00 GBP = 178.45 EUR
5000.00 JPY = 43.12 CAD
📄 License
This project is open-source and available under the MIT License.


---



---




