const pad = (num: number, length: number) =>
  num.toString().padStart(length, '0');

export const mmssccFormat = (millis: number) => {
  const centisTotal = Math.floor(millis / 10);
  const secondsTotal = Math.floor(millis / 1000);
  const minutesTotal = Math.floor(secondsTotal / 60);

  const minutes = pad(minutesTotal % 60, 1);
  const seconds = pad(secondsTotal % 60, 2);
  const centis = pad(centisTotal % 100, 2);
  return `${minutes}:${seconds}.${centis}`;
};
