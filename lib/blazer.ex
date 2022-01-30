defmodule Blazer do
  @moduledoc """
  Documentation for `Blazer`.
  """

  alias Blazer.Native

  def parse(term, opts \\ []) do
    target_case = Keyword.get(opts, :case, :camel)
      case term do
        x when is_binary(x) -> Native.convert_binary(x, target_case)
        x when is_map(x) -> Native.convert_map(x, target_case)
      end
  end
end
