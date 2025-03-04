import { describe, test, expect } from 'vitest';
import { Verbosity, ClientCEP78 } from 'ceps-ts-client';

describe('ClientCEP78 Tests', () => {
  test('should create a client with valid URLs', () => {
    const client = new ClientCEP78(
      'http://valid-rpc-url',
      'http://valid-sse-url',
      Verbosity.Low
    );
    expect(client.getRPCUrl()).toBe('http://valid-rpc-url/rpc');
    expect(client.getSSEUrl()).toBe('http://valid-sse-url/events');
    expect(client.getVerbosity()).toBe(Verbosity.Low);
  });

  test('should create a client with valid URLs', () => {
    const client = new ClientCEP78('http://valid-rpc-url');

    expect(client.getRPCUrl()).toBe('http://valid-rpc-url/rpc');
    expect(client.getSSEUrl()).toBe('');
    expect(client.getVerbosity()).toBe(Verbosity.Low);
  });

  test('should return an error when created with empty URLs', () => {
    expect(() => new ClientCEP78('', undefined, Verbosity.Low)).toThrow();
  });

  test('should throw an error when created with invalid RPC URL', () => {
    expect(
      () =>
        new ClientCEP78(
          'invalid-rpc-url',
          'http://valid-sse-url',
          Verbosity.Low
        )
    ).toThrow('RPC URL could not be initialized due to invalid URL.');
  });

  test('should throw an error when created with invalid SSE URL', () => {
    expect(
      () =>
        new ClientCEP78(
          'http://valid-rpc-url',
          'invalid-sse-url',
          Verbosity.Low
        )
    ).toThrow('SSE URL could not be initialized due to invalid URL.');
  });

  test('should allow updating the RPC URL', () => {
    const client = new ClientCEP78(
      'http://valid-rpc-url',
      'http://valid-sse-url',
      Verbosity.Low
    );

    client.setRPCUrl('http://new-rpc-url');
    expect(client.getRPCUrl()).toBe('http://new-rpc-url/rpc');
  });

  test('should allow updating the SSE URL', () => {
    const client = new ClientCEP78(
      'http://valid-rpc-url',
      'http://valid-sse-url',
      Verbosity.Low
    );

    client.setSSEUrl('http://new-sse-url');
    expect(client.getSSEUrl()).toBe('http://new-sse-url/events');
  });

  test('should allow updating verbosity', () => {
    const client = new ClientCEP78(
      'http://valid-rpc-url',
      'http://valid-sse-url',
      Verbosity.Low
    );

    client.setVerbosity(Verbosity.High);
    expect(client.getVerbosity()).toBe(Verbosity.High);
  });
});
