use graph::prelude::*;

use log::info;
use std::path::Path as StdPath;
use std::time::Instant;

use super::*;

pub(crate) fn triangle_count(args: CommonArgs, relabel: bool) -> Result<()> {
    let CommonArgs {
        path,
        format,
        use_32_bit,
        runs,
        warmup_runs,
    } = args;

    info!(
        "Reading graph ({} bit) from: {:?}",
        if use_32_bit { "32" } else { "64" },
        path
    );

    match (use_32_bit, format) {
        (true, FileFormat::EdgeList) => {
            run::<u32, _, _>(path, EdgeListInput::default(), runs, warmup_runs, relabel)
        }
        (true, FileFormat::Graph500) => {
            run::<u32, _, _>(path, Graph500Input::default(), runs, warmup_runs, relabel)
        }
        (false, FileFormat::EdgeList) => {
            run::<usize, _, _>(path, EdgeListInput::default(), runs, warmup_runs, relabel)
        }
        (false, FileFormat::Graph500) => {
            run::<usize, _, _>(path, Graph500Input::default(), runs, warmup_runs, relabel)
        }
    }
}

fn run<NI, Format, Path>(
    path: Path,
    file_format: Format,
    runs: usize,
    warmup_runs: usize,
    relabel: bool,
) -> Result<()>
where
    NI: Idx,
    Path: AsRef<StdPath>,
    Format: InputCapabilities<NI>,
    Format::GraphInput: TryFrom<InputPath<Path>>,
    UndirectedCsrGraph<NI>: TryFrom<(Format::GraphInput, CsrLayout)>,
    Error: From<<Format::GraphInput as TryFrom<InputPath<Path>>>::Error>,
    Error: From<<UndirectedCsrGraph<NI> as TryFrom<(Format::GraphInput, CsrLayout)>>::Error>,
{
    let mut graph: UndirectedCsrGraph<NI> = GraphBuilder::new()
        .csr_layout(CsrLayout::Deduplicated)
        .file_format(file_format)
        .path(path)
        .build()
        .unwrap();

    if relabel {
        relabel_graph(&mut graph);
    }

    for run in 1..=warmup_runs {
        let start = Instant::now();
        global_triangle_count(&graph);
        let took = start.elapsed();

        info!(
            "Warm-up run {} of {} finished in {:.6?}",
            run, warmup_runs, took,
        );
    }

    let mut durations = vec![];

    for run in 1..=runs {
        let start = Instant::now();
        global_triangle_count(&graph);
        let took = start.elapsed();
        durations.push(took);

        info!("Run {} of {} finished in {:.6?}", run, runs, took);
    }

    let total = durations
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or_default();

    info!("Average runtime: {:?}", total / runs as u32);

    Ok(())
}
