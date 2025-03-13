defmodule MplBubblegum.Client do
  @moduledoc """
  Main client interface for Bubblegum operations
  """
  use Rustler,
    otp_app: :mpl_bubblegum,
    crate: :mpl_bubblegum_nif,
    path: "native/mpl_bubblegum_nif"

  alias MplBubblegum.{Connection, Types}

  @doc """
  Creates a compressed NFT merkle tree configuration.

  ## Parameters
  - params: Map containing tree configuration parameters:
    - max_depth: Maximum depth of the merkle tree
    - max_buffer_size: Maximum buffer size for the tree
  - keypair: Binary representation of the signer's keypair

  ## Returns
  - {:ok, signature} on success
  - {:error, reason} on failure
  """
  def create_tree_config(_params, _keypair), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Mints a compressed NFT to the specified tree.

  ## Parameters
  - tree_id: String representation of the tree's public key
  - metadata: Map containing NFT metadata:
    - name: Name of the NFT
    - uri: URI pointing to the NFT's metadata
    - symbol: Symbol for the NFT (optional)
  - keypair: Binary representation of the signer's keypair

  ## Returns
  - {:ok, signature} on success
  - {:error, reason} on failure
  """
  def mint_compressed_nft(_tree_id, _metadata, _keypair), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Transfers a compressed NFT to a new owner.

  ## Parameters
  - nft_id: String representation of the NFT's asset ID
  - recipient: String representation of the recipient's public key
  - keypair: Binary representation of the sender's keypair

  ## Returns
  - {:ok, signature} on success
  - {:error, reason} on failure
  """
  def transfer_compressed_nft(_nft_id, _recipient, _keypair),
    do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Burns a compressed NFT.

  ## Parameters
  - nft_id: String representation of the NFT's asset ID
  - keypair: Binary representation of the owner's keypair

  ## Returns
  - {:ok, signature} on success
  - {:error, reason} on failure
  """
  def burn(_nft_id, _keypair), do: :erlang.nif_error(:nif_not_loaded)

  # Error handling helper
  defp handle_error(:invalid_keypair), do: "Invalid keypair format"
  defp handle_error(:rpc_error), do: "RPC communication failed"
  defp handle_error(:invalid_pubkey), do: "Invalid public key format"
  defp handle_error(other), do: "Error: #{inspect(other)}"
end
