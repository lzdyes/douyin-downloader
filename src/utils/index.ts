export const sleep = (delay?: number) => new Promise<void>((resole) => setTimeout(resole, delay))
