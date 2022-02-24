defmodule IlpStreamingTest do
  use ExUnit.Case

  test "encode" do
    assert is_list(IlpStreaming.encode(IlpStreamingHelper.connection()))
  end

  test "decode" do
    expected_result = IlpStreamingHelper.connection()

    result =
      IlpStreamingHelper.connection()
      |> IlpStreaming.encode()
      |> :binary.list_to_bin()
      |> IlpStreaming.decode()

    assert result == expected_result
  end
end
