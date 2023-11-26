export const isMobile = (): boolean => {
  return /Android|webOS|iPhone|iPad/i.test(navigator.userAgent);
};
