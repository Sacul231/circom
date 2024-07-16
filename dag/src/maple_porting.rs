use super::{Tree, DAG};
use circom_algebra::algebra::Constraint;
use circom_algebra::num_bigint::BigInt;
use constraint_writers::debug_writer::DebugWriter;
use constraint_writers::json_writer::ConstraintJSON;
use std::collections::HashMap;

type C = Constraint<usize>;

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
        }else{
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

fn visit_tree(tree: &Tree, writer: &mut ConstraintJSON) -> Result<(), ()> {
    for constraint in &tree.constraints {
        let json_value = transform_constraint_to_string(&constraint);
        writer.write_constraint(&json_value.to_string())?;
    }
    for edge in Tree::get_edges(tree) {
        let subtree = Tree::go_to_subtree(tree, edge);
        visit_tree(&subtree, writer)?;
    }
    Result::Ok(())
}

pub fn port_constraints(dag: &DAG, debug: &DebugWriter) -> Result<(), ()> {
    let mut writer = debug.build_constraints_file()?;
    visit_tree(&Tree::new(dag), &mut writer)?;
    writer.end()
}
