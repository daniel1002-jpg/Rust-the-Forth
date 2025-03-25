pub enum StackWord {
    DUP,
    DROP,
    SWAP,
    OVER,
    ROT,
}

pub enum Define {
    Start,
    End,
    Word(String),
}
