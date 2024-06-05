import { StepSettings } from "./render";

type PixelSortSettings = {
  mask: (image: { r: number; g: number; b: number }[]) => boolean[];
};

function getHue(r: number, g: number, b: number) {
	const min = Math.min(r, g, b);
	const max = Math.max(r, g, b);

	switch(max) {
		case r: {
			return (g-b)/(max-min);
		}
		case g: {
			return 2 + (b-r)/(max-min);
		}
		case b: {
			return 4 + (r-g)/(max-min);
		}
	}

	// If you get here. Run!
	throw Error("Max value was matched");
}

export function pixelSort({ mask }: PixelSortSettings) {
  return ({ image }: StepSettings) => {
    for (let i = 0; i < image.data.length; i += image.width * 4) {
      const slice = image.data.slice(i, i + image.width * 4);

      const slowSlice: { r: number; g: number; b: number }[] = [];

      for (let j = 0; j < slice.length; j += 4) {
        slowSlice.push({
          r: slice[j],
          g: slice[j + 1],
          b: slice[j + 2],
        });
      }

      const slowerSlice: number[] = [];

      const loadedMask = mask(slowSlice);

      let start = -1;
      let toBeSortedSpans: { start: number; end: number }[] = [];
      loadedMask.forEach((v, i) => {
        if (v && start == -1) {
          start = i;
        } else if (!v && start != -1) {
          toBeSortedSpans.push({ start: start, end: i });
          start = -1;
        }
      });
			if (start != -1) {
				toBeSortedSpans.push({ start: start, end: loadedMask.length });
				start = -1;
			}

      toBeSortedSpans.forEach((v) => {
        const sorted = slowSlice.slice(v.start, v.end).sort((a, b) => {
          const hueA = getHue(a.r, a.g, a.b);
          const hueB = getHue(b.r, b.g, b.b);

          if (hueA == hueB) return 0;
          if (hueA < hueB) return -1;
          return 1;
        });
        slowSlice.splice(v.start, sorted.length, ...sorted);
      });

      slowSlice.forEach((v) => {
        slowerSlice.push(v.r, v.g, v.b, 255);
      });

      image.data.set(slowerSlice, i);
    }

    return image;
  };
}
