use super::json_writer::ConstraintJSON;
use super::maple_writer::ConstraintMaple;


#[derive(Clone)]
pub struct DebugWriter {
    pub json_constraints: String,
    pub maple_constraints: String,

}
impl DebugWriter {
    pub fn new(c: String, c1: String) -> Result<DebugWriter, ()> {
        Result::Ok(DebugWriter { json_constraints: c, maple_constraints: c1 })
    }

    pub fn build_constraints_file(&self) -> Result<ConstraintJSON, ()> {
        ConstraintJSON::new(&self.json_constraints)
    }

    pub fn build_maple_constraints_file(&self) -> Result<ConstraintMaple, ()> {
        ConstraintMaple::new(&self.maple_constraints)
    }
}
