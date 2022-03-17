defmodule SPSP.Client do
  @moduledoc false

  def send_money() do
    IlpStreaming.Connection.Http.send_prepare()
  end
end
