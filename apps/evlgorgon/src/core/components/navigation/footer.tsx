"use client";

import { Button } from "@lucky-ui/basic/button";
import Link from "next/link";
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from "@lucky-ui/basic/select";

function LanguageSelect() {
	return (
		<Select>
			<SelectTrigger className="w-[11vw] bg-transparent border-none">
				<SelectValue placeholder="English (EN)" />
			</SelectTrigger>
			<SelectContent>
				<SelectGroup>
					<SelectItem value="english">English</SelectItem>
					<SelectItem value="spanish">Spanish</SelectItem>
					<SelectItem value="deutsch">Deutsch</SelectItem>
					<SelectItem value="french">French</SelectItem>
					<SelectItem value="japanese">Japanese</SelectItem>
				</SelectGroup>
			</SelectContent>
		</Select>
	);
}

const FOOTER_LINKS = [
	{
		id: "upgrade-footer-link",
		label: "Upgrade",
		link: "/",
	},
	{
		id: "blog-footer-link",
		label: "Blog",
		link: "/",
	},
	{
		id: "docs-footer-link",
		label: "Docs",
		link: "/",
	},
];

export default function Footer() {
	const render_links = () => {
		return FOOTER_LINKS.map(({ id, label, link }) => (
			<Button key={id} variant={"ghost"}>
				<Link href={link}>{label}</Link>
			</Button>
		));
	};

	return (
		<footer className="w-full pl-14">
			<div className="flex justify-center items-center text-neutral-200">
				{render_links()}
				<LanguageSelect />
			</div>
		</footer>
	);
}
