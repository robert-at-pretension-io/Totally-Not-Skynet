import { Graph, GraphAction, GraphState, SystemState } from "../generated/system_types_pb";

export function areAllPropertiesUndefined<T extends object>(obj: T): boolean {
  return Object.values(obj).every((value) => value === undefined);
}

export function isInstanceOf<T>(obj: any, types: (new () => T)[]): boolean {
  return types.some((type) => obj instanceof type);
}

export function stringToUint8Array(str: string): Uint8Array {
  const utf8Encoder = new TextEncoder();
  return utf8Encoder.encode(str);
}

export function initializeSystemState(): SystemState {
  const system_state = new SystemState();
  const graph_state = new GraphState();
  const graph = new Graph();
  const action_history: GraphAction[] = [];

  graph_state.setGraph(graph);
  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);
  system_state.setAuthenticated(false);
  system_state.setWebsocketReady(false);
  system_state.setNodesList([]);
  system_state.setSelectedNodeList([]);

  return system_state;
}