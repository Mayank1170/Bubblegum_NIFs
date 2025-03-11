# test/bubblegum_test.exs
defmodule BubblegumElixir.BubblegumTest do
  use ExUnit.Case

  test "test_nif returns 42" do
    assert BubblegumElixir.Bubblegum.test_nif() == 42
  end
end
