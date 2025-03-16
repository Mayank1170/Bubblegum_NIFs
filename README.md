# MPL-Bubblegum Elixir

Elixir NIFs for Metaplex Bubblegum compressed NFTs on Solana.

## Features

- Create compressed NFT merkle trees
- Mint compressed NFTs
- Transfer NFTs between wallets
- Burn NFTs

## Installation

```elixir
def deps do
  [
    {:mpl_bubblegum, "~> 0.1.0"}
  ]
end
```

## Quick Examples

### Create a tree

```elixir

keypair = :crypto.strong_rand_bytes(64)
params = %{max_depth: 14, max_buffer_size: 64}
{:ok, tree_id} = MplBubblegum.create_tree_config(params, keypair)
```

### Mint an NFT

```elixir

metadata = %{name: "My NFT", uri: "https://example.com/metadata.json", symbol: "CNFT"}
{:ok, asset_id} = MplBubblegum.mint_compressed_nft(tree_id, metadata, keypair)
```

### Transfer an NFT

```elixir

recipient = "5ZWj7a1f8tWkjBESHKgrLmXshuXxqeY9SYcfbshpAqPG"
{:ok, signature} = MplBubblegum.transfer_compressed_nft(asset_id, recipient, keypair)
```

### Burn an NFT

```elixir

{:ok, signature} = MplBubblegum.burn(asset_id, keypair)
```

## Testing

```bash
mix test
```

## License

Apache License 2.0


