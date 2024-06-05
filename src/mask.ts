import { StepSettings } from "./render";

function getLuminance(r: number, g: number, b: number) {
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

export type ContrastMaskSettings = {
  lowerThreshold: number;
  heigherThreshold: number;
};

/**
 *  NOTE: Both thresholds are between 0 and 1
 */
export function contrastMask({
  lowerThreshold,
  heigherThreshold,
}: ContrastMaskSettings) {
  return (image: { r: number; g: number; b: number }[]) => {
    return image.map((v) => {
      const luminance = getLuminance(v.r, v.g, v.b) / 256;

      return luminance >= lowerThreshold && luminance <= heigherThreshold;
    });
  };
}

export type RenderMaskSettings = {
	mask: (image: { r: number; g: number; b: number }[]) => boolean[];
}

export function renderMask({mask}: RenderMaskSettings) {
	return ({image}: StepSettings) => {
		let pixelated = []
		for (let j = 0; j < image.data.length; j += 4) {
			pixelated.push({
				r: image.data[j],
				g: image.data[j + 1],
				b: image.data[j + 2],
			});
		}

		const loadedMask = mask(pixelated);

		const out: number[] = [];
		const masked = loadedMask.map(v => v ? { r: 255, g: 0, b: 0 } : { r: 0, g: 0, b: 0 })
		masked.forEach((v) => {
			out.push(v.r, v.g, v.b, 255);
		});


		image.data.set(out);

		return image;
	}
}
