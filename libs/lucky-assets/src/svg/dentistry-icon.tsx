export default function DentistryIcon({
	width = 64,
	height = 64,
	color = "#000",
}) {
	return (
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 64 64"
			width={width}
			height={height}
		>
			<path
				d="M32 2C24.82 2 19 7.82 19 15v12c0 10.73 9.27 20 20 20s20-9.27 20-20V15c0-7.18-5.82-13-13-13S32 7.82 32 15V2zM25 24v-9c0-3.86 3.14-7 7-7s7 3.14 7 7v9h-6v12c0 4.42-3.58 8-8 8s-8-3.58-8-8V24h6z"
				fill="none"
				stroke={color}
				strokeWidth="2"
			/>
		</svg>
	);
}
