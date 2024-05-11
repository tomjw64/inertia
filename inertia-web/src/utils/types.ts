import { Dispatch, StateUpdater } from 'preact/hooks';

export type StateSetter<T> = Dispatch<StateUpdater<T>>;

export type Nullable<T> = T | null | undefined;
