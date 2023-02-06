use crate::sigma;
use anyhow;
use format_serde_error::SerdeError;
use serde::{de::Error, Deserialize, Serialize};
use std;

pub fn parse_sigma(rule: &str) -> Result<sigma::SigmaRule, anyhow::Error> {
    let sigma: sigma::SigmaRule = serde_yaml::from_str(rule)?;
    Ok(sigma)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rule = "
        title: testrule
        description: Hunt honey
        references:
              - https://example.com/
        author: John Doe
        date: 2023/02/06
        logsource:
            category: process_creation
            product: windows
        level: high
        ";

        let result = parse_sigma(rule);
        match result {
            Ok(d) => {
                println!("{:?}", d.header.level.unwrap());
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
