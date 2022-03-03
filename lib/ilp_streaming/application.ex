defmodule IlpStreaming.Application do
  @moduledoc false

  use Application

  require Logger

  @impl true
  def start(_type, _args) do
    children = [
      IlpStreaming.Client.Manager,
      IlpStreaming.Server.Manager
    ]

    opts = [strategy: :one_for_one, name: IlpStreaming.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
