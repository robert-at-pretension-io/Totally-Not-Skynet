
export type Node = {
  id: string;
  label?: string;
  data?: Action;
  type: "action" | "variable"
};

export type Edge = {
  id: string;
  source: string; // Node id
  target: string; // Node id
  label?: string;
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
  actedOn: {id: string} | null;
};

export type Action = {
  _id: Object,
  prompt: string,
  input_variables: string[],
  output_variables: string[],
  name: string,
  system: string
}



export type Process = {
  _id: Object,
  name: string,
  steps: string[],
  trigger: string,
  triggers_next_process: string,
  description: string,
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
  currentlySelected: "action" | "process" | "none",
  websocketReady: boolean,
  selectedAction: Action ,
  selectedProcess: Process ,
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