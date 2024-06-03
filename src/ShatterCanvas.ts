export type ShatterCanvasSettings = {
	context: CanvasRenderingContext2D,
	image?: HTMLImageElement | {
		source: CanvasImageSource,
		width: number,
		height: number
	}
}

export class ShatterCanvas {
	constructor({ context, image }: ShatterCanvasSettings) {
		if (image == undefined) {
			return;
		}

		if(image instanceof HTMLImageElement) {
			image.addEventListener("load", (event) => {
				const target = event.target as HTMLImageElement;
				context.drawImage(image, 0, 0, target.width, target.height);
			});
		} else {
				context.drawImage(image.source, 0, 0, image.width, image.height);
		}
	}
};
