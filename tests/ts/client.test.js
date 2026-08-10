import { describe, test, expect } from 'vitest';
import { CEP18Client, CEP78Client, CEP85Client } from 'ceps-client-wasm';

describe('ceps-client-wasm constructors', () => {
  test('CEP18Client normalizes RPC URL', () => {
    const client = new CEP18Client('http://127.0.0.1:11101', undefined, 'casper-net-1', 0);
    expect(client.rpcUrl()).toBe('http://127.0.0.1:11101/rpc');
    expect(client.chainName()).toBe('casper-net-1');
  });

  test('CEP18Client rejects empty RPC', () => {
    expect(() => new CEP18Client('', undefined, undefined, 0)).toThrow();
  });

  test('CEP78Client constructs', () => {
    const client = new CEP78Client('http://127.0.0.1:11101', 'http://127.0.0.1:18101/events');
    expect(client.rpcUrl()).toContain('/rpc');
    expect(client.SSEUrl()).toContain('/events');
  });

  test('CEP85Client constructs', () => {
    const client = new CEP85Client('http://127.0.0.1:11101');
    expect(client.rpcUrl()).toBe('http://127.0.0.1:11101/rpc');
  });

  test('setContractHash accepts hex', () => {
    const client = new CEP18Client('http://127.0.0.1:11101');
    const hex = 'b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f';
    expect(() => client.setContractHash(hex, hex)).not.toThrow();
  });
});
