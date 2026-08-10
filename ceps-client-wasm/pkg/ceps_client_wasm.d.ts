/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

/**
 * WASM wrapper for [`CEP18Client`].
 */
export class CEP18Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * SSE URL when set.
     */
    SSEUrl(): string | undefined;
    /**
     * Balance of `account` (`account-hash-…` or prefixed).
     */
    balanceOf(account: string): Promise<string>;
    /**
     * Chain name.
     */
    chainName(): string;
    /**
     * Collect SSE processed frames and decode CES for the bound contract.
     */
    collectCESEvents(event_names: string[], max_transactions?: number | null, timeout_ms?: bigint | null): Promise<string>;
    /**
     * Install with required fields (+ optional CES events when `events_mode` is set).
     */
    install(name: string, symbol: string, decimals: number, total_supply: string, events_mode: number | null | undefined, wasm: Uint8Array, secret_key_pem: string | null | undefined, payment_amount: string, wait?: boolean | null, make_only?: boolean | null, initiator_addr?: string | null): Promise<string>;
    /**
     * Token name.
     */
    name(): Promise<string>;
    /**
     * Create a CEP-18 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * Parse CES events for a transaction against the bound contract.
     */
    parseCES(transaction_hash: string): Promise<string>;
    /**
     * Put signed Transaction JSON (`CEPClient::put_transaction`).
     */
    putTransaction(transaction_json: string, wait?: boolean | null, wait_timeout_ms?: bigint | null): Promise<string>;
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes (hex or prefixed).
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * Token symbol.
     */
    symbol(): Promise<string>;
    /**
     * Wait for a transaction hash on SSE.
     */
    waitTransaction(transaction_hash: string, timeout_ms?: bigint | null): Promise<string>;
}

/**
 * WASM wrapper for [`CEP78Client`].
 */
export class CEP78Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * SSE URL when set.
     */
    SSEUrl(): string | undefined;
    /**
     * Balance of owner.
     */
    balanceOf(owner: string): Promise<string>;
    /**
     * Collect SSE processed frames and decode CES for the bound contract.
     */
    collectCESEvents(event_names: string[], max_transactions?: number | null, timeout_ms?: bigint | null): Promise<string>;
    /**
     * Collection name.
     */
    collectionName(): Promise<string>;
    /**
     * Install with defaults (Transferable / Raw / Ordinal) and optional events mode.
     */
    install(collection_name: string, collection_symbol: string, total_token_supply: bigint, events_mode: number | null | undefined, wasm: Uint8Array, secret_key_pem: string | null | undefined, payment_amount: string, wait?: boolean | null, make_only?: boolean | null, initiator_addr?: string | null): Promise<string>;
    /**
     * Create a CEP-78 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * Ownership mode (`u8`).
     */
    ownershipMode(): Promise<number>;
    /**
     * Parse CES events for a transaction against the bound contract.
     */
    parseCES(transaction_hash: string): Promise<string>;
    /**
     * Put signed Transaction JSON (`CEPClient::put_transaction`).
     */
    putTransaction(transaction_json: string, wait?: boolean | null, wait_timeout_ms?: bigint | null): Promise<string>;
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * Wait for a transaction hash on SSE.
     */
    waitTransaction(transaction_hash: string, timeout_ms?: bigint | null): Promise<string>;
}

/**
 * WASM wrapper for [`CEP85Client`].
 */
export class CEP85Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * SSE URL when set.
     */
    SSEUrl(): string | undefined;
    /**
     * Balance for account + token id.
     */
    balanceOf(account: string, id: string): Promise<string>;
    /**
     * Collect SSE processed frames and decode CES for the bound contract.
     */
    collectCESEvents(event_names: string[], max_transactions?: number | null, timeout_ms?: bigint | null): Promise<string>;
    /**
     * Collection name.
     */
    collectionName(): Promise<string>;
    /**
     * Install with URI and optional CES events / burn flag.
     */
    install(name: string, uri: string, events_mode: number | null | undefined, enable_burn: boolean | null | undefined, wasm: Uint8Array, secret_key_pem: string | null | undefined, payment_amount: string, wait?: boolean | null, make_only?: boolean | null, initiator_addr?: string | null): Promise<string>;
    /**
     * Create a CEP-85 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * Parse CES events for a transaction against the bound contract.
     */
    parseCES(transaction_hash: string): Promise<string>;
    /**
     * Put signed Transaction JSON (`CEPClient::put_transaction`).
     */
    putTransaction(transaction_json: string, wait?: boolean | null, wait_timeout_ms?: bigint | null): Promise<string>;
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * Wait for a transaction hash on SSE.
     */
    waitTransaction(transaction_hash: string, timeout_ms?: bigint | null): Promise<string>;
}

/**
 * WASM wrapper for [`CEP95Client`].
 */
export class CEP95Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * SSE URL when set.
     */
    SSEUrl(): string | undefined;
    /**
     * Balance of owner.
     */
    balanceOf(owner: string): Promise<string>;
    /**
     * Collect SSE processed frames and decode CES for the bound contract.
     */
    collectCESEvents(event_names: string[], max_transactions?: number | null, timeout_ms?: bigint | null): Promise<string>;
    /**
     * Install Odra OwnedCEP95 (or compatible) with package named-key name.
     */
    install(name: string, symbol: string, package_hash_key_name: string, wasm: Uint8Array, secret_key_pem: string | null | undefined, payment_amount: string, wait?: boolean | null, make_only?: boolean | null, initiator_addr?: string | null): Promise<string>;
    /**
     * Collection name.
     */
    name(): Promise<string>;
    /**
     * Create a CEP-95 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * Owner of token id.
     */
    ownerOf(token_id: string): Promise<string>;
    /**
     * Parse CES events for a transaction against the bound contract.
     */
    parseCES(transaction_hash: string): Promise<string>;
    /**
     * Put signed Transaction JSON (`CEPClient::put_transaction`).
     */
    putTransaction(transaction_json: string, wait?: boolean | null, wait_timeout_ms?: bigint | null): Promise<string>;
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * Collection symbol.
     */
    symbol(): Promise<string>;
    /**
     * Wait for a transaction hash on SSE.
     */
    waitTransaction(transaction_hash: string, timeout_ms?: bigint | null): Promise<string>;
}

export class IntoUnderlyingByteSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableByteStreamController): Promise<any>;
    start(controller: ReadableByteStreamController): void;
    readonly autoAllocateChunkSize: number;
    readonly type: ReadableStreamType;
}

export class IntoUnderlyingSink {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    abort(reason: any): Promise<any>;
    close(): Promise<any>;
    write(chunk: any): Promise<any>;
}

export class IntoUnderlyingSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableStreamDefaultController): Promise<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_cep18client_free: (a: number, b: number) => void;
    readonly cep18client_SSEUrl: (a: number) => [number, number];
    readonly cep18client_balanceOf: (a: number, b: number, c: number) => any;
    readonly cep18client_chainName: (a: number) => [number, number];
    readonly cep18client_collectCESEvents: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep18client_install: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: any, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => any;
    readonly cep18client_name: (a: number) => any;
    readonly cep18client_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly cep18client_parseCES: (a: number, b: number, c: number) => any;
    readonly cep18client_putTransaction: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep18client_rpcUrl: (a: number) => [number, number];
    readonly cep18client_setContractHash: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly cep18client_symbol: (a: number) => any;
    readonly cep18client_waitTransaction: (a: number, b: number, c: number, d: number, e: bigint) => any;
    readonly cep78client_balanceOf: (a: number, b: number, c: number) => any;
    readonly cep78client_collectCESEvents: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep78client_collectionName: (a: number) => any;
    readonly cep78client_install: (a: number, b: number, c: number, d: number, e: number, f: bigint, g: number, h: any, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => any;
    readonly cep78client_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly cep78client_ownershipMode: (a: number) => any;
    readonly cep78client_parseCES: (a: number, b: number, c: number) => any;
    readonly cep78client_putTransaction: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep78client_waitTransaction: (a: number, b: number, c: number, d: number, e: bigint) => any;
    readonly cep85client_balanceOf: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly cep85client_collectCESEvents: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep85client_collectionName: (a: number) => any;
    readonly cep85client_install: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: any, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => any;
    readonly cep85client_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly cep85client_parseCES: (a: number, b: number, c: number) => any;
    readonly cep85client_putTransaction: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep85client_waitTransaction: (a: number, b: number, c: number, d: number, e: bigint) => any;
    readonly cep95client_balanceOf: (a: number, b: number, c: number) => any;
    readonly cep95client_collectCESEvents: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep95client_install: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: any, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => any;
    readonly cep95client_name: (a: number) => any;
    readonly cep95client_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly cep95client_ownerOf: (a: number, b: number, c: number) => any;
    readonly cep95client_parseCES: (a: number, b: number, c: number) => any;
    readonly cep95client_putTransaction: (a: number, b: number, c: number, d: number, e: number, f: bigint) => any;
    readonly cep95client_symbol: (a: number) => any;
    readonly cep95client_waitTransaction: (a: number, b: number, c: number, d: number, e: bigint) => any;
    readonly cep78client_SSEUrl: (a: number) => [number, number];
    readonly cep85client_SSEUrl: (a: number) => [number, number];
    readonly cep95client_SSEUrl: (a: number) => [number, number];
    readonly cep78client_rpcUrl: (a: number) => [number, number];
    readonly cep85client_rpcUrl: (a: number) => [number, number];
    readonly cep95client_rpcUrl: (a: number) => [number, number];
    readonly cep78client_setContractHash: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly __wbg_cep78client_free: (a: number, b: number) => void;
    readonly cep85client_setContractHash: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly __wbg_cep85client_free: (a: number, b: number) => void;
    readonly cep95client_setContractHash: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly __wbg_cep95client_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingbytesource_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsink_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsource_free: (a: number, b: number) => void;
    readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
    readonly intounderlyingbytesource_cancel: (a: number) => void;
    readonly intounderlyingbytesource_pull: (a: number, b: any) => any;
    readonly intounderlyingbytesource_start: (a: number, b: any) => void;
    readonly intounderlyingbytesource_type: (a: number) => number;
    readonly intounderlyingsink_abort: (a: number, b: any) => any;
    readonly intounderlyingsink_close: (a: number) => any;
    readonly intounderlyingsink_write: (a: number, b: any) => any;
    readonly intounderlyingsource_cancel: (a: number) => void;
    readonly intounderlyingsource_pull: (a: number, b: any) => any;
    readonly wasm_bindgen_fcdb2a1b1112ab01___convert__closures_____invoke___wasm_bindgen_fcdb2a1b1112ab01___JsValue__core_7a2330d63e03cc2c___result__Result_____wasm_bindgen_fcdb2a1b1112ab01___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_fcdb2a1b1112ab01___convert__closures_____invoke___js_sys_1be2f9119c9634b3___Function_fn_wasm_bindgen_fcdb2a1b1112ab01___JsValue_____wasm_bindgen_fcdb2a1b1112ab01___sys__Undefined___js_sys_1be2f9119c9634b3___Function_fn_wasm_bindgen_fcdb2a1b1112ab01___JsValue_____wasm_bindgen_fcdb2a1b1112ab01___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_fcdb2a1b1112ab01___convert__closures_____invoke___wasm_bindgen_fcdb2a1b1112ab01___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
