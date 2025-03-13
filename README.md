# Compressed NFT App

This project provides an Elixir interface for MPL-Bubblegum compressed NFT operations. It includes functionality for creating tree configurations, minting, and transferring compressed NFTs.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Elixir](https://elixir-lang.org/install.html)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/compressed-nft-app.git
    cd compressed-nft-app
    ```

2. Install Rust dependencies:

    ```sh
    cd native/bubblegum_nif
    cargo build
    ```

3. Install Elixir dependencies:

    ```sh
    cd ../..
    mix deps.get
    ```

4. Compile the project:

    ```sh
    mix compile
    ```

## Usage

### Creating a Tree Configuration

```elixir
{:ok, signature} = Bubblegum.create_tree_config(14, 64, "11111111111111111111111111111111")
```

### Minting a Compressed NFT

```elixir
{:ok, signature} = Bubblegum.mint_v1(
  "Test NFT",
  "TEST",
  "https://example.com/metadata.json",
  "11111111111111111111111111111111",
  "11111111111111111111111111111111"
)
```

### Transferring a Compressed NFT

```elixir
{:ok, signature} = Bubblegum.transfer(
  "asset_id_here",
  "11111111111111111111111111111111",
  "22222222222222222222222222222222"
)
```

### Clearing the Cache

```elixir
:ok = Bubblegum.clear_cache()
```

## Running Tests

To run the tests, use the following command:

```sh
mix test
```

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
