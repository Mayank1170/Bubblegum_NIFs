# NIF for Elixir.BubblegumElixir.Bubblegum

## To build the NIF module:


## To load the NIF:

```elixir
defmodule BubblegumElixir.Bubblegum do
  use Rustler, otp_app: :bubblegum_elixir, crate: "bubblegumelixir_bubblegum"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples

[This](https://github.com/rusterlium/NifIo) is a complete example of a NIF written in Rust.
