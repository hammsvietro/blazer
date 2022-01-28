defmodule Blazer do
  @moduledoc """
  Documentation for `Blazer`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Blazer.hello()
      :world

  """
  def hello do
    1..10
    |> Enum.map(fn _ -> 10 end)
    |> IO.inspect
    :world
  end
end
