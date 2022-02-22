defmodule Blazer do
  @moduledoc """
  Blazer is a case parser for json keys.

  available case options:
  * `:camel` example: `camelCase`
  * `:pascal` example: `PascalCase`
  * `:snake` example: `snake_case`
  * `:upper` example: `UPPERCASE`
  * `:kebab` example: `kebab-case`
  * `:title` example: `Title Case`
  """

  alias Blazer.Native
  alias Blazer.Structs.Opts

  @doc"""
  Parses a map or a string to the desired case

  ```elixir
  iex(1)> parse(%{"firstKey" => "data", "secondKey" => "data"}, case: :snake, keys: :atoms)
  {:ok, %{first_key: "data", second_key: "data"}}

  iex(2)> parse("john_doe", case: :title)
  {:ok, "John Doe"}
  ```
  """

  @type opts :: [ case: :camel|:pascal|:snake|:upper|:title, keys: :strings|:atoms|:atoms!]
  @spec parse(String.t() | map(), Opts.t()) :: {:ok, String.t() | map()} | {:error, String.t()}
  def parse(input, opts) do
    transform(input, opts)
  end

  @spec parse!(String.t() | map(), Opts.t()) :: String.t() | map()
  def parse!(input, opts), do: force!(fn -> parse(input, opts) end)

  @spec encode_to_iodata!(map(), opts) :: [...]
  def encode_to_iodata!(input, opts \\ []) do
    {:ok, opts} = get_out_opts(opts)
    input
    |> parse!(opts)
    |> Jason.encode_to_iodata!(opts)
  end

  @doc"""
  encode a map into JSON after parsing its keys

  `opts` is passed to Jason, so all its options can be used
  """
  @spec encode(map(), opts) :: {:ok, String.t()} | {:error, String.t()}
  def encode(input, opts \\ []) do
    with {:ok, opts} <- get_out_opts(opts),
         {:ok, parsed} <- transform(input, opts),
         {:ok, encoded} <- Jason.encode(parsed, opts) do
      {:ok, encoded}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  @spec encode!(map(), opts) :: String.t()
  def encode!(input, opts \\ []) do
    force!(fn -> encode(input, opts) end)
  end

  @doc"""
  Decode a JSON into a map and parse its keys

  `opts` is passed to Jason, so all its options can be used
  """
  @spec decode(String.t(), opts) :: {:ok, map()} | {:error, String.t()}
  def decode(input, opts \\ []) do
    with {:ok, opts} <- get_in_opts(opts),
         {:ok, decoded} <- Jason.decode(input, opts),
         {:ok, parsed} <- transform(decoded, opts) do
      {:ok, parsed}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  @spec decode!(String.t(), opts) :: map()
  def decode!(input, opts \\ []) do
    force!(fn -> decode(input, opts) end)
  end

  defp transform(term, opts) when is_binary(term),
    do: Native.convert_binary(term, opts)

  defp transform(term, opts) when is_map(term), do: Native.convert_map(term, opts)
  defp transform(_term, _target_opts), do: raise("only strings and maps are accepted.")

  defp force!(fun) do
    case fun.() do
      {:ok, result} -> result
      {:error, reason} -> raise reason
    end
  end

  defp get_in_opts(opts), do: get_opts(opts, :inner_case)
  defp get_out_opts(opts), do: get_opts(opts, :outer_case)

  defp get_opts(opts, direction) do
    cond do
      length(opts) > 0 -> {:ok, opts}

      Application.get_env(:blazer, direction) ->
        {:ok, [keys: (Application.get_env(:blazer, :keys) || :strings), case: Application.get_env(:blazer, direction)]}

      true ->
        {:error, "Target case not provided, either pass an case in the options or set in the configs."}
    end
  end
end
