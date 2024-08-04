import { LogoIcon } from "@evlgorgon/core/components/common/evlware-logo";
import {
	Card,
	CardHeader,
	CardTitle,
	CardDescription,
	CardContent,
	CardFooter,
} from "@lucky-ui/basic/card";
// import Input from "@lucky-ui/basic/input";
import TextArea from "@lucky-ui/basic/textarea";
import { Button } from "@lucky-ui/basic/button";
import { IconUpload, IconSend } from "@tabler/icons-react";

const styles = {
	container: "h-full w-full | flex flex-1",
	wrapper:
		"h-full w-full | flex flex-col flex-1 gap-2 | py-20 pl-20 pr-6 | dark:bg-neutral-900",
	card_container: "h-full w-full | flex justify-center items-center",
	card_wrapper: "",
	card_header: "",
	card_content: "",
	card_footer: "",
} as const;

export default function WelcomeChatDashboard() {
	return (
		<div className={styles.container}>
			<div className={styles.wrapper}>
				<div className={styles.card_container}>
					<Card className="w-[60vw] | dark:bg-neutral-800 border-none rounded-3xl ">
						<CardHeader className="flex justify-center items-center | text-white space-y-6">
							<CardTitle className="flex flex-col justify-center items-center | space-y-4 ">
								<div>EVLWARE</div>
								<div>Name your target?</div>
							</CardTitle>
							<CardDescription>
								Make sure you have permissions to attack
							</CardDescription>
						</CardHeader>
						<CardContent className="w-full">
							<div className="flex flex-col justify-center items-center | w-full | rounded-md | space-x-4 | p-2 bg-white">
								<TextArea
									placeholder="Type your prompt here..."
									className="border-none"
								/>
								<div className="w-full | flex justify-end items-end mt-2 pt-4 px-2 border-t-[0.025rem]">
									<Button variant={"ghost"}>
										<IconUpload />
									</Button>
									<Button className="bg-green-500 text-white">
										<IconSend />
									</Button>
								</div>
							</div>
						</CardContent>
						<CardFooter></CardFooter>
					</Card>
				</div>
			</div>
		</div>
	);
}
