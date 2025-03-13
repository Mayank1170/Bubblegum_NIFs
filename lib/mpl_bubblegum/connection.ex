defmodule MplBubblegum.Connection do
  @moduledoc """
  Manages Solana network connections and keypair handling
  """

  defstruct [
    :keypair,
    :rpc_url,
    :commitment
  ]

  @type t :: %__MODULE__{
          keypair: binary,
          rpc_url: String.t(),
          commitment: String.t()
        }

  @doc """
  Creates a new connection configuration
  """
  @spec new(binary, String.t(), String.t()) :: t
  def new(keypair, rpc_url \\ "https://api.devnet.solana.com", commitment \\ "confirmed") do
    %__MODULE__{
      keypair: keypair,
      rpc_url: rpc_url,
      commitment: commitment
    }
  end
end
