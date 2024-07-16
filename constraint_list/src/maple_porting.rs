use super::{ConstraintStorage, C};
use crate::SignalMap;
use circom_algebra::num_bigint::BigInt;
use constraint_writers::debug_writer::DebugWriter;
use std::collections::HashMap;


pub fn transform_constraint_to_string(constraint: &C) -> String {
    let mut result;
    if !constraint.a().is_empty() || !constraint.b().is_empty(){
        result = format!("({}) * ({})", 
            hashmap_as_string(constraint.a()),
            hashmap_as_string(constraint.b()),
        );
        if !constraint.c().is_empty(){
            result = format!("{} + {}", 
                result,
                hashmap_as_string(constraint.c()),
            );
        }
    } else {
        result = hashmap_as_string(constraint.c());
    }
    result
}

fn hashmap_as_string(values: &HashMap<usize, BigInt>) -> String {
    let mut order: Vec<&usize> = values.keys().collect();
    order.sort();
    let mut result = String::new();
    let mut is_first = true;
    for i in order {
        let (key, value) = values.get_key_value(i).unwrap();
        let value = value.to_str_radix(10);
        if !is_first{
            result = format!("{} + ", result);
        } else{
            is_first = false;
        }
        if *key == 0{
            result = format!("{} {}", result, value);
        } else{
            result = format!("{} {} * s_{}", result, value, key);
        }
    }
    result
}

pub fn port_constraints(
    storage: &ConstraintStorage,
    map: &SignalMap,
    debug: &DebugWriter,
) -> Result<(), ()> {
    let mut writer = debug.build_maple_constraints_file()?;
    for c_id in storage.get_ids() {
        let constraint = storage.read_constraint(c_id).unwrap();
        let constraint = C::apply_correspondence(&constraint, map);
        let string_value = transform_constraint_to_string(&constraint);
        writer.write_constraint(&string_value)?;
    }
    writer.end()
}
