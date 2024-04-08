use std::collections::HashMap;
use crate::class::Class;
use crate::club::Club;
use crate::course::Course;
use crate::student::{Gender, Student};

/// 请基于Rust的基本数据结构写一个简单的学生管理系统
/// （比如，学生，社团，班级、课程等）
/// 明确类型之间的关系，进行基本的CRUD操作。
mod student;
mod club;
mod class;
mod course;

fn main() {
    println!("Hello, world!");

    // 创建学生
    let student_bob = Student{
        id:1,
        name: "bob".to_string(),
        age: 12,
        gender: Gender::BOY
    };
    let student_jack = Student{
        id:2,
        name: "jack".to_string(),
        age: 13,
        gender: Gender::BOY
    };
    let student_alice = Student{
        id:3,
        name: "alice".to_string(),
        age: 11,
        gender: Gender::GIRL
    };
    let mut student_map:HashMap<u32, Student>=HashMap::new();
    student_map.insert(student_bob.id, student_bob);
    student_map.insert(student_jack.id, student_jack);
    student_map.insert(student_alice.id, student_alice);

    let student_bob = student_map.get(&1).unwrap();
    let student_jack = student_map.get(&2).unwrap();
    let student_alice = student_map.get(&3).unwrap();

    println!("所有学生信息：");
    println!("{:?}", student_map);
    println!();
    println!();

    // 创建班级
    let mut class_1 = Class{
        id: 1,
        name: "Class_01".to_string(),
        students: HashMap::new(),
    };
    let mut class_2 = Class{
        id: 2,
        name: "Class_02".to_string(),
        students: HashMap::new(),
    };
    class_1.add(student_bob);
    class_1.add(student_jack);
    class_2.add(student_alice);

    println!("所有班级信息：");
    println!("班级1：{:?}", class_1);
    println!("班级2：{:?}", class_2);
    println!();
    println!();

    // 创建社团
    let mut club_1 = Club {
        id: 1,
        name: "Baseball Club".to_string(),
        students: HashMap::new(),
    };
    let mut club_2 = Club {
        id: 1,
        name: "Poker Club".to_string(),
        students: HashMap::new(),
    };
    club_1.add(student_bob);
    club_2.add(student_jack);
    club_2.add(student_alice);

    club_2.remove(student_alice);
    println!("所有社团信息：");
    println!("社团1：{:?}", class_1);
    println!("社团2：{:?}", class_2);
    println!();
    println!();

    // 创建课程
    let mut course_1 = Course{
        id: 1,
        name: "Math".to_string(),
        students: HashMap::new(),
    };
    let mut course_2 = Course{
        id: 1,
        name: "Science".to_string(),
        students: HashMap::new(),
    };
    let mut course_3 = Course{
        id: 1,
        name: "English".to_string(),
        students: HashMap::new(),
    };
    course_1.add(student_bob);
    course_1.add(student_jack);
    course_1.add(student_alice);
    course_2.add(student_bob);
    course_2.add(student_jack);
    course_2.add(student_alice);
    course_3.add(student_bob);
    course_3.add(student_jack);
    course_3.add(student_alice);
    println!("所有课程信息：");
    println!("课程1：{:?}", course_1);
    println!("课程2：{:?}", course_2);
    println!("课程3：{:?}", course_3);
    println!();
    println!();
}
