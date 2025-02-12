// Copyright 2019-present, OVH SAS.
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use std::collections::BTreeMap;

use serde_value::Value;

pub struct FlatSerializer {
    key_separator: String,
    prefix: String,
}

impl FlatSerializer {
    pub fn new(key_separator: String, prefix: String) -> FlatSerializer {
        FlatSerializer {
            key_separator,
            prefix,
        }
    }

    fn format_key(&self, xpath: &str, key: &str, _value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("{}{k}", self.prefix),
            (x, k) => format!("{x}{}{k}", self.key_separator),
        }
    }

    pub fn disassemble(&self, xpath: &str, key: &str, value: &Value) -> BTreeMap<Value, Value> {
        let mut parts = BTreeMap::new();
        match value {
            Value::Map(ref tree) => {
                for (k, v) in tree.iter() {
                    let subkey = match k {
                        Value::String(data) => data.to_string(),
                        Value::Char(data) => data.to_string(),
                        _ => panic!("Map keys MUST be strings or char"),
                    };
                    parts.append(&mut self.disassemble(
                        &self.format_key(xpath, key, value),
                        &subkey,
                        v,
                    ));
                }
            }
            Value::Seq(ref values) => {
                for (i, val) in values.iter().enumerate() {
                    parts.append(&mut self.disassemble(
                        &self.format_key(xpath, key, value),
                        &format!("{i}"),
                        val,
                    ));
                }
            }
            _ => {
                parts.insert(
                    Value::String(self.format_key(xpath, key, value)),
                    value.clone(),
                );
            }
        };
        parts
    }
}
