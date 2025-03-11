defmodule BubblegumElixirTest do
  use ExUnit.Case
  doctest BubblegumElixir

  test "greets the world" do
    assert BubblegumElixir.hello() == :world
  end
end
