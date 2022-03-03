defmodule IlpStreaming.Server.Manager do
  @moduledoc """
  Server process supervisor that provides an API to start listening STREAM
  endpoints.
  """

  alias IlpStreaming.Server.Worker

  use DynamicSupervisor

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_child do
    spec = {Worker, name: "stream_server-#{UUID.uuid4()}"}
    DynamicSupervisor.start_child(__MODULE__, spec)
  end

  @impl true
  def init(_init_arg) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end
