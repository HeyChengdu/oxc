import type { ESTree, Plugin, Rule } from "#oxlint/plugins";

const rule: Rule = {
  create(context) {
    let branch: ESTree.IfStatement | null = null;

    return {
      IfStatement(node) {
        branch = node;
      },
      "Program:exit"(node) {
        const graph = context.sourceCode.controlFlowGraph;
        if (graph.blocks.length === 0 || graph.edges.length === 0 || graph.nodes.length === 0) {
          throw new Error("Native control-flow graph is empty");
        }
        if (branch === null || branch.range === undefined) {
          throw new Error("Expected branch was not visited");
        }
        const [start, end] = branch.range;
        if (!graph.nodes.some(([nodeStart, nodeEnd]) => nodeStart === start && nodeEnd === end)) {
          throw new Error("Native control-flow node ranges do not match ESTree ranges");
        }
        if (!graph.blocks.some(block => (block.exitKind & 1) !== 0)) {
          throw new Error("Normal return block is missing");
        }
        if (!graph.blocks.some(block => (block.exitKind & 2) !== 0)) {
          throw new Error("Throw block is missing");
        }

        context.report({ message: "Native Oxc CFG is available to JS plugins.", node });
      },
    };
  },
};

const plugin: Plugin = {
  meta: { name: "native-cfg" },
  rules: { "native-cfg": rule },
};

export default plugin;
