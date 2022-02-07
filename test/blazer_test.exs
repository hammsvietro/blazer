defmodule BlazerTest do
  use ExUnit.Case
  doctest Blazer

  describe "parse/2" do
    test "should return {:error, reason} when the target case isn't valid" do
      input = %{snake_case: true}
      target = Blazer.parse(input, :invalid_case)
      assert {:error, _} = target
    end
  end
end
