extern crate chrono;
use crate::model::stream::BetfairMessage;
use crate::model::stream::Mc;
use chrono::prelude::*;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoMessage {
    #[serde(rename = "_id")]
    pub id: Id,
    pub payload: BetfairMessage,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Default)]
pub struct PriceCache {
    msg_count: i64,
    cache: HashMap<String, Mc>,
}

impl PriceCache {
    pub fn apply(&mut self, msg: BetfairMessage) -> Result<(), Box<dyn Error>> {
        self.msg_count += 1;
        match msg.op.as_str() {
            "connection" => println!("Connect Msg"),
            "status" => println!("Status Msg"),
            "mcm" => {
                if let Some(publish_time) = msg.pt {
                    let dt = Utc.timestamp(
                        publish_time / 1000,
                        ((publish_time % 1000) * 1000000) as u32,
                    );
                    println!("mcm.pt={}", dt.to_rfc3339());
                }
                if let Some(mc) = msg.mc {
                    for m in mc.into_iter() {
                        let mkt_id = m.id.to_owned();
                        if let Some(ref change_type) = msg.ct {
                            if change_type == "SUB_IMAGE" {
                                match self.cache.entry(mkt_id.clone()) {
                                    Occupied(mut o) => {
                                        o.insert(m.clone());
                                    }
                                    Vacant(e) => {
                                        e.insert(m.clone());
                                    }
                                }
                            }
                        } else {
                            if let Some(rc) = m.clone().rc {
                                match self.cache.entry(mkt_id.clone()) {
                                    Occupied(mut o) => {
                                        let entry = o.get_mut();
                                        for r in rc.into_iter() {
                                            println!("RunnerId={:?}", r.id);
                                            if let Some(ref available_to_back) = r.atb {
                                                println!("ATB={:?}", available_to_back);
                                                for atb_change in r.atb {}
                                            }
                                            if let Some(ref available_to_lay) = r.atl {
                                                println!("ATL={:?}", available_to_lay);
                                            }
                                        }
                                    }
                                    Vacant(_e) => {
                                        return Err(
                                            "Trying to update Market which does not exist in Cache"
                                                .into(),
                                        )
                                    }
                                }
                            }
                        }
                        if let Some(md) = m.clone().market_definition {
                            println!("{:?}", md.event_id);
                            println!("{:?}", md.status);
                            println!("{:?}", md.number_of_active_runners);
                            for r in md.runners.into_iter() {
                                println!("{:?}", r.id);
                                println!("{:?}", r.status);
                            }
                        }
                    }
                }
            }
            _ => panic!("{}", self.msg_count),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PriceCache;
    use crate::model::mongo::MongoMessage;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn it_works() {
        let mut pcache: PriceCache = Default::default();
        let lines = read_lines("../dumps/1.183738831").unwrap();
        let mut msg_count = 0;
        for line in lines {
            msg_count += 1;
            if let Ok(l) = line {
                // println!("{}", l);
                match serde_json::from_str::<MongoMessage>(&l) {
                    Ok(m) => {
                        if let Ok(_res) = pcache.apply(m.payload) {
                        } else {
                            panic!("Error During Price Cache Event Apply")
                        }
                    }
                    Err(e) => {
                        println!("{} - {}", msg_count, e);
                        panic!();
                    }
                };
            }
        }
        println!("{:?}", pcache.cache);
    }
}
