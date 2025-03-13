defmodule MplBubblegum do
  @moduledoc """
  Main interface for Metaplex Bubblegum compressed NFTs
  """

  alias MplBubblegum.Client

  defdelegate create_tree_config(params, keypair), to: Client
  defdelegate mint_compressed_nft(tree_id, metadata, keypair), to: Client
  defdelegate transfer_compressed_nft(nft_id, recipient, keypair), to: Client
  defdelegate burn(nft_id, keypair), to: Client
end
