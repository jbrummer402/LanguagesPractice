pub struct Node {
    value: Option<T>,
    activation_fn: fn(i32, i32) -> i32,
    parent: &Node,
    children: Vec<&Node>,
}

impl Node {
    pub fn default() -> Result<Self> {
        

        Ok(Self {
            value: Some(i32),
            activation_fn: fn(i32, i32) -> i32,
        })
    }
}