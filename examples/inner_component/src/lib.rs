use comet::prelude::*;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

component! { Counter,
    button @click: { self.value += 1 } {
        {{ self.value }}
    }
}

pub struct App {
    counter: Shared<Counter>,
    counter2: Shared<Counter>,
}

impl App {
    fn new() -> Self {
        Self {
            counter: Counter::default().into(),
            counter2: Counter::default().into(),
        }
    }
}

component! { App,
    div {
        @{self.counter}
        @{self.counter2}
        button
            @click: {self.counter.borrow_mut().value += 42} {
            {{ "counter1" }}
        }
        button
            @click: {self.counter2.borrow_mut().value += 42} {
            {{ "counter2" }}
        }
    }
}

comet!(App::new());
