defmodule Blazer.Native do
  use Rustler, otp_app: :blazer

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def convert(_arg1), do: :erlang.nif_error(:nif_not_loaded)

end
