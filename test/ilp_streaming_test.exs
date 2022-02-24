defmodule IlpStreamingTest do
  use ExUnit.Case

  test "encode" do
    assert is_list(IlpStreaming.encode(IlpStreamingHelper.connection()))
  end
end
