// package: skynet.types
// file: system_types.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class GraphNodeInfo extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GraphNodeInfo.AsObject;
  static toObject(includeInstance: boolean, msg: GraphNodeInfo): GraphNodeInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GraphNodeInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GraphNodeInfo;
  static deserializeBinaryFromReader(message: GraphNodeInfo, reader: jspb.BinaryReader): GraphNodeInfo;
}

export namespace GraphNodeInfo {
  export type AsObject = {
    id: string,
    name: string,
    description: string,
  }
}

export class Edge extends jspb.Message {
  hasSource(): boolean;
  clearSource(): void;
  getSource(): GraphNodeInfo | undefined;
  setSource(value?: GraphNodeInfo): void;

  hasTarget(): boolean;
  clearTarget(): void;
  getTarget(): GraphNodeInfo | undefined;
  setTarget(value?: GraphNodeInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Edge.AsObject;
  static toObject(includeInstance: boolean, msg: Edge): Edge.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Edge, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Edge;
  static deserializeBinaryFromReader(message: Edge, reader: jspb.BinaryReader): Edge;
}

export namespace Edge {
  export type AsObject = {
    source?: GraphNodeInfo.AsObject,
    target?: GraphNodeInfo.AsObject,
  }
}

export class Prompt extends jspb.Message {
  getPrompt(): string;
  setPrompt(value: string): void;

  getSystem(): string;
  setSystem(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Prompt.AsObject;
  static toObject(includeInstance: boolean, msg: Prompt): Prompt.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Prompt, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Prompt;
  static deserializeBinaryFromReader(message: Prompt, reader: jspb.BinaryReader): Prompt;
}

export namespace Prompt {
  export type AsObject = {
    prompt: string,
    system: string,
  }
}

export class Command extends jspb.Message {
  getCommand(): string;
  setCommand(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Command.AsObject;
  static toObject(includeInstance: boolean, msg: Command): Command.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Command, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Command;
  static deserializeBinaryFromReader(message: Command, reader: jspb.BinaryReader): Command;
}

export namespace Command {
  export type AsObject = {
    command: string,
  }
}

export class Conditional extends jspb.Message {
  getStatement(): string;
  setStatement(value: string): void;

  clearOptionsList(): void;
  getOptionsList(): Array<Node>;
  setOptionsList(value: Array<Node>): void;
  addOptions(value?: Node, index?: number): Node;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Conditional.AsObject;
  static toObject(includeInstance: boolean, msg: Conditional): Conditional.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Conditional, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Conditional;
  static deserializeBinaryFromReader(message: Conditional, reader: jspb.BinaryReader): Conditional;
}

export namespace Conditional {
  export type AsObject = {
    statement: string,
    optionsList: Array<Node.AsObject>,
  }
}

export class Graph extends jspb.Message {
  clearNodesInfoList(): void;
  getNodesInfoList(): Array<GraphNodeInfo>;
  setNodesInfoList(value: Array<GraphNodeInfo>): void;
  addNodesInfo(value?: GraphNodeInfo, index?: number): GraphNodeInfo;

  clearEdgesList(): void;
  getEdgesList(): Array<Edge>;
  setEdgesList(value: Array<Edge>): void;
  addEdges(value?: Edge, index?: number): Edge;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Graph.AsObject;
  static toObject(includeInstance: boolean, msg: Graph): Graph.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Graph, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Graph;
  static deserializeBinaryFromReader(message: Graph, reader: jspb.BinaryReader): Graph;
}

export namespace Graph {
  export type AsObject = {
    nodesInfoList: Array<GraphNodeInfo.AsObject>,
    edgesList: Array<Edge.AsObject>,
  }
}

export class Process extends jspb.Message {
  hasGraph(): boolean;
  clearGraph(): void;
  getGraph(): Graph | undefined;
  setGraph(value?: Graph): void;

  clearTopologicalOrderList(): void;
  getTopologicalOrderList(): Array<GraphNodeInfo>;
  setTopologicalOrderList(value: Array<GraphNodeInfo>): void;
  addTopologicalOrder(value?: GraphNodeInfo, index?: number): GraphNodeInfo;

  clearNodesList(): void;
  getNodesList(): Array<Node>;
  setNodesList(value: Array<Node>): void;
  addNodes(value?: Node, index?: number): Node;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Process.AsObject;
  static toObject(includeInstance: boolean, msg: Process): Process.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Process, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Process;
  static deserializeBinaryFromReader(message: Process, reader: jspb.BinaryReader): Process;
}

export namespace Process {
  export type AsObject = {
    graph?: Graph.AsObject,
    topologicalOrderList: Array<GraphNodeInfo.AsObject>,
    nodesList: Array<Node.AsObject>,
  }
}

export class NodeContent extends jspb.Message {
  hasPrompt(): boolean;
  clearPrompt(): void;
  getPrompt(): Prompt | undefined;
  setPrompt(value?: Prompt): void;

  hasProcess(): boolean;
  clearProcess(): void;
  getProcess(): Process | undefined;
  setProcess(value?: Process): void;

  hasConditional(): boolean;
  clearConditional(): void;
  getConditional(): Conditional | undefined;
  setConditional(value?: Conditional): void;

  hasCommand(): boolean;
  clearCommand(): void;
  getCommand(): Command | undefined;
  setCommand(value?: Command): void;

  getNodeContentCase(): NodeContent.NodeContentCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NodeContent.AsObject;
  static toObject(includeInstance: boolean, msg: NodeContent): NodeContent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: NodeContent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NodeContent;
  static deserializeBinaryFromReader(message: NodeContent, reader: jspb.BinaryReader): NodeContent;
}

export namespace NodeContent {
  export type AsObject = {
    prompt?: Prompt.AsObject,
    process?: Process.AsObject,
    conditional?: Conditional.AsObject,
    command?: Command.AsObject,
  }

  export enum NodeContentCase {
    NODE_CONTENT_NOT_SET = 0,
    PROMPT = 1,
    PROCESS = 2,
    CONDITIONAL = 3,
    COMMAND = 4,
  }
}

export class Node extends jspb.Message {
  hasNodeInfo(): boolean;
  clearNodeInfo(): void;
  getNodeInfo(): GraphNodeInfo | undefined;
  setNodeInfo(value?: GraphNodeInfo): void;

  clearInputVariablesList(): void;
  getInputVariablesList(): Array<VariableDefinition>;
  setInputVariablesList(value: Array<VariableDefinition>): void;
  addInputVariables(value?: VariableDefinition, index?: number): VariableDefinition;

  clearOutputVariablesList(): void;
  getOutputVariablesList(): Array<VariableDefinition>;
  setOutputVariablesList(value: Array<VariableDefinition>): void;
  addOutputVariables(value?: VariableDefinition, index?: number): VariableDefinition;

  getNodeType(): NodeTypesMap[keyof NodeTypesMap];
  setNodeType(value: NodeTypesMap[keyof NodeTypesMap]): void;

  hasNodeContent(): boolean;
  clearNodeContent(): void;
  getNodeContent(): NodeContent | undefined;
  setNodeContent(value?: NodeContent): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Node.AsObject;
  static toObject(includeInstance: boolean, msg: Node): Node.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Node, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Node;
  static deserializeBinaryFromReader(message: Node, reader: jspb.BinaryReader): Node;
}

export namespace Node {
  export type AsObject = {
    nodeInfo?: GraphNodeInfo.AsObject,
    inputVariablesList: Array<VariableDefinition.AsObject>,
    outputVariablesList: Array<VariableDefinition.AsObject>,
    nodeType: NodeTypesMap[keyof NodeTypesMap],
    nodeContent?: NodeContent.AsObject,
  }
}

export class Nodes extends jspb.Message {
  clearNodesList(): void;
  getNodesList(): Array<Node>;
  setNodesList(value: Array<Node>): void;
  addNodes(value?: Node, index?: number): Node;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Nodes.AsObject;
  static toObject(includeInstance: boolean, msg: Nodes): Nodes.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Nodes, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Nodes;
  static deserializeBinaryFromReader(message: Nodes, reader: jspb.BinaryReader): Nodes;
}

export namespace Nodes {
  export type AsObject = {
    nodesList: Array<Node.AsObject>,
  }
}

export class Identity extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getIpAddress(): string;
  setIpAddress(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Identity.AsObject;
  static toObject(includeInstance: boolean, msg: Identity): Identity.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Identity, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Identity;
  static deserializeBinaryFromReader(message: Identity, reader: jspb.BinaryReader): Identity;
}

export namespace Identity {
  export type AsObject = {
    id: string,
    ipAddress: string,
  }
}

export class Execution extends jspb.Message {
  hasCurrentNode(): boolean;
  clearCurrentNode(): void;
  getCurrentNode(): GraphNodeInfo | undefined;
  setCurrentNode(value?: GraphNodeInfo): void;

  hasProcess(): boolean;
  clearProcess(): void;
  getProcess(): Process | undefined;
  setProcess(value?: Process): void;

  clearCurrentVariableDefinitionsList(): void;
  getCurrentVariableDefinitionsList(): Array<VariableDefinition>;
  setCurrentVariableDefinitionsList(value: Array<VariableDefinition>): void;
  addCurrentVariableDefinitions(value?: VariableDefinition, index?: number): VariableDefinition;

  getExecutionId(): string;
  setExecutionId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Execution.AsObject;
  static toObject(includeInstance: boolean, msg: Execution): Execution.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Execution, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Execution;
  static deserializeBinaryFromReader(message: Execution, reader: jspb.BinaryReader): Execution;
}

export namespace Execution {
  export type AsObject = {
    currentNode?: GraphNodeInfo.AsObject,
    process?: Process.AsObject,
    currentVariableDefinitionsList: Array<VariableDefinition.AsObject>,
    executionId: string,
  }
}

export class VariableDefinition extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  hasStringValue(): boolean;
  clearStringValue(): void;
  getStringValue(): string;
  setStringValue(value: string): void;

  hasIntValue(): boolean;
  clearIntValue(): void;
  getIntValue(): number;
  setIntValue(value: number): void;

  hasFloatValue(): boolean;
  clearFloatValue(): void;
  getFloatValue(): number;
  setFloatValue(value: number): void;

  hasBoolValue(): boolean;
  clearBoolValue(): void;
  getBoolValue(): boolean;
  setBoolValue(value: boolean): void;

  getValueCase(): VariableDefinition.ValueCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VariableDefinition.AsObject;
  static toObject(includeInstance: boolean, msg: VariableDefinition): VariableDefinition.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: VariableDefinition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VariableDefinition;
  static deserializeBinaryFromReader(message: VariableDefinition, reader: jspb.BinaryReader): VariableDefinition;
}

export namespace VariableDefinition {
  export type AsObject = {
    name: string,
    stringValue: string,
    intValue: number,
    floatValue: number,
    boolValue: boolean,
  }

  export enum ValueCase {
    VALUE_NOT_SET = 0,
    STRING_VALUE = 2,
    INT_VALUE = 3,
    FLOAT_VALUE = 4,
    BOOL_VALUE = 5,
  }
}

export class SystemState extends jspb.Message {
  getAuthenticated(): boolean;
  setAuthenticated(value: boolean): void;

  getWebsocketReady(): boolean;
  setWebsocketReady(value: boolean): void;

  hasGraph(): boolean;
  clearGraph(): void;
  getGraph(): Graph | undefined;
  setGraph(value?: Graph): void;

  clearLocalNodesList(): void;
  getLocalNodesList(): Array<Node>;
  setLocalNodesList(value: Array<Node>): void;
  addLocalNodes(value?: Node, index?: number): Node;

  clearSelectedNodesList(): void;
  getSelectedNodesList(): Array<GraphNodeInfo>;
  setSelectedNodesList(value: Array<GraphNodeInfo>): void;
  addSelectedNodes(value?: GraphNodeInfo, index?: number): GraphNodeInfo;

  clearSelectedEdgesList(): void;
  getSelectedEdgesList(): Array<Edge>;
  setSelectedEdgesList(value: Array<Edge>): void;
  addSelectedEdges(value?: Edge, index?: number): Edge;

  hasExecutionStep(): boolean;
  clearExecutionStep(): void;
  getExecutionStep(): Execution | undefined;
  setExecutionStep(value?: Execution): void;

  hasSelectedProcess(): boolean;
  clearSelectedProcess(): void;
  getSelectedProcess(): Node | undefined;
  setSelectedProcess(value?: Node): void;

  clearBackendIdentitiesList(): void;
  getBackendIdentitiesList(): Array<Identity>;
  setBackendIdentitiesList(value: Array<Identity>): void;
  addBackendIdentities(value?: Identity, index?: number): Identity;

  clearPeerIdentitiesList(): void;
  getPeerIdentitiesList(): Array<Identity>;
  setPeerIdentitiesList(value: Array<Identity>): void;
  addPeerIdentities(value?: Identity, index?: number): Identity;

  hasPrimaryBackend(): boolean;
  clearPrimaryBackend(): void;
  getPrimaryBackend(): Identity | undefined;
  setPrimaryBackend(value?: Identity): void;

  hasClientIdentity(): boolean;
  clearClientIdentity(): void;
  getClientIdentity(): Identity | undefined;
  setClientIdentity(value?: Identity): void;

  clearMessageLogList(): void;
  getMessageLogList(): Array<Envelope>;
  setMessageLogList(value: Array<Envelope>): void;
  addMessageLog(value?: Envelope, index?: number): Envelope;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemState.AsObject;
  static toObject(includeInstance: boolean, msg: SystemState): SystemState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SystemState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemState;
  static deserializeBinaryFromReader(message: SystemState, reader: jspb.BinaryReader): SystemState;
}

export namespace SystemState {
  export type AsObject = {
    authenticated: boolean,
    websocketReady: boolean,
    graph?: Graph.AsObject,
    localNodesList: Array<Node.AsObject>,
    selectedNodesList: Array<GraphNodeInfo.AsObject>,
    selectedEdgesList: Array<Edge.AsObject>,
    executionStep?: Execution.AsObject,
    selectedProcess?: Node.AsObject,
    backendIdentitiesList: Array<Identity.AsObject>,
    peerIdentitiesList: Array<Identity.AsObject>,
    primaryBackend?: Identity.AsObject,
    clientIdentity?: Identity.AsObject,
    messageLogList: Array<Envelope.AsObject>,
  }
}

export class AuthenticationMessage extends jspb.Message {
  getClientEmail(): string;
  setClientEmail(value: string): void;

  getClientPassword(): string;
  setClientPassword(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthenticationMessage.AsObject;
  static toObject(includeInstance: boolean, msg: AuthenticationMessage): AuthenticationMessage.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthenticationMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthenticationMessage;
  static deserializeBinaryFromReader(message: AuthenticationMessage, reader: jspb.BinaryReader): AuthenticationMessage;
}

export namespace AuthenticationMessage {
  export type AsObject = {
    clientEmail: string,
    clientPassword: string,
  }
}

export class UserSettings extends jspb.Message {
  getOpenaiApiKey(): string;
  setOpenaiApiKey(value: string): void;

  getMongoDbUri(): string;
  setMongoDbUri(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSettings.AsObject;
  static toObject(includeInstance: boolean, msg: UserSettings): UserSettings.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UserSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSettings;
  static deserializeBinaryFromReader(message: UserSettings, reader: jspb.BinaryReader): UserSettings;
}

export namespace UserSettings {
  export type AsObject = {
    openaiApiKey: string,
    mongoDbUri: string,
  }
}

export class SystemError extends jspb.Message {
  getErrorMessage(): string;
  setErrorMessage(value: string): void;

  hasOriginator(): boolean;
  clearOriginator(): void;
  getOriginator(): Identity | undefined;
  setOriginator(value?: Identity): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemError.AsObject;
  static toObject(includeInstance: boolean, msg: SystemError): SystemError.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SystemError, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemError;
  static deserializeBinaryFromReader(message: SystemError, reader: jspb.BinaryReader): SystemError;
}

export namespace SystemError {
  export type AsObject = {
    errorMessage: string,
    originator?: Identity.AsObject,
  }
}

export class Letter extends jspb.Message {
  hasBody(): boolean;
  clearBody(): void;
  getBody(): Body | undefined;
  setBody(value?: Body): void;

  getVerb(): VerbTypesMap[keyof VerbTypesMap];
  setVerb(value: VerbTypesMap[keyof VerbTypesMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Letter.AsObject;
  static toObject(includeInstance: boolean, msg: Letter): Letter.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Letter, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Letter;
  static deserializeBinaryFromReader(message: Letter, reader: jspb.BinaryReader): Letter;
}

export namespace Letter {
  export type AsObject = {
    body?: Body.AsObject,
    verb: VerbTypesMap[keyof VerbTypesMap],
  }
}

export class Body extends jspb.Message {
  hasNode(): boolean;
  clearNode(): void;
  getNode(): Node | undefined;
  setNode(value?: Node): void;

  hasAuthenticationMessage(): boolean;
  clearAuthenticationMessage(): void;
  getAuthenticationMessage(): AuthenticationMessage | undefined;
  setAuthenticationMessage(value?: AuthenticationMessage): void;

  hasUserSettings(): boolean;
  clearUserSettings(): void;
  getUserSettings(): UserSettings | undefined;
  setUserSettings(value?: UserSettings): void;

  hasExecutionDetails(): boolean;
  clearExecutionDetails(): void;
  getExecutionDetails(): Execution | undefined;
  setExecutionDetails(value?: Execution): void;

  hasErrors(): boolean;
  clearErrors(): void;
  getErrors(): SystemError | undefined;
  setErrors(value?: SystemError): void;

  hasIdentity(): boolean;
  clearIdentity(): void;
  getIdentity(): Identity | undefined;
  setIdentity(value?: Identity): void;

  hasNodesToProcess(): boolean;
  clearNodesToProcess(): void;
  getNodesToProcess(): NodesToProcess | undefined;
  setNodesToProcess(value?: NodesToProcess): void;

  getContentsCase(): Body.ContentsCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Body.AsObject;
  static toObject(includeInstance: boolean, msg: Body): Body.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Body, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Body;
  static deserializeBinaryFromReader(message: Body, reader: jspb.BinaryReader): Body;
}

export namespace Body {
  export type AsObject = {
    node?: Node.AsObject,
    authenticationMessage?: AuthenticationMessage.AsObject,
    userSettings?: UserSettings.AsObject,
    executionDetails?: Execution.AsObject,
    errors?: SystemError.AsObject,
    identity?: Identity.AsObject,
    nodesToProcess?: NodesToProcess.AsObject,
  }

  export enum ContentsCase {
    CONTENTS_NOT_SET = 0,
    NODE = 1,
    AUTHENTICATION_MESSAGE = 2,
    USER_SETTINGS = 3,
    EXECUTION_DETAILS = 4,
    ERRORS = 5,
    IDENTITY = 6,
    NODES_TO_PROCESS = 7,
  }
}

export class NodesToProcess extends jspb.Message {
  clearNodesList(): void;
  getNodesList(): Array<Node>;
  setNodesList(value: Array<Node>): void;
  addNodes(value?: Node, index?: number): Node;

  hasContainingNodeInfo(): boolean;
  clearContainingNodeInfo(): void;
  getContainingNodeInfo(): GraphNodeInfo | undefined;
  setContainingNodeInfo(value?: GraphNodeInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NodesToProcess.AsObject;
  static toObject(includeInstance: boolean, msg: NodesToProcess): NodesToProcess.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: NodesToProcess, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NodesToProcess;
  static deserializeBinaryFromReader(message: NodesToProcess, reader: jspb.BinaryReader): NodesToProcess;
}

export namespace NodesToProcess {
  export type AsObject = {
    nodesList: Array<Node.AsObject>,
    containingNodeInfo?: GraphNodeInfo.AsObject,
  }
}

export class Envelope extends jspb.Message {
  clearLettersList(): void;
  getLettersList(): Array<Letter>;
  setLettersList(value: Array<Letter>): void;
  addLetters(value?: Letter, index?: number): Letter;

  hasSender(): boolean;
  clearSender(): void;
  getSender(): Identity | undefined;
  setSender(value?: Identity): void;

  hasReceiver(): boolean;
  clearReceiver(): void;
  getReceiver(): Identity | undefined;
  setReceiver(value?: Identity): void;

  getVerificationId(): string;
  setVerificationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Envelope.AsObject;
  static toObject(includeInstance: boolean, msg: Envelope): Envelope.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Envelope, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Envelope;
  static deserializeBinaryFromReader(message: Envelope, reader: jspb.BinaryReader): Envelope;
}

export namespace Envelope {
  export type AsObject = {
    lettersList: Array<Letter.AsObject>,
    sender?: Identity.AsObject,
    receiver?: Identity.AsObject,
    verificationId: string,
  }
}

export interface NodeTypesMap {
  PROMPT: 0;
  PROCESS: 1;
  CONDITIONAL: 2;
  COMMAND: 3;
}

export const NodeTypes: NodeTypesMap;

export interface VerbTypesMap {
  CREATE: 0;
  UPDATE: 1;
  REPLACE: 2;
  DELETE: 3;
  GET: 4;
  EXECUTE: 5;
  VALIDATE: 6;
  ACKNOWLEDGE: 7;
  INITIATE: 8;
  AUTHORIZED: 9;
}

export const VerbTypes: VerbTypesMap;

