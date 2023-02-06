use crate::sigma;
use anyhow;
use format_serde_error::SerdeError;
use serde::{de::Error, Deserialize, Serialize};
use std;

fn parse(rule: &str) -> Result<sigma::SigmaRule, anyhow::Error> {
    let sigma: sigma::SigmaRule = serde_yaml::from_str(rule)?;
    Ok(sigma)
}

#[cfg(test)]
mod tests {
    use super::parse;

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

        let result = parse(rule);
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("erro parsing");
            }
        }
    }
}
