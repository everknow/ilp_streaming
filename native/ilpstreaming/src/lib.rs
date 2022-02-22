// imports
use interledger::packet::Address;
use interledger::stream::packet::ErrorCode;
use rustler::{Encoder, Env, Error, NifResult, Term};
use std::boxed::Box;
use std::collections::HashMap;
use std::str::FromStr;
use interledger::stream::packet::{ConnectionCloseFrame, ConnectionNewAddressFrame, ConnectionAssetDetailsFrame,ConnectionMaxDataFrame,ConnectionDataBlockedFrame, 
    ConnectionMaxStreamIdFrame, ConnectionStreamIdBlockedFrame, StreamCloseFrame,StreamMoneyFrame, StreamMoneyBlockedFrame,StreamDataFrame,StreamMaxMoneyFrame, StreamMaxDataFrame,
    StreamDataBlockedFrame};


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
    let t = m.get("type").ok_or(error!("unknow type"))?;
    match t.decode::<&str>().or(err!("type not binary"))? {
        "connection_close_frame" => {
            // get fields
            let c = m.get("code").ok_or(error!("code is missing"))?;
            let m = m
                .get("from_username")
                .ok_or(error!("from_username is missing"))?;

            // transform
            let codes = c.decode::<u8>().or(err!("could not decode to_username"))?; //Type ErrorCode
            let message = m.decode::<&str>().or(err!("could not decode from_username"))?;

            //special transform 
            let code = ErrorCode::from(codes);



            let pn = ConnectionCloseFrame { code, message };
            Ok("No idea to return".encode(env)) //to fix, WIP
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

            let pn = ConnectionNewAddressFrame { source_account };

            Ok("No idea to return ".encode(env)) //to fix, WIP
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

            let pn = ConnectionAssetDetailsFrame {
                source_asset_code,
                source_asset_scale,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "connection_max_data_frame" => {
            // get fields
            let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
            // transform
            let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;
            let pn = ConnectionMaxDataFrame { max_offset };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "connection_data_blocked_frame" => {
            // get fields
            let mob = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
            // transform
            let max_offset = mob
                .decode::<u64>()
                .or(err!("could not decode max_offset"))?;
            let pn = ConnectionDataBlockedFrame { max_offset };

            Ok("No idea to return ".encode(env)) //to fix, WIP
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
            let pn = ConnectionMaxStreamIdFrame { max_stream_id };

            Ok("No idea to return ".encode(env)) //to fix, WIP
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
            let pn = ConnectionStreamIdBlockedFrame { max_stream_id };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "stream_close_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let c = m.get("code").ok_or(error!("code is missing"))?;
            let m = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let codes = si.decode::<u8>().or(err!("could not decode code"))?; 
            let message = si.decode::<&str>().or(err!("could not decode message"))?;

            //special transform
            let code = ErrorCode::from(codes);

            let pn = StreamCloseFrame {
                stream_id,
                code,
                message,
            };
            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "stream_money_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let s = m.get("shares").ok_or(error!("shares is missing"))?;
            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let shares = s.decode::<u64>().or(err!("could not decode shares"))?; //

            let pn = StreamMoneyFrame { stream_id, shares };
            Ok("No idea to return ".encode(env)) //to fix, WIP
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

            let pn = StreamMaxMoneyFrame {
                stream_id,
                receive_max,
                total_received,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
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

            let pn = StreamMoneyBlockedFrame {
                stream_id,
                send_max,
                total_sent,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "stream_data_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let o = m.get("offset").ok_or(error!("offset is missing"))?;
            let d = m.get("data").ok_or(error!("data is total_sen"))?;

            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let offset = o.decode::<u64>().or(err!("could not decode offset"))?;
            let data = d.into_binary().or(err!("could not decode data"))?.as_slice();

            //special transform
            //let data =  <u8>::try_from(datas).or(err!("could not decode data"))?;

            let pn = StreamDataFrame {
                stream_id,
                offset,
                data,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        "stream_max_data_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;

            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;

            let pn = StreamMaxDataFrame {
                stream_id,
                max_offset,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
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

            let pn = StreamDataBlockedFrame {
                stream_id,
                max_offset,
            };

            Ok("No idea to return ".encode(env)) //to fix, WIP
        }
        _ => {
            err!("type not recognised")
        }
    }
    

}





// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_close_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connection_close_frame data to map<String,Term>"
//     ))?;

//     // get fields
//     let c = m.get("code").ok_or(error!("code is missing"))?;
//     let m = m
//         .get("from_username")
//         .ok_or(error!("from_username is missing"))?;

//     // transform
//     let code = c
//         .decode::<ErrorCode>()
//         .or(err!("could not decode to_username"))?; //Type ErrorCode
//     let message = m
//         .decode::<&str>()
//         .or(err!("could not decode from_username"))?;

//     let pn = ConnectionCloseFrame { code, message };
//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_new_address<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connection_new_address data to map<String,Term>"
//     ))?;

//     // get fields
//     let sa = m
//         .get("source_account")
//         .ok_or(error!("source_account is missing"))?;

//     // transform
//     let source_accounts = sa
//         .decode::<&str>()
//         .or(err!("could not decode source_account"))?;

//     //special decode
//     let source_account =
//         Address::from_str(source_accounts).or(err!("could not decode the source_account"))?;

//     let pn = ConnectionNewAddressFrame { source_account };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_asset_details_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_cconnection_asset_details_frame data to map<String,Term>"
//     ))?;

//     // get fields
//     let sac = m
//         .get("source_asset_code")
//         .ok_or(error!("source_asset_code is missing"))?;
//     let sas = m
//         .get("source_asset_scale")
//         .ok_or(error!("source_asset_scale is missing"))?;

//     // transform
//     let source_asset_code = sac
//         .decode::<&str>()
//         .or(err!("could not decode source_asset_code"))?;
//     let source_asset_scale = sas
//         .decode::<u8>()
//         .or(err!("could not decode source_asset_code"))?;

//     let pn = ConnectionAssetDetailsFrame {
//         source_asset_code,
//         source_asset_scale,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_max_data_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connection_max_data_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
//     // transform
//     let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;
//     let pn = ConnectionMaxDataFrame { max_offset };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_data_blocked_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connection_data_blocked_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let mob = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
//     // transform
//     let max_offset = mob
//         .decode::<u64>()
//         .or(err!("could not decode max_offset"))?;
//     let pn = ConnectionDataBlockedFrame { max_offset };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connection_connection_max_stream_id_frame<'a>(
//     env: Env<'a>,
//     arg: Term,
// ) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connection_connection_max_stream_id_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let msi = m
//         .get("max_stream_id")
//         .ok_or(error!("max_stream_id is missing"))?;
//     // transform
//     let max_stream_id = msi
//         .decode::<u64>()
//         .or(err!("could not decode max_stream_id"))?;
//     let pn = ConnectionMaxStreamIdFrame { max_stream_id };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_connections_stream_id_blocked_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_connections_stream_id_blocked_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let msis = m
//         .get("max_stream_id")
//         .ok_or(error!("max_stream_id is missing"))?;
//     // transform
//     let max_stream_id = msis
//         .decode::<u64>()
//         .or(err!("could not decode max_stream_id"))?;
//     let pn = ConnectionStreamIdBlockedFrame { max_stream_id };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_close_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_close_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let c = m.get("code").ok_or(error!("code is missing"))?;
//     let m = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let code = si.decode::<u64>().or(err!("could not decode code"))?; // Type ErrorCode
//     let message = si.decode::<&str>().or(err!("could not decode message"))?;

//     let pn = StreamCloseFrame {
//         stream_id,
//         code,
//         message,
//     };
//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }
//#[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_money_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_money_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let s = m.get("shares").ok_or(error!("shares is missing"))?;
//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let shares = s.decode::<u64>().or(err!("could not decode shares"))?; //

//     let pn = StreamMoneyFrame { stream_id, shares };
//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_max_money_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_max_money_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let rm = m
//         .get("receive_max")
//         .ok_or(error!("receive_max is missing"))?;
//     let tr = m
//         .get("total_received")
//         .ok_or(error!("total_received is missing"))?;

//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let receive_max = rm
//         .decode::<u64>()
//         .or(err!("could not decode receive_max"))?;
//     let total_received = tr
//         .decode::<u64>()
//         .or(err!("could not decode total_received"))?;

//     let pn = StreamMaxMoneyFrame {
//         stream_id,
//         receive_max,
//         total_received,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_money_blocked_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the tream_money_blocked_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let sm = m.get("send_max").ok_or(error!("send_max is missing"))?;
//     let ts = m
//         .get("total_sent")
//         .ok_or(error!("total_received is total_sen"))?;

//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let send_max = sm.decode::<u64>().or(err!("could not decode send_max"))?;
//     let total_sent = ts.decode::<u64>().or(err!("could not decode total_sent"))?;

//     let pn = StreamMoneyBlockedFrame {
//         stream_id,
//         send_max,
//         total_sent,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_data_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode the stream_data_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let o = m.get("offset").ok_or(error!("offset is missing"))?;
//     let d = m.get("data").ok_or(error!("data is total_sen"))?;

//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let offset = o.decode::<u64>().or(err!("could not decode offset"))?;
//     let data = d.decode::<u64>().or(err!("could not decode data"))?;

//     let pn = StreamDataFrame {
//         stream_id,
//         offset,
//         data,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_max_data_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode stream_max_data_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;

//     // transform
//     let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;

//     let pn = StreamMaxDataFrame {
//         stream_id,
//         max_offset,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// fn encode_stream_data_blocked_frame<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
//     let m = arg.decode::<HashMap<String, Term>>().or(err!(
//         "could not decode stream_data_blocked_frame to map<String,Term>"
//     ))?;

//     // get fields
//     let sis = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
//     let mos = m.get("max_offset").ok_or(error!("max_offset is missing"))?;

//     // transform
//     let stream_id = sis.decode::<u64>().or(err!("could not decode stream_id"))?;
//     let max_offset = mos
//         .decode::<u64>()
//         .or(err!("could not decode max_offset"))?;

//     let pn = StreamDataBlockedFrame {
//         stream_id,
//         max_offset,
//     };

//     Ok("No idea to return ".encode(env)) //to fix, WIP
// }

//There is no Stream Receipt 0x17 { not implemented in Interleger-rs }
