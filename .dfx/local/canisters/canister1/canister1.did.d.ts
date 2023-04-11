import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_message' : ActorMethod<[], string>,
  'set_message' : ActorMethod<[string], null>,
}
