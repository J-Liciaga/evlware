import "./global.css";

export const metadata = {
	title: "EVLWARE",
	description:
		"AI-powered penetration testing framework. Automate and enhance your cybersecurity assessments using the latest machine learning technologies and advanced security tools.",
};

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<html lang="en">
			<body>{children}</body>
		</html>
	);
}
