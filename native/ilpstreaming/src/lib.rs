//imports
use bytes::BytesMut;
use interledger::packet::Address;
use interledger::packet::PacketType as IlpPacketType;
use interledger::stream::packet::*;
use rustler::{Binary, Encoder, Env, Error, NifResult, Term};
use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

rustler::init!("Elixir.IlpStreaming", [encode, decode]);

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
fn encode<'a>(env: Env<'a>, params: Term, key: Binary) -> NifResult<Term<'a>> {
    let m = params
        .decode::<HashMap<String, Term>>()
        .or(err!("could not decode params to map<String,Term>"))?;

    // get fields
    let s = m.get("sequence").ok_or(error!("sequence is missing"))?;
    let ipt = m
        .get("ilp_packet_type")
        .ok_or(error!("ilp_packet_type is missing"))?;
    let pa = m
        .get("prepare_amount")
        .ok_or(error!("prepare_amount is missing"))?;
    let f = m.get("frames").ok_or(error!("frames are missing"))?;

    // transform
    let sequence = s.decode::<u64>().or(err!("could not decode sequence"))?;
    let packet_type = IlpPacketType::try_from(
        ipt.decode::<u8>()
            .or(err!("could not decode packet type as binary"))?,
    )
    .or(err!("could not decode packet type as IlpPacketType"))?;

    let prepare_amount = pa
        .decode::<u64>()
        .or(err!("could not decode prepare_amount"))?;
    let fms = f
        .decode::<Vec<HashMap<String, Term>>>()
        .or(err!("could not decode frames"))?;

    //special transform
    let mut frames = <Vec<Frame>>::new();

    for hm in fms {
        let t = hm.get("type").ok_or(error!("type is missing"))?;
        match t.decode::<&str>().or(err!("type not binary"))? {
            "connection_close_frame" => {
                // get fields
                let c = hm
                    .get("code")
                    .ok_or(error!("connection_close_frame > code is missing"))?;
                let ms = hm
                    .get("message")
                    .ok_or(error!("connection_close_frame > message is missing "))?;
                // transform
                let codes = c
                    .decode::<u8>()
                    .or(err!("connection_close_frame > could not decode code"))?;
                let message = ms
                    .decode::<&str>()
                    .or(err!("connection_close_frame > could not decode message"))?;
                //special transform
                let code = ErrorCode::from(codes);
                let result = Frame::ConnectionClose(ConnectionCloseFrame { code, message });
                if let Frame::ConnectionClose(result) = result {
                    frames.push(Frame::ConnectionClose(result));
                }
            }
            "connection_new_address_frame" => {
                // get fields
                let sa = hm.get("source_account").ok_or(error!(
                    "connection_new_address_frame > source_account is missing"
                ))?;
                // transform
                let source_accounts = sa.decode::<&str>().or(err!(
                    "connection_new_address_frame > could not decode source_account"
                ))?;
                //special transform
                let source_account = Address::from_str(source_accounts).or(err!(
                    "connection_new_address_frame > could not decode the source_account"
                ))?;
                let result =
                    Frame::ConnectionNewAddress(ConnectionNewAddressFrame { source_account });
                if let Frame::ConnectionNewAddress(result) = result {
                    frames.push(Frame::ConnectionNewAddress(result));
                }
            }
            "connection_asset_details_frame" => {
                // get fields
                let sac = hm.get("source_asset_code").ok_or(error!(
                    "connection_asset_details_frame > source_asset_code is missing"
                ))?;
                let sas = hm.get("source_asset_scale").ok_or(error!(
                    "connection_asset_details_frame > source_asset_scale is missing"
                ))?;
                // transform
                let source_asset_code = sac.decode::<&str>().or(err!(
                    "connection_asset_details_frame > could not decode source_asset_code"
                ))?;
                let source_asset_scale = sas.decode::<u8>().or(err!(
                    "connection_asset_details_frame > could not decode scale"
                ))?;
                let result = Frame::ConnectionAssetDetails(ConnectionAssetDetailsFrame {
                    source_asset_code,
                    source_asset_scale,
                });
                if let Frame::ConnectionAssetDetails(result) = result {
                    frames.push(Frame::ConnectionAssetDetails(result));
                }
            }
            "connection_max_data_frame" => {
                // get fields
                let mo = hm
                    .get("max_offset")
                    .ok_or(error!("connection_max_data_frame > max_offset is missing"))?;
                // transform
                let max_offset = mo.decode::<u64>().or(err!(
                    "connection_max_data_frame > could not decode max_offset"
                ))?;
                let result = Frame::ConnectionMaxData(ConnectionMaxDataFrame { max_offset });
                if let Frame::ConnectionMaxData(result) = result {
                    frames.push(Frame::ConnectionMaxData(result));
                }
            }
            "connection_data_blocked_frame" => {
                // get fields
                let mob = hm.get("max_offset").ok_or(error!(
                    "connection_data_blocked_frame > max_offset is missing"
                ))?;
                // transform
                let max_offset = mob.decode::<u64>().or(err!(
                    "connection_data_blocked_frame > could not decode max_offset"
                ))?;
                let result =
                    Frame::ConnectionDataBlocked(ConnectionDataBlockedFrame { max_offset });
                if let Frame::ConnectionDataBlocked(result) = result {
                    frames.push(Frame::ConnectionDataBlocked(result));
                }
            }
            "connection_max_stream_id_frame" => {
                // get fields
                let msi = hm.get("max_stream_id").ok_or(error!(
                    "connection_max_stream_id_frame > max_stream_id is missing"
                ))?;
                // transform
                let max_stream_id = msi.decode::<u64>().or(err!(
                    "connection_max_stream_id_frame > could not decode max_stream_id"
                ))?;
                let result =
                    Frame::ConnectionMaxStreamId(ConnectionMaxStreamIdFrame { max_stream_id });
                if let Frame::ConnectionMaxStreamId(result) = result {
                    frames.push(Frame::ConnectionMaxStreamId(result));
                }
            }
            "connection_stream_id_blocked_frame" => {
                // get fields
                let msis = hm.get("max_stream_id").ok_or(error!(
                    "connection_stream_id_blocked_frame > max_stream_id is missing"
                ))?;
                // transform
                let max_stream_id = msis.decode::<u64>().or(err!(
                    "connection_stream_id_blocked_frame > could not decode max_stream_id"
                ))?;
                let result = Frame::ConnectionStreamIdBlocked(ConnectionStreamIdBlockedFrame {
                    max_stream_id,
                });
                if let Frame::ConnectionStreamIdBlocked(result) = result {
                    frames.push(Frame::ConnectionStreamIdBlocked(result));
                }
            }
            "stream_close_frame" => {
                // get fields
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_close_frame > stream_id is missing"))?;
                let c = hm
                    .get("code")
                    .ok_or(error!("stream_close_frame > code is missing"))?;
                let m = hm
                    .get("message")
                    .ok_or(error!("stream_close_frame > message is missing"))?;
                // transform
                let stream_id = si
                    .decode::<u64>()
                    .or(err!("stream_close_frame > could not decode stream_id"))?;
                let codes = c
                    .decode::<u8>()
                    .or(err!("stream_close_frame > could not decode code"))?;
                let message = m
                    .decode::<&str>()
                    .or(err!("stream_close_frame > could not decode message"))?;
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
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_money_frame > stream_id is missing"))?;
                let s = hm
                    .get("shares")
                    .ok_or(error!("stream_money_frame > shares is missing"))?;
                // transform
                let stream_id = si
                    .decode::<u64>()
                    .or(err!("stream_money_frame > could not decode stream_id"))?;
                let shares = s
                    .decode::<u64>()
                    .or(err!("stream_money_frame > could not decode shares"))?;
                let result = Frame::StreamMoney(StreamMoneyFrame { stream_id, shares });
                if let Frame::StreamMoney(result) = result {
                    frames.push(Frame::StreamMoney(result));
                }
            }
            "stream_max_money_frame" => {
                // get fields
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_max_money_frame > stream_id is missing"))?;
                let rm = hm
                    .get("receive_max")
                    .ok_or(error!("stream_max_money_frame > receive_max is missing"))?;
                let tr = hm
                    .get("total_received")
                    .ok_or(error!("stream_max_money_frame > total_received is missing"))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
                let receive_max = rm.decode::<u64>().or(err!(
                    "stream_max_money_frame > could not decode receive_max"
                ))?;
                let total_received = tr.decode::<u64>().or(err!(
                    "stream_max_money_frame > could not decode total_received"
                ))?;
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
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_money_blocked_frame > stream_id is missing"))?;
                let sm = hm
                    .get("send_max")
                    .ok_or(error!("stream_money_blocked_frame > send_max is missing"))?;
                let ts = hm.get("total_sent").ok_or(error!(
                    "stream_money_blocked_frame > total_received is total_sen"
                ))?;
                // transform
                let stream_id = si.decode::<u64>().or(err!(
                    "stream_money_blocked_frame > could not decode stream_id"
                ))?;
                let send_max = sm.decode::<u64>().or(err!(
                    "stream_money_blocked_frame > could not decode send_max"
                ))?;
                let total_sent = ts.decode::<u64>().or(err!(
                    "stream_money_blocked_frame > could not decode total_sent"
                ))?;
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
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_data_frame > stream_id is missing"))?;
                let o = hm
                    .get("offset")
                    .ok_or(error!("stream_data_frame > offset is missing"))?;
                let d = hm
                    .get("data")
                    .ok_or(error!("stream_data_frame > data is total_sen"))?;
                // transform
                let stream_id = si
                    .decode::<u64>()
                    .or(err!("stream_data_frame > could not decode stream_id"))?;
                let offset = o
                    .decode::<u64>()
                    .or(err!("stream_data_frame > could not decode offset"))?;
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
                let si = hm
                    .get("stream_id")
                    .ok_or(error!("stream_max_data_frame > stream_id is missing"))?;
                let mo = hm
                    .get("max_offset")
                    .ok_or(error!("stream_max_data_frame > max_offset is missing"))?;
                // transform
                let stream_id = si
                    .decode::<u64>()
                    .or(err!("stream_max_data_frame > could not decode stream_id"))?;
                let max_offset = mo
                    .decode::<u64>()
                    .or(err!("stream_max_data_frame > could not decode max_offset"))?;
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
                let sis = hm
                    .get("stream_id")
                    .ok_or(error!("stream_data_blocked_frame > stream_id is missing"))?;
                let mos = hm
                    .get("max_offset")
                    .ok_or(error!("stream_data_blocked_frame > max_offset is missing"))?;
                // transform
                let stream_id = sis.decode::<u64>().or(err!(
                    "stream_data_blocked_frame > could not decode stream_id"
                ))?;
                let max_offset = mos.decode::<u64>().or(err!(
                    "stream_data_blocked_frame > could not decode max_offset"
                ))?;
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

    let shared_key = key.as_slice();

    let k = StreamPacketBuilder {
        sequence: sequence,
        ilp_packet_type: packet_type,
        prepare_amount: prepare_amount,
        frames: &frames,
    }
    .build();

    Ok(k.into_encrypted(&shared_key).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>, stream: Binary, key: Binary) -> NifResult<Term<'a>> {
    let shared_secret = key.as_slice();
    let ciphertext = BytesMut::from(stream.as_slice());

    let stream_packet = StreamPacket::from_encrypted(shared_secret, ciphertext)
        .or(err!("could not decode stream to StreamPacket"))?;

    let mut resulting_stream: HashMap<String, Term> = HashMap::new();

    let sequence = stream_packet.sequence();
    let prepare_amount = stream_packet.prepare_amount();
    let ilp_packet_type = stream_packet.ilp_packet_type() as u8;
    let mut resulting_frames = Vec::new();

    for frame in stream_packet.frames() {
        let decoded = decode_frame(env, frame);
        resulting_frames.push(decoded);
    }

    resulting_stream.insert("sequence".to_string(), sequence.encode(env));
    resulting_stream.insert("prepare_amount".to_string(), prepare_amount.encode(env));
    resulting_stream.insert("ilp_packet_type".to_string(), ilp_packet_type.encode(env));
    resulting_stream.insert("frames".to_string(), resulting_frames.encode(env));

    Ok(resulting_stream.encode(env))
}

fn decode_frame<'a>(env: Env<'a>, frame: Frame) -> HashMap<&'a str, Term<'a>> {
    let mut decoded_frame = HashMap::new();

    match frame {
        Frame::ConnectionClose(ref frame) => {
            decoded_frame.insert("type", "connection_close".encode(env));
            decoded_frame.insert("code", u8::from(frame.code).encode(env));
            decoded_frame.insert("message", frame.message.encode(env));
        }
        Frame::ConnectionNewAddress(ref frame) => {
            decoded_frame.insert("type", "connection_new_address".encode(env));
            decoded_frame.insert("code", frame.source_account.clone().to_string().encode(env));
        }
        Frame::ConnectionAssetDetails(ref frame) => {
            decoded_frame.insert("type", "connection_asset_details".encode(env));
            decoded_frame.insert("source_asset_code", frame.source_asset_code.encode(env));
            decoded_frame.insert("source_asset_scale", frame.source_asset_scale.encode(env));
        }
        Frame::ConnectionMaxData(ref frame) => {
            decoded_frame.insert("type", "connection_max_data".encode(env));
            decoded_frame.insert("max_offset", frame.max_offset.encode(env));
        }
        Frame::ConnectionDataBlocked(ref frame) => {
            decoded_frame.insert("type", "connection_data_blocked".encode(env));
            decoded_frame.insert("max_offset", frame.max_offset.encode(env));
        }
        Frame::ConnectionMaxStreamId(ref frame) => {
            decoded_frame.insert("type", "connection_max_stream_id".encode(env));
            decoded_frame.insert("max_offset", frame.max_stream_id.encode(env));
        }
        Frame::ConnectionStreamIdBlocked(ref frame) => {
            decoded_frame.insert("type", "connection_stream_id_blocked".encode(env));
            decoded_frame.insert("max_offset", frame.max_stream_id.encode(env));
        }
        Frame::StreamClose(ref frame) => {
            decoded_frame.insert("type", "stream_close".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));
            decoded_frame.insert("code", u8::from(frame.code).encode(env));
            decoded_frame.insert("message", frame.message.encode(env));
        }
        Frame::StreamMoney(ref frame) => {
            decoded_frame.insert("type", "stream_money".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));
            decoded_frame.insert("shares", frame.shares.encode(env));
        }
        Frame::StreamMaxMoney(ref frame) => {
            decoded_frame.insert("type", "stream_max_close".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));   
            decoded_frame.insert("receive_max", frame.receive_max.encode(env));   
            decoded_frame.insert("total_received", frame.total_received.encode(env));  
        }
        Frame::StreamMoneyBlocked(ref frame) => {
            decoded_frame.insert("type", "stream_money_blocked".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));   
            decoded_frame.insert("send_max", frame.send_max.encode(env));   
            decoded_frame.insert("total_sent", frame.total_sent.encode(env));   
        }
        Frame::StreamData(ref frame) => {
            decoded_frame.insert("type", "stream_data".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));   
            decoded_frame.insert("offset", frame.offset.encode(env));   
            decoded_frame.insert("data", frame.data.encode(env));   
        }
        Frame::StreamMaxData(ref frame) => {
            decoded_frame.insert("type", "stream_max_data".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));   
            decoded_frame.insert("max_offset", frame.max_offset.encode(env));   
        }
        Frame::StreamDataBlocked(ref frame) => {
            decoded_frame.insert("type", "stream_data_blocked".encode(env));
            decoded_frame.insert("stream_id", frame.stream_id.encode(env));   
            decoded_frame.insert("max_offset", frame.max_offset.encode(env));    
        }
        Frame::Unknown(ref _unknown_frame) => {
            decoded_frame.insert("type", "unknown_frame".encode(env));
            decoded_frame.insert("error", "could not decode as any Frame type".encode(env));
        }
    };

    decoded_frame
}
