defmodule SPSP.Server.Plug do
  @moduledoc false

  use Plug.Router


  plug(Plug.Logger)
  plug(:match)
  plug(Plug.Parsers, parsers: [:json], json_decoder: Jason)
  plug(:dispatch)

  # Establish connection
  get "/" do
    send_resp(conn, 200, establish_conn())
  end

  # Receive money
  post "/" do
    IO.inspect(conn, label: "conn")
    res = process(conn.body_params)
    send_resp(conn, 201, res)
  end

  defp process(_body_params) do
    fulfill()
  end

  defp fulfill do
    %{
      "frames" => [],
      "ilp_packet_type" => 13,
      "prepare_amount" => 99,
      "sequence" => 1
    }
    # |> IlpStreaming.encode()
    # |> :binary.list_to_bin()
    |> Jason.encode!()
  end

  defp establish_conn do
    Jason.encode!(%{
      destination_account: "example.ilpdemo.red.bob",
      shared_secret: "secret",
      receipts_enabled: false
      })
  end
end
