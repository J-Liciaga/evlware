"use client";

import * as React from "react";
import { format } from "date-fns";
import { Calendar as CalendarIcon } from "lucide-react";

import { cn } from "../utils";
import { Button } from "./button";
import Calendar from "./calendar";
import { Popover, PopoverContent, PopoverTrigger } from "./popover";

export interface DatePickerProps {
	placeholder?: string;
	className?: string;
	field?: any;
}

export function DatePicker({ placeholder, className, field }: DatePickerProps) {
	const [date, setDate] = React.useState<Date>();

	const selectHandler = (date: Date | undefined) => {
		if (field) {
			field.value = date?.toISOString();
			field.onChange(date?.toISOString());
		}
		setDate(date);
	};

	return (
		<Popover>
			<PopoverTrigger asChild>
				<Button
					variant={"outline"}
					className={cn(
						"w-full justify-start text-left font-normal",
						!date && "text-muted-foreground",
						className,
					)}
				>
					<CalendarIcon className="mr-2 h-4 w-4" />
					{date ? format(date, "PPP") : <span>{placeholder}</span>}
				</Button>
			</PopoverTrigger>
			<PopoverContent className="w-auto p-0">
				<Calendar
					mode="single"
					selected={date}
					onSelect={e => selectHandler(e)}
					initialFocus
				/>
			</PopoverContent>
		</Popover>
	);
}
