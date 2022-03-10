defmodule Demo do
  @moduledoc false

  def setup, do: IlpStreaming.Server.Manager.start_child()

  def prepare_params do
    %{
      "sequence" => 1,
      "ilp_packet_type" => 12,
      "prepare_amount" => 99,
      "frames" => [
        %{
          "type" => "connection_asset_details",
          "source_asset_code" => "XYZ",
          "source_asset_scale" => 9
        },
        %{
          "type" => "connection_max_data",
          "max_offset" => 8766
        },
        %{
          "type" => "connection_data_blocked",
          "max_offset" => 888
        },
        %{
          "type" => "connection_max_stream_id",
          "max_stream_id" => 34
        },
        %{
          "type" => "stream_money",
          "stream_id" => 88,
          "shares" => 99
        },
        %{
          "type" => "stream_max_money",
          "stream_id" => 11,
          "receive_max" => 987,
          "total_received" => 500
        },
        %{
          "type" => "stream_data",
          "stream_id" => 35,
          "offset" => 9000,
          "data" => "pino"
        },
        %{
          "type" => "stream_max_data",
          "stream_id" => 7,
          "max_offset" => 4444
        }
      ]
    }
  end

  def new_client_conn, do: IlpStreaming.Client.Manager.start_child()

  def start_simulation(conn) do
    Task.async(fn ->
      Enum.each(0..20, fn _ ->
        IlpStreaming.Client.Worker.send_prepare_sync(conn, self(), prepare_params())
        Process.sleep(500)
        :ok
      end)
    end)
  end
end
