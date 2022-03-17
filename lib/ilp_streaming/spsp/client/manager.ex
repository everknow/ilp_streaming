defmodule SPSP.Client.Manager do
  @moduledoc """
  Client process supervisor that provides an API to start SPSP clients.
  """

  alias SPSP.Client.Worker

  use DynamicSupervisor

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_child do
    name = :"stream_client-#{UUID.uuid4()}"

    case DynamicSupervisor.start_child(__MODULE__, {Worker, name: name}) do
      {:ok, _} -> {:ok, name}
      error -> error
    end
  end

  @impl true
  def init(_init_arg) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end
