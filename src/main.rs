use self::args::CmdArgs;
use cairo_lang_sierra::{extensions::core::CoreTypeConcrete, ProgramParser};
use clap::Parser;
use sierra_emu::{ProgramTrace, StateDump, Value, VirtualMachine};
use std::{
    fs::{self, File},
    io::stdout,
    sync::Arc,
};
use tracing::{debug, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdArgs::parse();

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    info!("Loading the Sierra program from disk.");
    let source_code = fs::read_to_string(args.program)?;

    info!("Parsing the Sierra program.");
    let program = Arc::new(
        ProgramParser::new()
            .parse(&source_code)
            .map_err(|e| e.to_string())?,
    );

    info!("Preparing the virtual machine.");
    let mut vm = VirtualMachine::new(program.clone());

    debug!("Pushing the entry point's frame.");
    let function = program
        .funcs
        .iter()
        .find(|f| match &args.entry_point {
            args::EntryPoint::Number(x) => f.id.id == *x,
            args::EntryPoint::String(x) => f.id.debug_name.as_deref() == Some(x.as_str()),
        })
        .unwrap();

    debug!(
        "Entry point argument types: {:?}",
        function.signature.param_types
    );
    let mut iter = args.args.into_iter();
    vm.push_frame(
        function.id.clone(),
        function
            .signature
            .param_types
            .iter()
            .map(|type_id| {
                let type_info = vm.registry().get_type(type_id).unwrap();
                match type_info {
                    CoreTypeConcrete::Felt252(_) => Value::parse_felt(&iter.next().unwrap()),
                    CoreTypeConcrete::GasBuiltin(_) => Value::U128(args.available_gas.unwrap()),
                    CoreTypeConcrete::RangeCheck(_) | CoreTypeConcrete::SegmentArena(_) => {
                        Value::Unit
                    }
                    _ => todo!(),
                }
            })
            .collect::<Vec<_>>(),
    );

    let mut trace = ProgramTrace::new();

    info!("Running the program.");
    while let Some((statement_idx, state)) = vm.step() {
        trace.push(StateDump::new(statement_idx, state));
    }

    match args.output {
        Some(path) => serde_json::to_writer(File::create(path)?, &trace)?,
        None => serde_json::to_writer(stdout().lock(), &trace)?,
    };

    Ok(())
}
