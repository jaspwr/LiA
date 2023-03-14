use std::{ path::Path, error::Error, sync::Mutex };

use julia::api::Julia;

use crate::{ hierachy_construction::CompilerGlobals, cli::print_info };

struct JuliaSession {
    session: Julia,
}
unsafe impl Send for JuliaSession {}
unsafe impl Sync for JuliaSession {}

static JULIA_SESSION: Mutex<Option<JuliaSession>> = Mutex::new(None);

pub fn execute(code: String, compiler_globals: &mut CompilerGlobals) -> Result<String, String> {
    // TODO: Figure out a way to do this without global and work out
    //       all of the weird situations.
    let mut jl = JULIA_SESSION.lock().unwrap();
    if jl.is_none() {
        *jl = Some(JuliaSession::new(compiler_globals)?);
    }
    let mut jl = jl.as_mut().unwrap();

    let val = (match jl.session.eval_string(&code) {
        Ok(v) => { Ok(v) }
        Err(e) => { Err(format!("Julia error: {}", stringify_jl_err(e))) }
    })?;

    Ok(val.to_string())
}

impl JuliaSession {
    fn new(compiler_globals: &mut CompilerGlobals) -> Result<JuliaSession, String> {
        print_info("Initializing Julia session...".to_string());
        let mut jl = match Julia::new() {
            Ok(jl) => jl,
            Err(e) => {
                return Err(
                    format!("Failed to initialize Julia. Ensure you have Julia installed before using a `jl` statement. {}", e)
                );
            }
        };
        if let Err(e) = init_session(&mut jl, compiler_globals) {
            return Err(format!("Failed to initialize Julia session. {}", stringify_jl_err(e)));
        }
        Ok(JuliaSession { session: jl })
    }
}

fn init_session(
    jl: &mut Julia,
    compiler_globals: &mut CompilerGlobals
) -> Result<(), julia::error::Error> {
    let api: String = String::from_utf8_lossy(include_bytes!("../../../LiaAPI.jl")).to_string();
    jl.eval_string(api)?;

    let img_folder = Path::new(&compiler_globals.job.input_path);
    let img_folder = img_folder.parent().unwrap();

    jl.eval_string(
        format!(
            "doc_info = LiaDocInfo(true, \"{}\", \"{}\", \"{}\");",
            compiler_globals.job.input_path,
            compiler_globals.job.output_path,
            img_folder.to_str().unwrap()
        )
    )?;

    Ok(())
}

fn stringify_jl_err(e: julia::error::Error) -> String {
    match e {
        julia::error::Error::UnhandledException(e) => {
            return e.description().to_string();
        }
        _ => { format!("Unknown error: {}", e) }
    }
}