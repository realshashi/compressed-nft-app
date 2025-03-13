defmodule BubblegumTest do
  use ExUnit.Case
  doctest Bubblegum

  setup do
    # Add any setup code here if needed
    :ok
  end

  describe "tree configuration" do
    test "create_tree_config with valid parameters" do
      result = Bubblegum.create_tree_config(
        14,
        64,
        "11111111111111111111111111111111"
      )

      assert match?({:ok, _signature}, result)
    end

    test "create_tree_config with invalid authority" do
      result = Bubblegum.create_tree_config(
        14,
        64,
        "invalid_pubkey"
      )

      assert match?({:error, "Invalid public key: " <> _}, result)
    end

    test "create_tree_config with invalid depth" do
      result = Bubblegum.create_tree_config(
        -1,
        64,
        "11111111111111111111111111111111"
      )

      assert match?({:error, _}, result)
    end
  end

  describe "NFT operations" do
    test "mint_v1 with valid parameters" do
      result = Bubblegum.mint_v1(
        "Test NFT",
        "TEST",
        "https://example.com/metadata.json",
        "11111111111111111111111111111111",
        "11111111111111111111111111111111"
      )

      assert match?({:ok, _signature}, result)
    end

    test "mint_v1 with invalid collection address" do
      result = Bubblegum.mint_v1(
        "Test NFT",
        "TEST",
        "https://example.com/metadata.json",
        "invalid_collection",
        "11111111111111111111111111111111"
      )

      assert match?({:error, "Invalid public key: " <> _}, result)
    end

    test "transfer with valid parameters" do
      result = Bubblegum.transfer(
        "asset_id_here",
        "11111111111111111111111111111111",
        "22222222222222222222222222222222"
      )

      assert match?({:ok, _signature}, result)
    end

    test "transfer with invalid owner address" do
      result = Bubblegum.transfer(
        "asset_id_here",
        "invalid_owner",
        "22222222222222222222222222222222"
      )

      assert match?({:error, "Invalid public key: " <> _}, result)
    end
  end

  describe "cache operations" do
    test "clear_cache" do
      assert match?({:ok, _message}, Bubblegum.clear_cache())
    end
  end
end