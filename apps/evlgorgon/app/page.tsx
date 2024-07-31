"use client";

import {
	PublicFooter,
	PublicHeader,
} from "@evlgorgon/core/components/navigation/public";
import {
	HeroSection,
	FeaturesSection,
	ServicesSection,
	RoadmapSection,
} from "@evlgorgon/screens/index";

const styles = {
	container: "min-h-screen min-w-screen",
} as const;

export default function IndexPage() {
	return (
		<>
			<PublicHeader />
			<div className={styles.container}>
				<HeroSection />
				<FeaturesSection />
				<ServicesSection />
				<RoadmapSection />
			</div>
			<PublicFooter />
		</>
	);
}
