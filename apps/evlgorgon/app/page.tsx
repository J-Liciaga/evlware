"use client";

import Footer from "@evlgorgon/core/components/navigation/footer";
import { Help } from "@evlgorgon/core/components/navigation/help";
import Sidebar from "@evlgorgon/core/components/navigation/sidebar";
import { WelcomeChatDashboard } from "@evlgorgon/screens/index";

export default function IndexPage() {
	return (
		<>
			<Sidebar />
			<div className="h-screen w-screen | flex flex-col justify-between | py-4 | dark:bg-neutral-900">
				<WelcomeChatDashboard />
				<Footer />
			</div>
			<Help />
		</>
	);
}
