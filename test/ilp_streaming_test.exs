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

 test "encode_tests" do

  assert {:error, "could not decode arg to map<String,Term>"} = IlpStreaming.encode(%{})
  assert {:error, "sequence missing"} = IlpStreaming.encode(%{"sequence" => 1})
  assert {:error, "packet_type missing"} = IlpStreaming.encode(%{"ilp_packet_type" => 12})
  assert {:error, "prepare_amount missing"} = IlpStreaming.encode(%{"prepare_amount" => 99})

  assert {:error, "frames are missing"} = IlpStreaming.encode(%{
    "type" => "connection_close",
    "code" => 1,
    "message" => "oop"
  },
  %{
    "type" => "connection_new_address",
    "source_account" => "example.blah"
  },
  %{
    "type" => "connection_asset_details",
    "source_asset_code" => "XYZ",
    "source_asset_scale" => 9
  },
  %{
    "type" => "connection_max_data",
    "max_offset" => 8766
  },
  %{
    "type" => "connection_data_blocked",
    "max_offset" => 888
  },
  %{
    "type" => "connection_max_stream_id",
    "max_stream_id" => 34
  },
  %{
    "type" => "stream_close",
    "stream_id" => 76,
    "code" => 1,
    "message" => "pippo"
  },
  %{
    "type" => "stream_money",
    "stream_id" => 88,
    "shares" => 99
  },
  %{
    "type" => "stream_max_money",
    "stream_id" => 11,
    "receive_max" => 987,
    "total_received" => 500
  },
  %{
    "type" => "stream_money_blocked",
    "stream_id" => 66,
    "send_max" => 2000,
    "total_sent" => 6000
  },
  %{
    "type" => "stream_data",
    "stream_id" => 35,
    "offset" => 9000,
    "data" => "pino"
  },
  %{
    "type" => "stream_max_data",
    "stream_id" => 7,
    "max_offset" => 4444
  },
  %{
    "type" => "stream_data_blocked",
    "stream_id" => 42,
    "max_offset" => 6942
  })
 end


end
