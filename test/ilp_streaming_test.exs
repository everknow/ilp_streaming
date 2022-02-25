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
    assert {:error, "could not decode params to map<String,Term>"} = IlpStreaming.encode("")
    assert {:error, "sequence is missing"} = IlpStreaming.encode(%{})
    assert {:error, "ilp_packet_type is missing"} = IlpStreaming.encode(%{"sequence" => 1})

    assert {:error, "prepare_amount is missing"} =
             IlpStreaming.encode(%{"sequence" => 1, "ilp_packet_type" => 12})

    assert {:error, "frames are missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99
             })

    assert {:error, "could not decode sequence"} =
             IlpStreaming.encode(%{
               "sequence" => false,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => []
             })

    assert {:error, "could not decode packet type as binary"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => "something",
               "prepare_amount" => 99,
               "frames" => []
             })

    assert {:error, "could not decode prepare_amount"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => "hello",
               "frames" => []
             })

    assert {:error, "could not decode frames"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [true]
             })

    assert {:error, "type is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{}]
             })

    assert {:error, "type not binary"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => 1}]
             })

    assert {:error, "Error unexpected frame_type"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "hi"}]
             })

    assert {:error, "connection_close > code is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_close"}]
             })

    assert {:error, "connection_close > message is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_close", "code" => 1}]
             })

    assert {:error, "connection_close > could not decode code"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_close", "code" => true, "message" => 2}]
             })

    assert {:error, "connection_close > could not decode message"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_close", "code" => 2, "message" => 2}]
             })

    assert {:error, "connection_new_address > source_account is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_new_address"}]
             })

    assert {:error, "connection_new_address > could not decode source_account"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_new_address", "source_account" => 1}]
             })

    assert {:error, "connection_new_address > could not decode the source_account"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "connection_new_address", "source_account" => "ss", "message" => 2}
               ]
             })

    assert {:error, "connection_new_address > could not decode the source_account"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "connection_new_address",
                   "source_account" => "ss",
                   "message" => "example.blah",
                   "result" => ""
                 }
               ]
             })

    assert {:error, "connection_asset_details > source_asset_code is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_asset_details"}]
             })

    assert {:error, "connection_asset_details > source_asset_scale is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_asset_details", "source_asset_code" => 1}]
             })

    assert {:error, "connection_asset_details > could not decode source_asset_code"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "connection_asset_details",
                   "source_asset_scale" => 1,
                   "source_asset_code" => true
                 }
               ]
             })

    assert {:error, "connection_asset_details > could not decode source_asset_code"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "connection_asset_details",
                   "source_asset_scale" => true,
                   "source_asset_code" => 1
                 }
               ]
             })

    assert {:error, "connection_max_data > max_offset is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_max_data"}]
             })

    assert {:error, "connection_max_data > could not decode max_offset"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_max_data", "max_offset" => ""}]
             })

    assert {:error, "connection_data_blocked > max_offset is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_data_blocked"}]
             })

    assert {:error, "connection_data_blocked > could not decode max_offset"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_data_blocked", "max_offset" => ""}]
             })

    assert {:error, "connection_max_stream_id > max_stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_max_stream_id"}]
             })

    assert {:error, "connection_max_stream_id > could not decode max_stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_max_stream_id", "max_stream_id" => ""}]
             })

    assert {:error, "connection_stream_id_blocked > max_stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_stream_id_blocked"}]
             })

    assert {:error, "connection_stream_id_blocked > could not decode max_stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "connection_stream_id_blocked", "max_stream_id" => ""}]
             })

    assert {:error, "stream_close > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_close"}]
             })

    assert {:error, "stream_close > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_close", "code" => ""}]
             })

    assert {:error, "stream_close > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_close", "code" => "", "message" => 1}]
             })

    assert {:error, "stream_close > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_close", "code" => "", "message" => 1}]
             })

    assert {:error, "stream_money > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money"}]
             })

    assert {:error, "stream_money > shares is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money", "stream_id" => ""}]
             })

    assert {:error, "stream_money > could not decode stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money", "stream_id" => "", "shares" => 2}]
             })

    assert {:error, "stream_money > could not decode shares"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money", "stream_id" => 34, "shares" => ""}]
             })

    assert {:error, "stream_max_money > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_max_money"}]
             })

    assert {:error, "stream_max_money > receive_max is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_max_money", "stream_id" => ""}]
             })

    assert {:error, "stream_max_money > total_received is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_max_money", "stream_id" => 1, "receive_max" => ""}
               ]
             })

    assert {:error, "stream_max_money > could not decode total_received"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "stream_max_money",
                   "stream_id" => 1,
                   "receive_max" => 11,
                   "total_received" => ""
                 }
               ]
             })

    assert {:error, "stream_max_money > could not decode receive_max"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "stream_max_money",
                   "stream_id" => 1,
                   "receive_max" => "",
                   "total_received" => 10
                 }
               ]
             })

    assert {:error, "stream_money_blocked > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money_blocked"}]
             })

    assert {:error, "stream_money_blocked > send_max is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_money_blocked", "stream_id" => ""}]
             })

    assert {:error, "stream_money_blocked > total_sent is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_money_blocked", "stream_id" => 1, "send_max" => ""}
               ]
             })

    assert {:error, "stream_money_blocked > could not decode total_sent"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{
                   "type" => "stream_money_blocked",
                   "stream_id" => 1,
                   "send_max" => 10,
                   "total_sent" => ""
                 }
               ]
             })

    assert {:error, "stream_data > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_data"}]
             })

    assert {:error, "stream_data > offset is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_data", "stream_id" => ""}]
             })

    assert {:error, "stream_data > data is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_data", "stream_id" => 1, "offset" => ""}]
             })

    assert {:error, "stream_data > could not decode stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_data", "stream_id" => "", "offset" => 1, "data" => ""}
               ]
             })

    assert {:error, "stream_data > could not decode offset"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_data", "stream_id" => 1, "offset" => true, "data" => ""}
               ]
             })

    assert {:error, "could not decode data"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_data", "stream_id" => 1, "offset" => 1, "data" => true}
               ]
             })

    assert {:error, "stream_max_data > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_max_data"}]
             })

    assert {:error, "stream_max_data > max_offset is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_max_data", "stream_id" => 1}]
             })

    assert {:error, "stream_max_data > could not decode stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_max_data", "stream_id" => true, "max_offset" => 1}
               ]
             })

    assert {:error, "stream_max_data > could not decode max_offset"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_max_data", "stream_id" => 222, "max_offset" => true}
               ]
             })

    assert {:error, "stream_data_blocked > stream_id is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_data_blocked"}]
             })

    assert {:error, "stream_data_blocked > max_offset is missing"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [%{"type" => "stream_data_blocked", "stream_id" => 233}]
             })

    assert {:error, "stream_data_blocked > could not decode stream_id"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_data_blocked", "stream_id" => true, "max_offset" => 33}
               ]
             })

    assert {:error, "stream_data_blocked > could not decode max_offset"} =
             IlpStreaming.encode(%{
               "sequence" => 1,
               "ilp_packet_type" => 12,
               "prepare_amount" => 99,
               "frames" => [
                 %{"type" => "stream_data_blocked", "stream_id" => 2, "max_offset" => false}
               ]
             })
  end
end
