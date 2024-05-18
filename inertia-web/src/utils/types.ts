import { Dispatch, StateUpdater } from 'preact/hooks';

export type ValueOf<T> = T[keyof T];

export type StateSetter<T> = Dispatch<StateUpdater<T>>;

export type Nullable<T> = T | null | undefined;
