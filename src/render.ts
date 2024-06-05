export type StepSettings = {
  image: ImageData;
  context: CanvasRenderingContext2D;
};

export type ShatterSettings = {
  context: CanvasRenderingContext2D;
  width: number;
  height: number;
  steps?: ((settings: StepSettings) => ImageData | void)[];
};

export function render({
  context,
  steps = [],
  width,
  height,
}: ShatterSettings) {
  context.getContextAttributes().willReadFrequently = true;

  let image = new ImageData(width, height);

  steps.forEach((step) => {
    const result = step({ image: image, context: context });
    if (result != undefined) {
      image = result;
    }
  });

  context.putImageData(image, 0, 0);
}
