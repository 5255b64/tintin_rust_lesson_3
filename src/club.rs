use std::collections::HashMap;
use crate::student::Student;

/// 社团
#[derive(Debug)]
pub struct Club<'a>{
    pub id:u32,
    pub name:String,
    pub students:HashMap<u32, &'a Student>
}

impl <'a> Club <'a> {
    pub fn add(&mut self, stu:&'a Student) {
        self.students.insert(stu.id, stu);
    }

    pub fn remove(&mut self, stu:&'a Student) {
        self.students.remove(&stu.id);
    }
}