use comet::prelude::*;

component! {
    i32,
    button
        [height: {20 + self * 10}]
        @click: { *self += 1 } {
        {{ self }}
    }
}

comet!(0);
