export type Node = {
  id: string;
  label?: string;
  data?: any;
  source? : null; // need these purely for type checking
  target? : null; // need these purely for type checking... they will never ben assigned
};

export type Edge = {
  id: string;
  source: string; // Node id
  target: string; // Node id
  label?: string;
  data?: any; // purely for type checking.. never will be assigned
};

export type Graph = {
  nodes: Node[];
  edges: Edge[];
};

export type selectedGraphComponent = {
    type: "Node" | "Edge" | null;
    instance: Node | Edge;
    neighbors: Node[] | null;
    outgoing: Edge[] | null;
    incoming: Edge[] | null;
}

export type GraphState = {
  graph: Graph;
  selected: selectedGraphComponent | null;
  lastAction: "addNode" | "addEdge" | "removeNode" | "removeEdge" | "selectNode" | "deselectNode" | "none" | "selectEdge" | "deselectEdge" | "updateNode" | "updateEdge" | "resetGraph";
  actedOn: Node | Edge | null;
};

export type Action = {
  _id: string,
  prompt: string,
  name: string,
  system: string
}

export type Process = {
  _id: string,
  name: string,
  steps: string[],
  trigger: string,
  triggers_next_process: string,
  waits_for_branch_completion: boolean,
  description: string,
  creates_process_branch: boolean,
  branch_step: string
}

export type Message = {
  role: string,
  content: string
}

export type AiSystemState = {
  actions: Action[],
  processes: Process[],
  messages: Message[]
}

export type SystemState = {
  websocketReady: boolean,
  selectedAction: Action | null,
  selectedProcess: Process | null,
}

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
}

export type UpdateAction = {
  action: Action;
};

export type CreateAction = {
  create_action: Action;
}

export type CreateProcess = {
  create_process: Process;
}

export type MessageTypes =
  | { type: 'Goal'; data: Goal }
  | { type: 'InitializeProject'; data: InitializeProject }
  | { type: 'SetOpenAIKey'; data: OpenaiKey }
  | { type: 'Prompt'; data: Prompt}
  | { type: 'UpdateAction'; data: UpdateAction}
  | { type: 'CreateAction'; data: CreateAction}
  | { type: 'CreateProcess'; data: CreateProcess}