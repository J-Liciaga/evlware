"use client";

import Sidebar from "@evlgorgon/core/components/navigation/sidebar";
import { WelcomeChatDashboard } from "@evlgorgon/screens/index";

export default function IndexPage() {
	return (
		<>
			<Sidebar />
			<div className="h-screen w-screen">
				<WelcomeChatDashboard />
			</div>
		</>
	);
}
