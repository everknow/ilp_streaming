defmodule IlpStreaming.Connection.Http do
  use GenServer

  require Logger

  defstruct [:conn, requests: %{}]

  def start_link({scheme, host, port}) do
    GenServer.start_link(__MODULE__, {scheme, host, port}, name: __MODULE__)
  end

  def request(pid, method, path, headers, body) do
    GenServer.call(pid, {:request, method, path, headers, body})
  end

  def send_prepare(prepare \\ "") do
    GenServer.call(__MODULE__, {:request, "POST", "/", [{"content-type", "application/json"}], prepare})
  end

  ## Callbacks

  @impl true
  def init({scheme, host, port}) do
    case Mint.HTTP.connect(scheme, host, port) do
      {:ok, conn} ->
        state = %__MODULE__{conn: conn}
        {:ok, state}

      {:error, reason} ->
        {:stop, reason}
    end
  end

  @impl true
  def handle_call({:request, method, path, headers, body}, from, state) do
    # In both the successful case and the error case, we make sure to update the connection
    # struct in the state since the connection is an immutable data structure.
    case Mint.HTTP.request(state.conn, method, path, headers, body) do
      {:ok, conn, request_ref} ->
        state = put_in(state.conn, conn)
        # We store the caller this request belongs to and an empty map as the response.
        # The map will be filled with status code, headers, and so on.
        state = put_in(state.requests[request_ref], %{from: from, response: %{}})
        {:noreply, state}

      {:error, conn, reason} ->
        state = put_in(state.conn, conn)
        {:reply, {:error, reason}, state}
    end
  end

  @impl true
  def handle_info(message, state) do
    case Mint.HTTP.stream(state.conn, message) do
      {:error, _, %Mint.TransportError{reason: :closed}, []} ->
        {:ok, conn} = Mint.HTTP.connect(:http, "localhost", 8000)
        state = %__MODULE__{conn: conn}
        {:noreply, state}

      :unknown ->
        _ = Logger.error(fn -> "Received unknown message: " <> inspect(message) end)
        {:noreply, state}

      {:ok, conn, responses} ->
        state = put_in(state.conn, conn)
        state = Enum.reduce(responses, state, &process_response/2)
        {:noreply, state}
    end
  end

  defp process_response({:status, request_ref, status}, state) do
    put_in(state.requests[request_ref].response[:status], status)
  end

  defp process_response({:headers, request_ref, headers}, state) do
    put_in(state.requests[request_ref].response[:headers], headers)
  end

  defp process_response({:data, request_ref, new_data}, state) do
    update_in(state.requests[request_ref].response[:data], fn data -> (data || "") <> new_data end)
  end

  # When the request is done, we use GenServer.reply/2 to reply to the caller that was
  # blocked waiting on this request.
  defp process_response({:done, request_ref}, state) do
    {%{response: response, from: from}, state} = pop_in(state.requests[request_ref])
    GenServer.reply(from, {:ok, response})
    state
  end

  # A request can also error, but we're not handling the erroneous responses for
  # brevity.
end
