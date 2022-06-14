use clap::Parser;
use glob;
use std::{fmt::Debug, io::Read};
use tracing::{error, info};

const STACK_SIZE: usize = 4 * 1024 * 1024;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Input glob for .pyc files")]
    inputs: String,

    #[clap(
        help = "Desired output base to put re-targeted/decompiled scripts to",
        short,
        long
    )]
    out_base: Option<String>,

    #[clap(help = "Run decompile", short, long, takes_value = false)]
    decompile: bool,
}

fn run() -> anyhow::Result<()> {
    use rustpython_vm as vm;

    let args = Args::parse();

    let mut settings: vm::vm::Settings = Default::default();
    settings.allow_external_library = true;
    settings.debug = false;
    settings.optimize = 2;

    let out_base = args.out_base.unwrap_or_else(|| ".".to_string());

    // For retrigger build on changes :)
    std::include_str!("../scripts/modules/pymarshal_remap.py");

    vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
        vm.add_frozen(rustpython_vm::py_freeze!(dir = "scripts/modules"));
    })
    .enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let pyc_retarget = std::include_str!("../scripts/pyc_retarget.py");
        let pyc_decompile = std::include_str!("../scripts/pyc_decompile.py");

        // let code = vm
        //     .compile_with_opts(
        //         &pymarshal,
        //         vm::compile::Mode::Exec,
        //         "pymarshal_remap.py".to_string(),
        //         vm.compile_opts(),
        //     )
        //     .unwrap();
        // import_codeobj(vm, "pymarshal_remap", code, false).unwrap();

        let code_obj = vm
            .compile(
                pyc_retarget,
                vm::compile::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err));
        if let Err(e) = &code_obj {
            vm.print_exception(e.clone());
            anyhow::bail!("Failed to load retargeter");
        }
        let code_obj = code_obj.unwrap();

        if let Err(e) = vm.run_code_obj(code_obj, scope.clone()) {
            vm.print_exception(e.clone());
        }

        let code_obj = vm
            .compile(
                pyc_decompile,
                vm::compile::Mode::Exec,
                "<embedded_decompiler>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err));
        if let Err(e) = &code_obj {
            vm.print_exception(e.clone());
            anyhow::bail!("Failed to load decompiler");
        }
        let code_obj = code_obj.unwrap();

        if let Err(e) = vm.run_code_obj(code_obj, scope.clone()) {
            vm.print_exception(e.clone());
        }

        let retarget_file = scope.globals.get_item("retarget_file", vm).unwrap();
        let decompile_file = scope.globals.get_item("decompile_file", vm).unwrap();
        let _decompile_multiple_files = scope
            .globals
            .get_item("decompile_multiple_files", vm)
            .unwrap();

        let _retarget_buffer = scope.globals.get_item("retarget_buffer", vm).unwrap();

        let mut file_success = 0;
        let mut file_failed = 0;
        let mut files_processed = 0;

        let globs = glob::glob(&args.inputs).expect("Failed to read glob pattern");
        let total_files = globs.count();

        for entry in glob::glob(&args.inputs).expect("Failed to read glob pattern") {
            info!("{}/{}", files_processed, total_files);
            files_processed += 1;
            match entry {
                Ok(path) => {
                    info!("Processing {:?}", path.display());
                    let head = {
                        let mut file = std::io::BufReader::new(std::fs::File::open(&path)?);
                        let mut head = [0; 8];
                        file.read_exact(&mut head)?;
                        head
                    };

                    let target = std::path::Path::new(&out_base).join(&path);

                    if &head != b"\x03\xf3\x0d\x0a\xff\xff\xff\xff" {
                        info!("Retargeting {:?}", path.display());
                        if let Some(parent) = target.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        if let Err(e) = vm.invoke(
                            &retarget_file,
                            (path.display().to_string(), target.display().to_string()),
                        ) {
                            vm.print_exception(e.clone());
                        }
                    } else {
                        info!("Already re-targetted, copy");
                        if let Some(parent) = target.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        let path = std::fs::canonicalize(&path)?;
                        let target = match std::fs::canonicalize(&target) {
                            Ok(target) => target,
                            Err(_) => target,
                        };
                        if path != target {
                            std::fs::copy(&path, &target)?;
                        }
                    }

                    if args.decompile {
                        info!("Decompiling");
                        let target = std::path::Path::new(&out_base).join(&path);
                        match vm.invoke(&decompile_file, (target.display().to_string(),)) {
                            Ok(v) => {
                                if !v.is_true(vm).unwrap() {
                                    error!("Decompile of {} failed", target.display());
                                    file_failed += 1;
                                } else {
                                    file_success += 1;
                                }
                            }
                            Err(e) => {
                                file_failed += 1;
                                vm.print_exception(e.clone())
                            }
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }

        info!("Success: {} | Failed: {}", file_success, file_failed);

        // vm.get_method_or_type_error(code_obj, method_name, err_msg)
        Ok(())
    })
}

fn main() -> anyhow::Result<()> {
    use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

    #[macro_export]
    macro_rules! safe_unwrap {
        ($result:expr) => {
            $result.unwrap()
        };
    }

    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();
    let filter = filter.add_directive(safe_unwrap!("rustpython_vm::frame=error".parse()));

    let subscriber = if cfg!(debug_assertions) {
        FmtSubscriber::builder().with_max_level(tracing::Level::DEBUG)
    } else {
        FmtSubscriber::builder().with_max_level(tracing::Level::INFO)
    }
    .with_env_filter(filter);
    let _ = subscriber.try_init();

    let child = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap()
}
