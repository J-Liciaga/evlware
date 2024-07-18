"use client";

import { type ReactElement, type ReactNode } from "react";
import * as RadixPortal from "@radix-ui/react-portal";

export interface PortalProps {
	show: any;
	children: ReactElement | ReactNode;
}

export default function Portal({ show, children }: PortalProps) {
	return show ? <RadixPortal.Root>{children}</RadixPortal.Root> : null;
}
