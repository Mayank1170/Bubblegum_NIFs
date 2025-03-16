defmodule MplBubblegum.Types do
  @moduledoc """
  Data types for Bubblegum operations
  """

  defmodule TreeConfig do
    @moduledoc "Configuration for a compressed NFT merkle tree"
    defstruct [
      :max_depth,
      :max_buffer_size,
      :public,
      :decompressible
    ]

    @type t :: %__MODULE__{
            max_depth: pos_integer,
            max_buffer_size: pos_integer,
            public: boolean,
            decompressible: boolean
          }
  end

  defmodule Metadata do
    @moduledoc "Metadata for a compressed NFT"
    defstruct [
      :name,
      :symbol,
      :uri,
      :seller_fee_basis_points,
      :creators,
      :primary_sale_happened,
      :is_mutable,
      :collection,
      :uses
    ]

    @type t :: %__MODULE__{
            name: String.t(),
            symbol: String.t(),
            uri: String.t(),
            seller_fee_basis_points: non_neg_integer,
            creators: [Creator.t()],
            primary_sale_happened: boolean,
            is_mutable: boolean,
            collection: nil | Collection.t(),
            uses: nil | Uses.t()
          }
  end

  defmodule Creator do
    @moduledoc "Creator information for an NFT"
    defstruct [
      :address,
      :verified,
      :share
    ]

    @type t :: %__MODULE__{
            address: String.t(),
            verified: boolean,
            share: non_neg_integer
          }
  end

  defmodule Collection do
    @moduledoc "Collection information for an NFT"
    defstruct [
      :verified,
      :key
    ]

    @type t :: %__MODULE__{
            verified: boolean,
            key: String.t()
          }
  end

  defmodule Uses do
    @moduledoc "Uses information for an NFT"
    defstruct [
      :use_method,
      :remaining,
      :total
    ]

    @type t :: %__MODULE__{
            use_method: integer(),
            remaining: integer(),
            total: integer()
          }
  end
end
