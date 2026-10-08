pub trait GameObject: Send + Sync {
    fn x(&self) -> u64;
    fn y(&self) -> u64;
}

pub struct SnakeTail {
    pub x: u64,
    pub y: u64,
    pub next: Option<Box<SnakeTail>>,
}

impl GameObject for SnakeTail {
    fn x(&self) -> u64 {
        self.x
    }
    fn y(&self) -> u64 {
        self.y
    }
}

pub struct SnakeHead {
    pub x: u64,
    pub y: u64,
    pub tail: SnakeTail,
}

impl GameObject for SnakeHead {
    fn x(&self) -> u64 {
        self.x
    }

    fn y(&self) -> u64 {
        self.y
    }
}

pub struct Food {
    pub x: u64,
    pub y: u64,
}

impl GameObject for Food {
    fn x(&self) -> u64 {
        self.x
    }

    fn y(&self) -> u64 {
        self.y
    }
}

pub struct Wall {
    pub x: u64,
    pub y: u64,
}

impl GameObject for Wall {
    fn x(&self) -> u64 {
        self.x
    }

    fn y(&self) -> u64 {
        self.y
    }
}
