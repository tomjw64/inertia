import { ComponentChildren } from 'preact';

export const RenderWhen = ({
  when,
  children,
}: {
  children?: ComponentChildren;
  when: boolean;
}) => {
  return <>{when && children}</>;
};
