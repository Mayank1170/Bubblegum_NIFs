defmodule MplBubblegumTest do
  use ExUnit.Case

  test "test_nif returns 42" do
    assert MplBubblegum.Client.test_nif() == 42
  end

  @tag :external
  test "create tree config" do
    params = %{
      max_depth: 14,
      max_buffer_size: 64
    }
    
    keypair = :crypto.strong_rand_bytes(64)
    result = MplBubblegum.Client.create_tree_config(params, keypair)
    assert match?({:ok, _}, result)
  end
end
