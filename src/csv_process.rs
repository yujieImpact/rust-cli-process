use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, string::String};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,

    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(inputfile: &str, outputfile: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(inputfile)?;
    let mut ret = Vec::with_capacity(128);     //for result

                                                            //loop
    for item in reader.deserialize::<Player>() {
        let temp: Player = item?;
        ret.push(temp);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    let _ = fs::write(outputfile, json);
    Ok(())
}
