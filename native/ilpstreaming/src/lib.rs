// imports
use bytes::BufMut;
use bytes::BytesMut;
use interledger::packet::oer::MutBufOerExt;
use interledger::packet::Address;
use interledger::packet::PacketType as IlpPacketType;
use interledger::stream::packet::FrameType;
use interledger::stream::packet::{
    ConnectionAssetDetailsFrame, ConnectionCloseFrame, ConnectionDataBlockedFrame,
    ConnectionMaxDataFrame, ConnectionMaxStreamIdFrame, ConnectionNewAddressFrame,
    ConnectionStreamIdBlockedFrame, ErrorCode, Frame, SerializableFrame, StreamCloseFrame,
    StreamDataBlockedFrame, StreamDataFrame, StreamMaxDataFrame, StreamMaxMoneyFrame,
    StreamMoneyBlockedFrame, StreamMoneyFrame, StreamPacket, StreamPacketBuilder, UnknownFrameData,
};
use rustler::Binary;
use rustler::{Encoder, Env, Error, NifResult, Term};
use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

rustler::init!("Elixir.IlpStreaming", [encode]); //decode missing

// macro space
#[macro_export]
macro_rules! err {
    ( $( $x:expr ),* ) => {
        {
            $(
                Err(Error::Term(Box::new($x)))
            )*
        }
    };
}
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            $(
                Error::Term(Box::new($x))
            )*
        }
    };
}

// functions..

#[rustler::nif(schedule = "DirtyCpu")]
fn encode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let m = arg
        .decode::<HashMap<String, Term>>()
        .or(err!("could not decode arg to map<String,Term>"))?;

    // get fields
    let s = m.get("sequence").ok_or(error!("sequence is missing"))?;
    let ipt = m
        .get("ilp_packet_type")
        .ok_or(error!("ilp_packet_type is missing"))?;
    let pa = m
        .get("ilp_packet_type")
        .ok_or(error!("prepare_amount is missing"))?;
    let f = m.get("frames").ok_or(error!("frames are missing"))?;

    // transform
    let sequence = s.decode::<u64>().or(err!("could not decode sequence"))?;
    let prepare_amount = pa
        .decode::<u64>()
        .or(err!("could not decode prepare_amount"))?;
    let fms = f
        .decode::<Vec<HashMap<String, Term>>>()
        .or(err!("could not decode frames"))?;

    //special transform
    let mut frames = <Vec<Frame>>::new();

    for frame in fms {
        let hm = arg
            .decode::<HashMap<String, Term>>()
            .or(err!("could not decode arg to map<String,Term>"))?;
        let t = hm.get("type").ok_or(error!("unknow type"))?;
         match t.decode::<&str>().or(err!("type not binary"))? {
            "connection_close_frame" => {
                // get fields
                let c = m.get("code").ok_or(error!("code is missing"))?;
                let m = m
                    .get("from_username")
                    .ok_or(error!("from_username is missing"))?;
                // transform
                let codes = c.decode::<u8>().or(err!("could not decode to_username"))?; //Type ErrorCode
                let message = m
                    .decode::<&str>()
                    .or(err!("could not decode from_username"))?;
                //special transform
                let code = ErrorCode::from(codes);
                let result = Frame::ConnectionClose(ConnectionCloseFrame { code, message });
                if let Frame::ConnectionClose(result) = result {
                    frames.push(Frame::ConnectionClose(result));
                }
            }
            "connection_new_address_frame" => {
                // get fields
                let sa = m
                    .get("source_account")
                    .ok_or(error!("source_account is missing"))?;
                // transform
                let source_accounts = sa
                    .decode::<&str>()
                    .or(err!("could not decode source_account"))?;
                //special transform
                let source_account = Address::from_str(source_accounts)
                    .or(err!("could not decode the source_account"))?;
                let result = Frame::ConnectionNewAddress(ConnectionNewAddressFrame { source_account });
                if let Frame::ConnectionNewAddress(result) = result {
                    frames.push(Frame::ConnectionNewAddress(result));
                }
            }
            "connection_asset_details_frame" => {
                // get fields
                let sac = m
                    .get("source_asset_code")
                    .ok_or(error!("source_asset_code is missing"))?;
                let sas = m
                    .get("source_asset_scale")
                    .ok_or(error!("source_asset_scale is missing"))?;
                // transform
                let source_asset_code = sac
                    .decode::<&str>()
                    .or(err!("could not decode source_asset_code"))?;
                let source_asset_scale = sas
                    .decode::<u8>()
                    .or(err!("could not decode source_asset_code"))?;
                let result = Frame::ConnectionAssetDetails(ConnectionAssetDetailsFrame{
                    source_asset_code,
                    source_asset_scale,
                });
                if let Frame::ConnectionAssetDetails(result) = result {
                    frames.push(Frame::ConnectionAssetDetails(result));
                }
            }
            "connection_max_data_frame" => {
                // get fields
                let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
                // transform
                let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;
                let result = Frame::ConnectionMaxData(ConnectionMaxDataFrame { max_offset });
                if let Frame::ConnectionMaxData(result) = result {
                    frames.push(Frame::ConnectionMaxData(result));
                }
            }
            "connection_data_blocked_frame" => {
                // get fields
                let mob = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
                // transform
                let max_offset = mob
                    .decode::<u64>()
                    .or(err!("could not decode max_offset"))?;
                let result = Frame::ConnectionDataBlocked(ConnectionDataBlockedFrame{ max_offset });
                if let Frame::ConnectionDataBlocked(result) = result {
                    frames.push(Frame::ConnectionDataBlocked(result));
                }
            }
            "connection_max_stream_id_frame" => {
                // get fields
                let msi = m
                    .get("max_stream_id")
                    .ok_or(error!("max_stream_id is missing"))?;
                // transform
                let max_stream_id = msi
                    .decode::<u64>()
                    .or(err!("could not decode max_stream_id"))?;
                let result = Frame::ConnectionMaxStreamId(ConnectionMaxStreamIdFrame{ max_stream_id });
                if let Frame::ConnectionMaxStreamId(result) = result {
                    frames.push(Frame::ConnectionMaxStreamId(result));
                }
            }
            "connection_stream_id_blocked_frame" => {
                // get fields
                let msis = m
                    .get("max_stream_id")
                    .ok_or(error!("max_stream_id is missing"))?;
                // transform
                let max_stream_id = msis
                    .decode::<u64>()
                    .or(err!("could not decode max_stream_id"))?;
              let result =  Frame::ConnectionStreamIdBlocked(ConnectionStreamIdBlockedFrame { max_stream_id });
                if let Frame::ConnectionStreamIdBlocked(result) = result {
                    frames.push(Frame::ConnectionStreamIdBlocked(result));
                }
            }
            "stream_close_frame" => {
                // get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let c = m.get("code").ok_or(error!("code is missing"))?;
                let m = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let codes = c.decode::<u8>().or(err!("could not decode code"))?;
                let message = m.decode::<&str>().or(err!("could not decode message"))?;
                //special transform
                let code = ErrorCode::from(codes);
                let result = Frame::StreamClose(StreamCloseFrame {
                    stream_id,
                    code,
                    message,
                });
                if let Frame::StreamClose(result) = result {
                    frames.push(Frame::StreamClose(result));
                }
            }
            "stream_money_frame" => {
                // get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let s = m.get("shares").ok_or(error!("shares is missing"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let shares = s.decode::<u64>().or(err!("could not decode shares"))?;
                let result = Frame::StreamMoney(StreamMoneyFrame { stream_id, shares });
                if let Frame::StreamMoney(result) = result {
                    frames.push(Frame::StreamMoney(result));
                }
            }
            "stream_max_money_frame" => {
                // get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let rm = m
                    .get("receive_max")
                    .ok_or(error!("receive_max is missing"))?;
                let tr = m
                    .get("total_received")
                    .ok_or(error!("total_received is missing"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let receive_max = rm
                    .decode::<u64>()
                    .or(err!("could not decode receive_max"))?;
                let total_received = tr
                    .decode::<u64>()
                    .or(err!("could not decode total_received"))?;
                let result = Frame::StreamMaxMoney(StreamMaxMoneyFrame {
                    stream_id,
                    receive_max,
                    total_received,
                });
                if let Frame::StreamMaxMoney(result) = result {
                    frames.push(Frame::StreamMaxMoney(result));
                }
            }
            "stream_money_blocked_frame" => {
                //get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let sm = m.get("send_max").ok_or(error!("send_max is missing"))?;
                let ts = m
                    .get("total_sent")
                    .ok_or(error!("total_received is total_sen"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let send_max = sm.decode::<u64>().or(err!("could not decode send_max"))?;
                let total_sent = ts.decode::<u64>().or(err!("could not decode total_sent"))?;
                let result = Frame::StreamMoneyBlocked(StreamMoneyBlockedFrame {
                    stream_id,
                    send_max,
                    total_sent,
                });
                if let Frame::StreamMoneyBlocked(result) = result {
                    frames.push(Frame::StreamMoneyBlocked(result));
                }
            }
            "stream_data_frame" => {
                // get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let o = m.get("offset").ok_or(error!("offset is missing"))?;
                let d = m.get("data").ok_or(error!("data is total_sen"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let offset = o.decode::<u64>().or(err!("could not decode offset"))?;
                let data = d
                    .into_binary()
                    .or(err!("could not decode data"))?
                    .as_slice();
                //special transform
                let result = Frame::StreamData(StreamDataFrame {
                    stream_id,
                    offset,
                    data,
                });
                if let Frame::StreamData(result) = result {
                    frames.push(Frame::StreamData(result));
                }
            }
            "stream_max_data_frame" => {
                // get fields
                let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;
                let result = Frame::StreamMaxData(StreamMaxDataFrame {
                    stream_id,
                    max_offset,
                });
                if let Frame::StreamMaxData(result) = result {
                    frames.push(Frame::StreamMaxData(result));
                }
            }
            "stream_data_blocked_frame" => {
                // get fields
                let sis = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
                let mos = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
                // transform
                let stream_id = sis.decode::<u64>().or(err!("could not decode stream_id"))?;
                let max_offset = mos
                    .decode::<u64>()
                    .or(err!("could not decode max_offset"))?;
                let result = Frame::StreamDataBlocked(StreamDataBlockedFrame {
                    stream_id,
                    max_offset,
                });
                if let Frame::StreamDataBlocked(result) = result {
                    frames.push(Frame::StreamDataBlocked(result));
                }
            }
            _ => {
                return Ok("Error".encode(env));
            }
        };
    }

    StreamPacketBuilder {
        sequence: sequence,
        ilp_packet_type: IlpPacketType::try_from(ipt.into_binary()?.as_slice()).unwrap(),
        prepare_amount: prepare_amount,
        frames: &frames,
    }
    .build();

    Ok("result".encode(env))
}

// #[rustler::nif(schedule = "DirtyCpu")]
// fn decode<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {

//     match StreamPacket::from_encrypted(" ".as_bytes(), BytesMut::from(" ")).unwrap(){

//         "connection_close_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "connection_new_address_frame" => {
//             Ok("No idea to return ".encode(env))
//         }
//         "connection_asset_details_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "connection_max_data_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "connection_data_blocked_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "connection_max_stream_id_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "connection_stream_id_blocked_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_close_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_money_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_max_money_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_money_blocked_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_data_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_max_data_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         "stream_data_blocked_frame" => {

//             Ok("No idea to return ".encode(env))
//         }
//         _ => {
//             err!("type not recognised")
//         }
//     }
