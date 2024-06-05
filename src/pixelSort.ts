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

      slowSlice.sort((a, b) =>
        Math.max(a.r, a.g, a.g) < Math.max(b.r, b.g, b.g) ? -1 : +1,
      );

      const slowerSlice: number[] = [];

      slowSlice.forEach((v) => {
        slowerSlice.push(v.r, v.g, v.b, 255);
      });

      image.data.set(slowerSlice, i);
    }

    return image;
  };
}
