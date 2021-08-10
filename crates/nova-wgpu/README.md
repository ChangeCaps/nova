# Nova Wgpu
This crate serves as a layer of indirection, which allows us to cut down about 100 
dependencies making compile times of game libs much faster. This means using a bunch of dynamic dispatch
which isn't great for performance but in the end it's believed to be worth it.