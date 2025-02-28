#[used]
#[link_section = ".init_array"]
static INIT: extern "C" fn() = {
    extern "C" fn init() {
        println!("Hello world!");
    }
    init
};

#[used]
#[link_section = ".fini_array"]
static FINI: extern "C" fn() = {
    extern "C" fn fini() {
        println!("Goodbye world!");
    }
    fini
};
