defmodule Blazer.Native do
  @moduledoc false
  use Rustler, otp_app: :blazer

  def convert_map(_arg1), do: :erlang.nif_error(:nif_not_loaded)

end
