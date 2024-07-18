export interface TypographyProps {
	content: string;
}

const H1 = ({ content }: TypographyProps) => (
	<h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
		{content}
	</h1>
);

const H2 = ({ content }: TypographyProps) => (
	<h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
		{content}
	</h2>
);

const H3 = ({ content }: TypographyProps) => (
	<h3 className="scroll-m-20 text-2xl font-semibold tracking-tight">
		{content}
	</h3>
);

const H4 = ({ content }: TypographyProps) => (
	<h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
		{content}
	</h4>
);

const P = ({ content }: TypographyProps) => (
	<p className="leading-7 [&:not(:first-child)]:mt-6">{content}</p>
);

const Blockquote = ({ content }: TypographyProps) => (
	<blockquote className="mt-6 border-l-2 pl-6 italic">{content}</blockquote>
);

const InlineCode = ({ content }: TypographyProps) => (
	<code className="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold">
		{content}
	</code>
);

const Lead = ({ content }: TypographyProps) => (
	<p className="text-xl text-muted-foreground">{content}</p>
);

const LargeText = ({ content }: TypographyProps) => (
	<div className="text-lg font-semibold">{content}</div>
);

const SmallText = ({ content }: TypographyProps) => (
	<small className="text-sm font-medium leading-none">{content}</small>
);

const Muted = ({ content }: TypographyProps) => (
	<p className="text-sm text-muted-foreground">{content}</p>
);

export {
	H1,
	H2,
	H3,
	H4,
	P,
	Blockquote,
	InlineCode,
	Lead,
	LargeText,
	SmallText,
	Muted,
};
