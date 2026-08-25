export const enum ControlFlowEdgeKind {
  Normal = 0,
  Jump = 1,
  Backedge = 2,
  NewFunction = 3,
  Finalize = 4,
  Error = 5,
  Unreachable = 6,
  Join = 7,
}

export interface ControlFlowBlock {
  readonly id: number;
  readonly unreachable: boolean;
  /** 0 = none, 1 = normal return, 2 = throw, 3 = both. */
  readonly exitKind: number;
}

export interface ControlFlowGraph {
  readonly blocks: readonly ControlFlowBlock[];
  /** Tuples of `[fromBlockId, toBlockId, edgeKind]`. */
  readonly edges: readonly (readonly [number, number, ControlFlowEdgeKind])[];
  /** Tuples of `[nodeStart, nodeEnd, blockId]`, using ESTree UTF-16 offsets. */
  readonly nodes: readonly (readonly [number, number, number])[];
}

let controlFlowGraph: ControlFlowGraph | null = null;

export function setupControlFlowGraph(json: string): void {
  controlFlowGraph = JSON.parse(json) as ControlFlowGraph;
}

export function resetControlFlowGraph(): void {
  controlFlowGraph = null;
}

export function getControlFlowGraph(): ControlFlowGraph {
  if (controlFlowGraph === null) throw new Error("Control-flow graph is not available");
  return controlFlowGraph;
}
