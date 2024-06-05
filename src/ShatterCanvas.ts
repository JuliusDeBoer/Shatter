export type ShatterCanvasSettings = {
  context: CanvasRenderingContext2D;
  image?:
    | HTMLImageElement
    | {
        source: CanvasImageSource;
        width: number;
        height: number;
      };
  steps?: ((image: ImageData) => ImageData)[];
};

export class ShatterCanvas {
  constructor({ context, image, steps }: ShatterCanvasSettings) {
    context.getContextAttributes().willReadFrequently = true;

    if (image == undefined) {
      return;
    }

    if (image instanceof HTMLImageElement) {
      image.addEventListener("load", (event) => {
        const target = event.target as HTMLImageElement;

        context.drawImage(image, 0, 0, target.width, target.height);

        let data = context.getImageData(0, 0, target.width, target.height, {
          colorSpace: "srgb",
        });

        (steps ?? []).forEach((step) => {
          data = step(data);
        });

        context.putImageData(data, 0, 0);
      });
    } else {
      context.drawImage(image.source, 0, 0, image.width, image.height);

      let data = context.getImageData(0, 0, image.width, image.height, {
        colorSpace: "srgb",
      });

      (steps ?? []).forEach((step) => {
        data = step(data);
      });

      context.putImageData(data, 0, 0);
    }
  }
}
