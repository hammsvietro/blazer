defmodule Blazer do
  @moduledoc """
  Documentation for `Blazer`.
  """

  alias Blazer.Native

  def parse(input, opts) do
    transform(input, opts)
  end

  def parse!(input, opts), do: force!(fn -> parse(input, opts) end)

  def encode_to_iodata!(input, opts \\ []) do
    input
    |> parse(opts)
    |> Jason.encode_to_iodata!()
  end

  def encode(input, opts \\ []) do
    with {:ok, target_case} <- get_out_case(opts),
         {:ok, parsed} <- transform(input, target_case),
         {:ok, encoded} <- Jason.encode(parsed) do
      {:ok, encoded}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  def encode!(input, opts \\ []) do
    force!(fn -> encode(input, opts) end)
  end

  def decode(input, opts \\ []) do
    with {:ok, target_case} <- get_in_case(opts),
         {:ok, decoded} <- Jason.decode(input),
         {:ok, parsed} <- transform(decoded, target_case) do
      {:ok, parsed}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  def decode!(input, opts \\ []) do
    force!(fn -> decode(input, opts) end)
  end

  defp transform(term, opts) when is_binary(term),
    do: Native.convert_binary(term, opts)

  defp transform(term, opts) when is_map(term), do: Native.convert_map(term, opts)
  defp transform(_term, _target_case), do: raise("only strings and maps are accepted.")

  defp force!(fun) do
    case fun.() do
      {:ok, result} -> result
      {:error, reason} -> raise reason
    end
  end

  defp get_in_case(opts), do: get_case(opts, :input_case)
  defp get_out_case(opts), do: get_case(opts, :output_case)

  defp get_case(opts, direction) do
    cond do
      Keyword.get(opts, :case) ->
        {:ok, Keyword.get(opts, :case)}

      Application.get_env(:blazer, direction) ->
        {:ok, Application.get_env(:blazer, direction)}

      true ->
        {:error,
         "Target case not provided, either pass an case in the options or set in the configs."}
    end
  end
end
