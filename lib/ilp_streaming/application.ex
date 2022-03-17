defmodule IlpStreaming.Application do
  @moduledoc false

  use Application

  require Logger

  @impl true
  def start(_type, _args) do
    conn_opts = {:http, "localhost", 8000}

    children = [
      {Plug.Cowboy, scheme: :http, plug: SPSP.Server.Plug, options: [port: 8000]},
      {IlpStreaming.Connection.Http, conn_opts}
    ]

    opts = [strategy: :one_for_one, name: IlpStreaming.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
