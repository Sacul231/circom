use std::fs::File;
use std::io::{BufWriter, Write};

pub struct ConstraintMaple {
    writer_constraints: BufWriter<File>,
}

impl ConstraintMaple {
    pub fn new(file: &str) -> Result<ConstraintMaple, ()> {
        let file_constraints = File::create(file).map_err(|_err| {})?;
        let writer_constraints = BufWriter::new(file_constraints);

        Result::Ok(ConstraintMaple { writer_constraints })
    }
    pub fn write_constraint(&mut self, constraint: &str) -> Result<(), ()> {
        self.writer_constraints.write_all(b"Poly: ").map_err(|_err| {})?;
        self.writer_constraints.write_all(constraint.as_bytes()).map_err(|_err| {})?;
        self.writer_constraints.flush().map_err(|_err| {})?;
        self.writer_constraints.write_all(b"\n").map_err(|_err| {})?;
        self.writer_constraints.flush().map_err(|_err| {})?;
        Result::Ok(())
    }
    pub fn end(mut self) -> Result<(), ()> {
        self.writer_constraints.write_all(b"End:\n").map_err(|_err| {})?;
        self.writer_constraints.flush().map_err(|_err| {})?;
        Result::Ok(())
    }
}

