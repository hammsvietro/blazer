defmodule BlazerTest do
  use ExUnit.Case
  doctest Blazer

  test "greets the world" do
    assert Blazer.hello() == :world
  end
end
