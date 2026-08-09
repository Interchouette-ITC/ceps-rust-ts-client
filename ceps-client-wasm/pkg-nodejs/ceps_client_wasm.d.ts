/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

/**
 * WASM wrapper for [`Cep18Client`].
 */
export class Cep18Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Balance of `account` (`account-hash-…` or prefixed).
     */
    balanceOf(account: string): Promise<string>;
    /**
     * Chain name.
     */
    chainName(): string;
    /**
     * Install with required fields (+ optional CES events when `events_mode` is set).
     */
    install(name: string, symbol: string, decimals: number, total_supply: string, events_mode: number | null | undefined, wasm: Uint8Array, secret_key_pem: string, payment_amount: string, wait?: boolean | null): Promise<string>;
    /**
     * Token name.
     */
    name(): Promise<string>;
    /**
     * Create a CEP-18 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes (hex or prefixed).
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * SSE URL when set.
     */
    sseUrl(): string | undefined;
    /**
     * Token symbol.
     */
    symbol(): Promise<string>;
}

/**
 * WASM wrapper for [`Cep78Client`].
 */
export class Cep78Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Balance of owner.
     */
    balanceOf(owner: string): Promise<string>;
    /**
     * Collection name.
     */
    collectionName(): Promise<string>;
    /**
     * Install with defaults (Transferable / Raw / Ordinal) and optional events mode.
     */
    install(collection_name: string, collection_symbol: string, total_token_supply: bigint, events_mode: number | null | undefined, wasm: Uint8Array, secret_key_pem: string, payment_amount: string, wait?: boolean | null): Promise<string>;
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
    parseCes(transaction_hash: string): Promise<string>;
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * SSE URL when set.
     */
    sseUrl(): string | undefined;
}

/**
 * WASM wrapper for [`Cep85Client`].
 */
export class Cep85Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Balance for account + token id.
     */
    balanceOf(account: string, id: string): Promise<string>;
    /**
     * Collection name.
     */
    collectionName(): Promise<string>;
    /**
     * Install with URI and optional CES events / burn flag.
     */
    install(name: string, uri: string, events_mode: number | null | undefined, enable_burn: boolean | null | undefined, wasm: Uint8Array, secret_key_pem: string, payment_amount: string, wait?: boolean | null): Promise<string>;
    /**
     * Create a CEP-85 client.
     */
    constructor(rpc_url: string, sse_url?: string | null, chain_name?: string | null, verbosity?: number | null);
    /**
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * SSE URL when set.
     */
    sseUrl(): string | undefined;
}

/**
 * WASM wrapper for [`Cep95Client`].
 */
export class Cep95Client {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Balance of owner.
     */
    balanceOf(owner: string): Promise<string>;
    /**
     * Install Odra OwnedCep95 (or compatible) with package named-key name.
     */
    install(name: string, symbol: string, package_hash_key_name: string, wasm: Uint8Array, secret_key_pem: string, payment_amount: string, wait?: boolean | null): Promise<string>;
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
     * RPC URL.
     */
    rpcUrl(): string;
    /**
     * Bind contract hashes.
     */
    setContractHash(contract_hash: string, package_hash?: string | null): void;
    /**
     * SSE URL when set.
     */
    sseUrl(): string | undefined;
    /**
     * Collection symbol.
     */
    symbol(): Promise<string>;
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
