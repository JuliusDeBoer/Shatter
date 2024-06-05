import { StepSettings } from "./render";

export function loadImage(imageSource: CanvasImageSource) {
  return ({ context, image }: StepSettings) => {
    // TODO: Figure out a way to get image data without drawing it to a canvas
    context.drawImage(imageSource, 0, 0, image.width, image.height);

    return context.getImageData(0, 0, image.width, image.height, {
      colorSpace: "srgb",
    });
  };
}
