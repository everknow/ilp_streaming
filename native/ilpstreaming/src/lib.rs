// imports
use bytes::{Bytes, BytesMut};
use interledger::errors::{AccountStoreError, AddressStoreError, ExchangeRateStoreError};
use interledger::packet::Address;
use interledger::rates::ExchangeRateStore;
use interledger::router::RouterStore;
use interledger::service::{Account, AccountStore, AddressStore, Username};
use interledger::service_util::MaxPacketAmountAccount;
use interledger::stream::{PaymentNotification,ConnectionGenerator,StreamReceiverService}; //waypoint
use rustler::types::binary::{Binary, OwnedBinary};
use rustler::{Encoder, Env, Error, NifResult, Term};
use std::boxed::Box;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

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
fn encode_payment_notification<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let m = arg.decode::<HashMap<String, Term>>().or(err!("could not decode the stream data to map<String,Term>"))?;

            // get fields
            let tu = m.get("to_username").ok_or(error!("to_username value is missing"))?;
            let fu = m.get("from_username").ok_or(error!("from_username is missing"))?;
            let d = m.get("destination").ok_or(error!("destination is missing"))?;
            let a = m.get("amount").ok_or(error!("amount is missing"))?;
            let t = m.get("timestamp").ok_or(error!("timestamp is missing"))?;
            let s = m.get("sequence").ok_or(error!("sequence is missing"))?;
            let cc = m.get("connection_closed").ok_or(error!("connection_closed is missing"))?;

            // transform
            let to_usernames = tu.decode::<&str>().or(err!("could not decode to_username"))?;
            let from_usernames = fu.decode::<&str>().or(err!("could not decode from_username"))?;
            let destinations = d.decode::<&str>().or(err!("could not decode the destination"))?;
            let amount = a.decode::<u64>().or(err!("could not decode the amount"))?;
            let timestamp = t.decode::<String>().or(err!("could not decode the timestamp"))?;
            let sequence = s.decode::<u64>().or(err!("could not decode the sequence"))?;
            let connection_closed = cc.decode::<bool>().or(err!("could not decode connection_closed value "))?;

            //special decode
            let destination = Address::from_str(destinations).or(err!("could not decode the destination"))?;
            let to_username = Username::from_str(to_usernames).or(err!("could not decode to_username"))?;
            let from_username = Username::from_str(from_usernames).or(err!("could not decode from_username"))?;
           

            let pn = PaymentNotification {
                to_username,
                from_username,
                destination,
                amount,
                timestamp,
                sequence,
                connection_closed,
            };
            Ok("No idea to return ".encode(env))  //to fix, WIP
        }


 