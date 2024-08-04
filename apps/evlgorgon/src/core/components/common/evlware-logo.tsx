"use client";

import Link from "next/link";
import { motion } from "framer-motion";
import Image from "next/image";
import { dark_evlware_logo, light_evlware_logo } from "@lucky-assets/icons";

const styles = {
	container:
		"font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20",
	icon_logo:
		"h-5 w-6 bg-black dark:bg-white rounded-br-lg rounded-tr-sm rounded-tl-lg rounded-bl-sm flex-shrink-0",
	text_logo: "font-medium text-black dark:text-white whitespace-pre",
} as const;

export function Logo() {
	return (
		<Link href="/" className={styles.container}>
			<div className={styles.icon_logo} />
			<motion.span
				initial={{ opacity: 0 }}
				animate={{ opacity: 1 }}
				className={styles.text_logo}
			>
				EVLWARE
			</motion.span>
		</Link>
	);
}

export function LogoIcon() {
	return (
		<Link
			href="#"
			className="h-10 w-16 font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20"
		>
			{/* <div className="h-5 w-6 bg-black dark:bg-white rounded-br-lg rounded-tr-sm rounded-tl-lg rounded-bl-sm flex-shrink-0" /> */}
			<Image
				src={dark_evlware_logo}
				alt="evlware-logo"
				height={45}
				width={45}
			/>
		</Link>
	);
}
