

/// 学生
#[derive(Debug)]
pub struct Student {
    pub id:u32,
    pub name:String,
    pub age:u8,
    pub gender:Gender
}

#[derive(Debug)]
pub enum Gender{
    BOY,
    GIRL
}