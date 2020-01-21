pub struct BinaryChunk {
    header: Header,
    size_upvalues: u8,
    main_func: Arc<Prototype>,
}

pub struct Header {
    // 0x1B4C7561 Esc,L,u,a
    signature: [u8; 4],
    // v5.3.5 = 0x53
    version: u8,
    // 0
    format: u8,
    // 0x1993, 0x0d, 0x0a, 0x1a, 0x0a
    luac_data: [u8; 6],
    cint_size: u8,
    sizet_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    luac_int: i64,
    luac_num: f64,
}

pub struct Prototype {
    source: String,
    line_defined: u32,
    last_line_defined: u32,
    num_params: u8,
    is_vararg: u8,
    max_stack_size: u8,
    code: Vec<u8>,
    constants: Vec<Object>,
    upvalues: Vec<Upvalues>,
    protos: Vec<Arc<Prototype>>,
    line_info: Vec<u32>,
    loc_vars: Vec<LocVar>,
    upvalue_names: Vec<String>,
}

pub struct Object {}

pub struct Upvalues {}

pub struct LocVar {}