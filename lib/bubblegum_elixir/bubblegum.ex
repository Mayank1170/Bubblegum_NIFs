defmodule BubblegumElixir.Bubblegum do
  use Rustler,
    otp_app: :bubblegum_elixir,
    crate: :bubblegumelixir_bubblegum_nif,
    path: "native/bubblegumelixir_bubblegum_nif"

  def test_nif(), do: :erlang.nif_error(:nif_not_loaded)
end
