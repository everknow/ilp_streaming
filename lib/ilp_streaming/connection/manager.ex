defmodule IlpStreaming.Connection.Manager do
  @moduledoc """
  Connection processes' supervisor.
  """

  alias IlpStreaming.Connection.Http

  use DynamicSupervisor

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_child do
    spec = {Http, name: "http-conn-#{UUID.uuid4()}"}
    DynamicSupervisor.start_child(__MODULE__, spec)
  end

  @impl true
  def init(_init_arg), do: DynamicSupervisor.init(strategy: :one_for_one)
end
