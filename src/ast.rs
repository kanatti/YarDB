pub enum Statement<'a> {
    Insert(InsertStatement<'a>),
    Select(SelectStatement),
    Stats(StatsStatement),
}

#[derive(Debug)]
pub struct InsertStatement<'a> {
    pub id: i32,
    pub name: &'a str,
    pub email: &'a str,
}

pub struct SelectStatement {}

pub struct StatsStatement {}
