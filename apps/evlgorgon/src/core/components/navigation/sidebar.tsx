"use client";

import React, { useState } from "react";
import {
	IconArrowLeft,
	IconBrandTabler,
	IconSettings,
	IconUserBolt,
	IconCategoryPlus,
} from "@tabler/icons-react";
import {
	Sidebar as LuckySidebar,
	SidebarBody,
	SidebarLink,
} from "@lucky-ui/animated/sidebar";
import { Logo, LogoIcon } from "@evlgorgon/core/components/common/evlware-logo";

const styles = {
	container:
		"fixed left-0 z-50 | h-screen max-w-7xl w-fit | flex flex-1 flex-col md:flex-row | bg-gray-100 dark:bg-neutral-800 | overflow-hidden",
} as const;

const DEFAULT_LINKS = [
	{
		label: "New Chat",
		href: "#",
		icon: (
			<IconBrandTabler className="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
		),
	},
	{
		label: "Projects",
		href: "#",
		icon: (
			<IconCategoryPlus className="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
		),
	},
];

export default function Sidebar() {
	const [open, setOpen] = useState(false);

	return (
		<div className={styles.container}>
			<LuckySidebar open={open} setOpen={setOpen}>
				<SidebarBody className="justify-between gap-10">
					<div className="flex flex-col flex-1 overflow-y-auto overflow-x-hidden">
						{open ? <Logo /> : <LogoIcon />}
						<div className="mt-8 flex flex-col gap-2">
							{DEFAULT_LINKS.map((link, idx) => (
								<SidebarLink key={idx} link={link} />
							))}
						</div>
					</div>
					{/* <div>
							<SidebarLink
							link={{
								label: "Manu Arora",
								href: "#",
								icon: (
								<Image
									src="https://assets.aceternity.com/manu.png"
									className="h-7 w-7 flex-shrink-0 rounded-full"
									width={50}
									height={50}
									alt="Avatar"
								/>
								),
							}}
							/>
						</div> */}
				</SidebarBody>
			</LuckySidebar>
		</div>
	);
}
