defmodule SPSP.Client.Worker do
  @moduledoc """
  Each process represents an active STREAM client. Client definition
  taken from Interledger STREAM protocol specs as `the endpoint initiating
  the connection`.

  Ref: https://interledger.org/rfcs/0029-stream/#2-conventions-and-definitions
  """

  alias IlpStreaming.Connection.Http

  use GenServer

  def start_link(opts) when is_list(opts) do
    GenServer.start_link(__MODULE__, opts, opts)
  end

  def send_money(conn_id, from, params) do
    GenServer.call(conn_id, {:send_money, from, params})
  end

  @impl GenServer
  def init(args) do
    {:ok, conn} = IlpStreaming.Connection.Manager.start_child()

    state =
      args
      |> Enum.into(%{})
      |> Map.put(:self, self())
      |> Map.put(:conn, conn)

    {:ok, state}
  end

  @impl GenServer
  def handle_call({:send_money, from, params}, _from, state) do
    case IlpStreaming.encode(params) do
      {:error, _} ->
        {:reply, {:error, "could not encode prepare with those params"}, state}

      encoded_packet ->
        Http.request(
          state.conn,
          "POST",
          "/",
          [{"content-type", "application/json"}],
          Jason.encode!(%{packet: encoded_packet})
        )

        # send(Server, {:request, :binary.list_to_bin(encoded_packet), state.self})
        {:reply, {:ok, "sent prepare to server"}, Map.put(state, :reply_to, from)}
    end
  end

  @impl GenServer
  def handle_info({:response, {:ok, payload}}, state) do
    case IlpStreaming.decode(:binary.list_to_bin(payload)) do
      {:error, _} ->
        msg = "could not decode recieved payload from server"
        IO.puts("CLIENT #{state.name}: error - #{msg}")
        IO.inspect(payload, label: "Payload")
        send(state.reply_to, {:error, msg})

      decoded_packet ->
        case decoded_packet["ilp_packet_type"] do
          12 -> IO.puts("CLIENT #{state.name}: received prepare packet")
          13 -> IO.puts("CLIENT #{state.name}: received fulfill packet")
          14 -> IO.puts("CLIENT #{state.name}: received reject packet")
        end

        send(state.reply_to, {:ok, decoded_packet})
    end

    {:noreply, state}
  end

  def handle_info({:response, error}, state) do
    send(state.reply_to, error)
    {:noreply, state}
  end
end
