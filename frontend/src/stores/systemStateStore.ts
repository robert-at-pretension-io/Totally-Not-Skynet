import { writable } from 'svelte/store';
import { SystemState } from '../system_types';

// Replace 'ws://example.com' with your WebSocket server URL
const system_state : SystemState = {
    websocketReady: false,
}

const systemStateStore = writable(system_state);

export default systemStateStore;