function getLuminance(r: number, g: number ,b: number) {
	return 0.2126*r + 0.7152*g + 0.0722*b;
}

export function pixelSort() {
  return (image: ImageData) => {
    for (let i = 0; i < image.data.length; i += (image.width * 4)) {
      const slice = image.data.slice(i, i + (image.width * 4));

      const slowSlice: { r: number; g: number; b: number }[] = [];

      for (let j = 0; j < slice.length; j += 4) {
        slowSlice.push({
          r: slice[j],
          g: slice[j + 1],
          b: slice[j + 2],
        });
      }

      const slowerSlice: number[] = [];

      const luminanceMap = slowSlice.map(v => {
				const luminance = getLuminance(v.r, v.g, v.b);

				if(luminance < 60 || luminance > 170) {
					return false
				}

				return true
			});

			let start = -1;
			let toBeSortedSpans: { start: number, end: number }[] = [];
			luminanceMap.forEach((v, i) => {
				if(v && start == -1) {
					start = i;
				} else if (!v && start != -1) {
					toBeSortedSpans.push({ start: start, end: i });
					start = -1;
				}
			});

			toBeSortedSpans.forEach(v => {
				const sorted = slowSlice.slice(v.start, v.end).sort((a, b) => {
					const luminanceA = getLuminance(a.r, a.g, a.b);
					const luminanceB = getLuminance(b.r, b.g, b.b);

					if (luminanceA == luminanceB) return 0;
					if (luminanceA < luminanceB) return -1;
					return 1;
				});
				slowSlice.splice(v.start, sorted.length, ...sorted);
			})


			slowSlice.forEach((v) => {
        slowerSlice.push(v.r, v.g, v.b, 255);
      });

      image.data.set(slowerSlice, i);
    }

    return image;
  };
}
