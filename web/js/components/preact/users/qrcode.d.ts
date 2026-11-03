/** The qrcode package ships no types and @types/qrcode is not installed. Covers TotpSetupModal's one call. */
declare module 'qrcode' {
  interface QRCodeToCanvasOptions {
    width?: number;
    margin?: number;
    color?: { dark?: string; light?: string };
  }

  const QRCode: {
    toCanvas(canvas: HTMLCanvasElement, text: string, options?: QRCodeToCanvasOptions): Promise<unknown>;
  };

  export default QRCode;
}
