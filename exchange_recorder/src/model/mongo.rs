use crate::model::stream::BetfairMessage;

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

#[cfg(test)]
mod tests {
    use crate::model::mongo::MongoMessage;
    use std::env;
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
        let lines = read_lines("../dumps/1.183204772").unwrap();
        for line in lines {
            if let Ok(l) = line {
                // println!("{}", l);
                match serde_json::from_str::<MongoMessage>(&l) {
                    Ok(m) => {
                        println!("Deserialized");
                        println!("{:?}", m);
                        // println!("{}", m.payload);
                    }
                    Err(e) => {
                        println!("{}", e);
                        panic!();
                    }
                };
            }
        }
    }
}
