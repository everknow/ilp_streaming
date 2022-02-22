// imports
use interledger::packet::oer::MutBufOerExt;
use interledger::stream::packet::SerializableFrame;
use interledger::packet::Address;
use interledger::stream::packet::ErrorCode;
use interledger::stream::packet::{
    ConnectionAssetDetailsFrame, ConnectionCloseFrame, ConnectionDataBlockedFrame,
    ConnectionMaxDataFrame, ConnectionMaxStreamIdFrame, ConnectionNewAddressFrame,
    ConnectionStreamIdBlockedFrame, StreamCloseFrame, StreamDataBlockedFrame, StreamDataFrame,
    StreamMaxDataFrame, StreamMaxMoneyFrame, StreamMoneyBlockedFrame, StreamMoneyFrame,
};
use rustler::{Encoder, Env, Error, NifResult, Term};
use std::boxed::Box;
use std::collections::HashMap;
use std::str::FromStr;
use bytes::{ BufMut};
use interledger::stream::packet::{ FrameType};

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
    let m = arg.decode::<HashMap<String, Term>>() .or(err!("could not decode arg to map<String,Term>"))?;
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
            let message = m
                .decode::<&str>()
                .or(err!("could not decode from_username"))?;

            //special transform
            let code = ErrorCode::from(codes);

            let frame = ConnectionCloseFrame { code, message };
            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionClose as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = ConnectionNewAddressFrame { source_account };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionNewAddress as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = ConnectionAssetDetailsFrame {
                source_asset_code,
                source_asset_scale,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionAssetDetails as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
        }
        "connection_max_data_frame" => {
            // get fields
            let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
            // transform
            let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;
            let frame = ConnectionMaxDataFrame { max_offset };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionMaxData as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env))         }
        "connection_data_blocked_frame" => {
            // get fields
            let mob = m.get("max_offset").ok_or(error!("max_offset is missing"))?;
            // transform
            let max_offset = mob
                .decode::<u64>()
                .or(err!("could not decode max_offset"))?;
            let frame = ConnectionDataBlockedFrame { max_offset };
            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionDataBlocked as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
            
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
            let frame = ConnectionMaxStreamIdFrame { max_stream_id };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionMaxStreamId as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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
            let frame = ConnectionStreamIdBlockedFrame { max_stream_id };
            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::ConnectionStreamIdBlocked as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = StreamCloseFrame {
                stream_id,
                code,
                message,
            };
            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamClose as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
        }
        "stream_money_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let s = m.get("shares").ok_or(error!("shares is missing"))?;
            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let shares = s.decode::<u64>().or(err!("could not decode shares"))?; //

            let frame = StreamMoneyFrame { stream_id, shares };
            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamMoney as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = StreamMaxMoneyFrame {
                stream_id,
                receive_max,
                total_received,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamMaxMoney as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = StreamMoneyBlockedFrame {
                stream_id,
                send_max,
                total_sent,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamMoneyBlocked as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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
            let frame = StreamDataFrame {
                stream_id,
                offset,
                data,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamData as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
        }
        "stream_max_data_frame" => {
            // get fields
            let si = m.get("stream_id").ok_or(error!("stream_id is missing"))?;
            let mo = m.get("max_offset").ok_or(error!("max_offset is missing"))?;

            // transform
            let stream_id = si.decode::<u64>().or(err!("could not decode stream_id"))?;
            let max_offset = mo.decode::<u64>().or(err!("could not decode max_offset"))?;

            let frame = StreamMaxDataFrame {
                stream_id,
                max_offset,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamMaxData as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
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

            let frame = StreamDataBlockedFrame {
                stream_id,
                max_offset,
            };

            let mut result = <Vec<u8>>::new();
            let mut contents = Vec::new();
            
            result.put_u8(FrameType::StreamDataBlocked as u8);
            frame.put_contents(&mut contents);
            result.put_var_octet_string(&*contents);

            Ok(result.encode(env)) 
        }
        _ => {
            err!("type not recognised")
        }
    }

}


// #[rustler::nif(schedule = "DirtyCpu")]
// fn decode<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {

//     match t.decode::<&str>().or(err!("type not binary"))? {
//         "connection_close_frame" => {  
              
//             // Frame::ConnectionClose(ref frame) => {
//             //     buffer_unencrypted.put_u8(FrameType::ConnectionClose as u8);
//             //     frame.put_contents(&mut contents);
//             // }        
        
           
            
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
   
