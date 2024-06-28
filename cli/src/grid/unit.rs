pub enum UnitType {
    Row,
    Col,
    MiniGrid,
}

pub enum Unit {
    Row(u32),
    Col(u32),
    MiniGrid(u32),
}
