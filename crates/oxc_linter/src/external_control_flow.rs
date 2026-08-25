use oxc_cfg::{EdgeType, InstructionKind, graph::visit::EdgeRef};
use oxc_semantic::Semantic;
use oxc_span::{GetSpan, Span};
use serde::Serialize;

/// Compact, read-only projection of Oxc's native control-flow graph for JS plugins.
///
/// IDs are local to one linted file. Spans are converted to UTF-16 immediately before the
/// projection is sent to JS, so they match ESTree `range` values.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalControlFlow {
    pub blocks: Vec<ExternalControlFlowBlock>,
    pub edges: Vec<[u32; 3]>,
    pub nodes: Vec<[u32; 3]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalControlFlowBlock {
    pub id: u32,
    pub unreachable: bool,
    /// 0 = none, 1 = normal return, 2 = throw, 3 = both.
    pub exit_kind: u8,
}

impl ExternalControlFlow {
    pub fn from_semantic(semantic: &Semantic<'_>) -> Self {
        let cfg = semantic.cfg().expect("linter semantic must contain a control-flow graph");
        let graph = cfg.graph();

        let blocks = graph
            .node_indices()
            .map(|graph_id| {
                let block_id = graph[graph_id];
                let block = cfg.basic_block(graph_id);
                let mut exit_kind = 0;
                for instruction in block.instructions() {
                    match instruction.kind {
                        InstructionKind::ImplicitReturn | InstructionKind::Return(_) => {
                            exit_kind |= 1;
                        }
                        InstructionKind::Throw => exit_kind |= 2,
                        _ => {}
                    }
                }
                ExternalControlFlowBlock {
                    id: to_u32(block_id.index()),
                    unreachable: block.is_unreachable(),
                    exit_kind,
                }
            })
            .collect();

        let edges = graph
            .edge_references()
            .map(|edge| {
                [
                    to_u32(graph[edge.source()].index()),
                    to_u32(graph[edge.target()].index()),
                    edge_kind(edge.weight()),
                ]
            })
            .collect();

        let nodes = semantic
            .nodes()
            .iter_enumerated()
            .map(|(node_id, node)| {
                let block_id = graph[semantic.nodes().cfg_id(node_id)];
                let span = node.kind().span();
                [span.start, span.end, to_u32(block_id.index())]
            })
            .collect();

        Self { blocks, edges, nodes }
    }

    pub fn convert_spans_to_utf16(&mut self, converter: &mut impl FnMut(&mut Span)) {
        for node in &mut self.nodes {
            let mut span = Span::new(node[0], node[1]);
            converter(&mut span);
            node[0] = span.start;
            node[1] = span.end;
        }
    }
}

fn edge_kind(edge: &EdgeType) -> u32 {
    match edge {
        EdgeType::Normal => 0,
        EdgeType::Jump => 1,
        EdgeType::Backedge => 2,
        EdgeType::NewFunction => 3,
        EdgeType::Finalize => 4,
        EdgeType::Error(_) => 5,
        EdgeType::Unreachable => 6,
        EdgeType::Join => 7,
    }
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("control-flow ID must fit in u32")
}
