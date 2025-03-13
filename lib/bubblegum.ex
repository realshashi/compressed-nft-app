defmodule Bubblegum do
  @moduledoc """
  Elixir interface for MPL-Bubblegum compressed NFT operations.
  Provides functionality for creating tree configs, minting, and transferring compressed NFTs.
  """

  use Rustler,
    otp_app: :bubblegum,
    crate: :bubblegum_nif,
    mode: if(Mix.env() == :prod, do: :release, else: :debug),
    path: "native/bubblegum_nif"

  @type public_key :: <<_::256>>
  @type signature :: <<_::512>>
  @type error :: {:error, String.t()}

  @doc """
  Creates a new merkle tree configuration for compressed NFTs.

  ## Parameters
  - max_depth: Integer representing the maximum depth of the merkle tree
  - max_buffer_size: Integer representing the maximum buffer size
  - authority: String representing the authority public key

  ## Returns
  - {:ok, transaction_signature} on success
  - {:error, reason} on failure
  """
  @spec create_tree_config(
    max_depth :: integer(),
    max_buffer_size :: integer(),
    authority :: String.t()
  ) :: {:ok, signature()} | error()
  def create_tree_config(_max_depth, _max_buffer_size, _authority), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Mints a new compressed NFT.

  ## Parameters
  - name: String name of the NFT
  - symbol: String symbol of the NFT
  - uri: String URI of the NFT metadata
  - collection: String representing the collection public key
  - recipient: String representing the recipient's public key

  ## Returns
  - {:ok, transaction_signature} on success
  - {:error, reason} on failure
  """
  @spec mint_v1(
    name :: String.t(),
    symbol :: String.t(),
    uri :: String.t(),
    collection :: String.t(),
    recipient :: String.t()
  ) :: {:ok, signature()} | error()
  def mint_v1(_name, _symbol, _uri, _collection, _recipient), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Transfers a compressed NFT to a new owner.

  ## Parameters
  - asset_id: String representing the asset ID
  - owner: String representing the current owner's public key
  - recipient: String representing the recipient's public key

  ## Returns
  - {:ok, transaction_signature} on success
  - {:error, reason} on failure
  """
  @spec transfer(
    asset_id :: String.t(),
    owner :: String.t(),
    recipient :: String.t()
  ) :: {:ok, signature()} | error()
  def transfer(_asset_id, _owner, _recipient), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Clears the cache.

  ## Returns
  - :ok on success
  - {:error, reason} on failure
  """
  @spec clear_cache() :: :ok | error()
  def clear_cache, do: :erlang.nif_error(:nif_not_loaded)
end