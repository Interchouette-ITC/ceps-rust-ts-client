import { describe, test, expect } from 'vitest';
import { Cep18Client, Cep78Client, Cep85Client } from 'ceps-wasm';

describe('ceps-wasm constructors', () => {
  test('Cep18Client normalizes RPC URL', () => {
    const client = new Cep18Client('http://127.0.0.1:11101', undefined, 'casper-net-1', 0);
    expect(client.rpcUrl()).toBe('http://127.0.0.1:11101/rpc');
    expect(client.chainName()).toBe('casper-net-1');
  });

  test('Cep18Client rejects empty RPC', () => {
    expect(() => new Cep18Client('', undefined, undefined, 0)).toThrow();
  });

  test('Cep78Client constructs', () => {
    const client = new Cep78Client('http://127.0.0.1:11101', 'http://127.0.0.1:18101/events');
    expect(client.rpcUrl()).toContain('/rpc');
    expect(client.sseUrl()).toContain('/events');
  });

  test('Cep85Client constructs', () => {
    const client = new Cep85Client('http://127.0.0.1:11101');
    expect(client.rpcUrl()).toBe('http://127.0.0.1:11101/rpc');
  });

  test('setContractHash accepts hex', () => {
    const client = new Cep18Client('http://127.0.0.1:11101');
    const hex = 'b485c074cef7ccaccd0302949d2043ab7133abdb14cfa87e8392945c0bd80a5f';
    expect(() => client.setContractHash(hex, hex)).not.toThrow();
  });
});
