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
      const luminance = getLuminance(v.r, v.g, v.b) / 255;

      return luminance > lowerThreshold && heigherThreshold < 170;
    });
  };
}
