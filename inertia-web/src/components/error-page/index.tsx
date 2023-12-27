import { ComponentChildren } from 'preact';
import { FlexCenter } from '../flex-center';
import { ThemedPanel } from '../themed-panel';
import { FullScreen } from '../full-screen';

export const ErrorPage = ({ children }: { children: ComponentChildren }) => {
  return (
    <FullScreen>
      <FlexCenter expand>
        <ThemedPanel>{children}</ThemedPanel>
      </FlexCenter>
    </FullScreen>
  );
};
