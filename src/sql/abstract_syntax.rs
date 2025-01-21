/// This module defines the structures that represent SQL statements
/// after parsing. These structures form the abstract syntax tree
/// that can be used by the query executor.


#[derive(Debug, PartialEq)]
pub enum Statement {
    CreateTable(CreateTableStatement),
    Insert(InsertStatement),
    Select(SelectStatement),
    Delete(DeleteStatement),
}


/// Represents a SQL `CREATE TABLE` statement.
///
/// # Fields
///
/// * `table_name` - The name of the table to be created.
/// * `columns` - A vector of column definitions specifying the columns of the table.
/// This variant is used to define a new table in the database with the specified schema.
///
/// # Example
///
/// ```sql
/// CREATE TABLE table_name (
///     column1 datatype,
///     column2 datatype,
///     ...
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug, PartialEq)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Integer,
    Text,
    Boolean,
}

#[derive(Debug, PartialEq)]
pub struct InsertStatement {
    pub table_name: String,
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub table_name: String,
    pub columns: Vec<String>,
    pub condition: Option<Condition>,
}

#[derive(Debug, PartialEq)]
pub struct DeleteStatement {
    pub table_name: String,
    pub condition: Option<Condition>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Text(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub left: String,
    pub operator: Operator,
    pub right: Value,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
}
