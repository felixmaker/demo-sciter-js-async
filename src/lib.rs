use sciter::{make_args, types::*, vmap, Value};

/// Extension entry point.
#[no_mangle]
pub extern "system" fn SciterLibraryInit(
    api: &'static sciter::ISciterAPI,
    exported: &mut VALUE,
) -> BOOL {
    sciter::set_host_api(api);

    let ext_api = vmap! {
        "do_something" => do_something,
    };

    ext_api.pack_to(exported);

    true as BOOL
}

pub fn do_something(args: &[Value]) -> Value {
    if let [dur, resolve, reject] = args {
        let dur: u64 = dur.to_int().unwrap_or(2) as u64;
        let resolve = resolve.clone();
        let reject = reject.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(dur));
            if let Err(err) = resolve.call(None, &make_args!(dur as i32), None) {
                let _ = reject.call(None, &make_args!(err.to_string()), None);
            }
        });
    }

    Value::from(true)
}
