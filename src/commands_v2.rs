use parser;
use redisql_lib::redis as r;

#[allow(non_snake_case)]
pub extern "C" fn CreateDB_v2(
    ctx: *mut r::rm::ffi::RedisModuleCtx,
    argv: *mut *mut r::rm::ffi::RedisModuleString,
    argc: ::std::os::raw::c_int,
) -> i32 {
    let context = r::rm::Context::new(ctx);
    let argvector = match r::create_argument(argv, argc) {
        Ok(argvector) => argvector,
        Err(mut error) => {
            return error.reply(&context);
        }
    };
    let command = match parser::CreateDB::parse(argvector) {
        Ok(comm) => comm,
        Err(e) => return e.reply(e),
    }
    0
}