defmodule IlpStreaming.Server.Worker do
  @moduledoc """
  Each process represents an active STREAM server. Server definition
  taken from Interledger STREAM protocol specs as `the endpoint accepting
  incoming connections`.

  Ref: https://interledger.org/rfcs/0029-stream/#2-conventions-and-definitions
  """

  use GenServer

  def start_link(opts) when is_list(opts) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl GenServer
  def init(init_arg) do
    {:ok, init_arg}
  end

  @impl GenServer
  def handle_info({:request, encoded_packet, reply_to}, state) do
    case IlpStreaming.decode(encoded_packet) do
      {:error, _} ->
        msg = "could not decode recieved payload from client"
        IO.puts("SERVER: error - #{msg}")
        IO.inspect(encoded_packet, label: "Payload")
        send(reply_to, {:response, {:error, reject()}})

      decoded_packet ->
        case decoded_packet["ilp_packet_type"] do
          12 -> IO.puts("SERVER: received prepare packet")
          13 -> IO.puts("SERVER: received fulfill packet")
          14 -> IO.puts("SERVER: received reject packet")
        end

        send(reply_to, {:response, {:ok, fulfill()}})
    end

    {:noreply, state}
  end

  defp reject do
    %{
      "frames" => [
        %{
          "code" => 1,
          "message" => "pippo",
          "stream_id" => 76,
          "type" => "stream_close"
        }
      ],
      "ilp_packet_type" => 14,
      "prepare_amount" => 99,
      "sequence" => 1
    }
    |> IlpStreaming.encode()
  end

  defp fulfill do
    %{
      "frames" => [],
      "ilp_packet_type" => 13,
      "prepare_amount" => 99,
      "sequence" => 1
    }
    |> IlpStreaming.encode()
  end
end
