import { Edge } from "@dagrejs/graphlib";
import type { Graph, json } from "graphlib";

export type selectedGraphComponent = {
  type: "Node" | "Edge" | null;
};

export type GraphState = {
  graph: Graph;
  lastAction:
  | "addNode"
  | "addEdge"
  | "removeNode"
  | "removeEdge"
  | "selectNode"
  | "deselectNode"
  | "none"
  | "selectEdge"
  | "deselectEdge"
  | "updateNode"
  | "updateEdge"
  | "resetGraph";
  actedOn: Edge | [string, string] | null;
  lastActedOn: Edge | [string, string] | null;
  name: string | null;
  global_variables: Map<string, string>;
};

export type Action = {
  _id: MongoId;
  prompt: string;
  input_variables: string[];
  output_variables: string[];
  name: string;
  system: string;
};

export type MongoId = {
  $oid: string;
};

export type Process = {
  _id: MongoId;
  name: string;
  graph: Graph | Object;
  description: string;
  topological_order: string[];
};

export type Message = {
  role: string;
  content: string;
};

export type AiSystemState = {
  actions: Action[];
  processes: Process[];
  messages: Message[];
};

export type SystemState = {
  currentlySelected: "action" | "process" | "none";
  websocketReady: boolean;
  selectedAction: Action;
  selectedProcess: Process;
};

export type Goal = {
  text: string;
};

export type InitializeProject = {
  initial_message: string;
};

export type OpenaiKey = {
  key: string;
};

export type Prompt = {
  prompt_text: string;
};

export type UpdateAction = {
  action: Action;
};

export type CreateAction = {
  create_action: Action;
};

export type CreateProcess = {
  create_process: Process;
};

export type MessageTypes =
  | { type: "Goal"; data: Goal }
  | { type: "InitializeProject"; data: InitializeProject }
  | { type: "SetOpenAIKey"; data: OpenaiKey }
  | { type: "Prompt"; data: Prompt }
  | { type: "UpdateAction"; data: UpdateAction }
  | { type: "CreateAction"; data: CreateAction }
  | { type: "CreateProcess"; data: CreateProcess };
