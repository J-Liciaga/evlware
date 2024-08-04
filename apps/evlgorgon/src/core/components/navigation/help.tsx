import * as React from "react";

import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@lucky-ui/basic/select";

export function Help() {
	return (
		<div className="fixed bottom-4 right-4">
			<Select>
				<SelectTrigger
					indicator={false}
					className="w-fit p-4 bg-white rounded-full"
				>
					<SelectValue placeholder="?" />
				</SelectTrigger>
				<SelectContent>
					<SelectGroup>
						<SelectItem value="apple">Get Started</SelectItem>
						<SelectItem value="banana">Help & FAQ</SelectItem>
						<SelectItem value="blueberry">
							Keyboard Shortcuts
						</SelectItem>
						<SelectItem value="grapes">EVLWARE Pro</SelectItem>
						<SelectItem value="pineapple">
							Terms of Service
						</SelectItem>
						<SelectItem value="pineapple">
							Privacy Policy
						</SelectItem>
					</SelectGroup>
				</SelectContent>
			</Select>
		</div>
	);
}
