#[derive(Default, Serialize, Queryable)]
pub struct Widget {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
}
