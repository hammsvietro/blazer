defmodule Blazer.Native do
  @moduledoc false
  use Rustler, otp_app: :blazer

  def convert_map(_map, _case), do: :erlang.nif_error(:nif_not_loaded)
  def convert_binary(_binary, _case), do: :erlang.nif_error(:nif_not_loaded)
end
