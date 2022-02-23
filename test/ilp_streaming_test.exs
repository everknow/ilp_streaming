defmodule IlpStreamingTest do
  use ExUnit.Case

  test "encode" do
    assert IlpStreaming.encode(IlpStreamingHelper.connection()) == true
  end

end
