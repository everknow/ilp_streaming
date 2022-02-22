defmodule IlpStreaming do
  use Rustler, otp_app: :ilp_streaming, crate: "ilpstreaming"

  # When your NIF is loaded, it will override this function.
  def encode(_a), do: :erlang.nif_error(:nif_not_loaded)
end
