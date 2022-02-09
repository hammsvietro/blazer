defmodule BlazerTest do
  use ExUnit.Case
  doctest Blazer

  describe "parse/2" do
    test "should return {:error, reason} when the target case isn't valid" do
      input = %{input_case: true}
      return = Blazer.parse(input, :invalid_case)
      assert {:error, reason} = return
      assert is_binary(reason)
    end

    test "should return {:ok, result} when the the input is valid" do
      input = %{input_case: true}
      return = Blazer.parse(input, :camel)
      assert {:ok, output_map} = return
      assert %{"inputCase" => true} = output_map
    end
  end
end
